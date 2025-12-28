use super::Pair;

pub fn compute_combinations(sequence: &[u16; 12], pair_sums: &[Vec<Pair>]) -> Vec<[Pair; 12]> {
    let pair_options: [&Vec<Pair>; 12] = sequence
        .iter()
        .map(|&sum_value| &pair_sums[sum_value as usize - 4])
        .collect::<Vec<&Vec<Pair>>>()
        .try_into()
        .unwrap();

    let mut all_solutions = Vec::new();
    let mut selected_pairs = Vec::new();
    find_all_valid_combinations(&pair_options, 0, &mut selected_pairs, &mut all_solutions);

    let mut results = Vec::new();

    for pairs in all_solutions {
        let splits = split_pairs_evenly(pairs);
        results.extend(splits);
    }

    results
}

fn find_all_valid_combinations(
    pair_options: &[&Vec<Pair>; 12],
    index: usize,
    selected_pairs: &mut Vec<Pair>,
    all_solutions: &mut Vec<[Pair; 12]>,
) {
    if index == pair_options.len() {
        if check_frequency_constraint(selected_pairs) {
            let value = selected_pairs.clone().try_into().unwrap();
            all_solutions.push(value);
        }
        return;
    }

    for &pair in pair_options[index] {
        selected_pairs.push(pair);
        find_all_valid_combinations(pair_options, index + 1, selected_pairs, all_solutions);
        selected_pairs.pop();
    }
}

fn check_frequency_constraint(pairs: &[Pair]) -> bool {
    let mut digit_count = [0; 9];

    for &[a, b] in pairs {
        digit_count[a as usize - 1] += 1;
        digit_count[b as usize - 1] += 1;
    }

    let mut dup_sum = 0;
    let mut count_2 = 0;

    for (i, count) in digit_count.iter().enumerate() {
        match count {
            4 => dup_sum += i + 1,
            2 => count_2 += 1,
            _ => return false,
        }
    }

    count_2 == 6 && dup_sum == 15
}

pub fn split_pairs_evenly(pairs: [Pair; 12]) -> Vec<[Pair; 12]> {
    let mut results = Vec::new();

    for mask in 0u32..(1u32 << 12) {
        if mask.count_ones() == 6 && (mask & 1) == 1 {
            let group1 = (0..12).filter(|&i| (mask & (1 << i)) != 0).map(|i| pairs[i]);
            let group2 = (0..12).filter(|&i| (mask & (1 << i)) == 0).map(|i| pairs[i]);

            let res: [Pair; 12] = group1.chain(group2).collect::<Vec<Pair>>().try_into().unwrap();

            if check_group_constraint(res[0..6].try_into().unwrap())
                && check_group_constraint(res[6..12].try_into().unwrap())
            {
                results.push(res);
            }
        }
    }

    results
}

fn check_group_constraint(pairs: [Pair; 6]) -> bool {
    let mut digit_count = [0; 9];

    for [a, b] in pairs {
        digit_count[a as usize - 1] += 1;
        digit_count[b as usize - 1] += 1;
    }

    let mut count_2 = 0;
    let mut count_1 = 0;

    for &count in &digit_count[0..9] {
        match count {
            2 => count_2 += 1,
            1 => count_1 += 1,
            _ => return false,
        }
    }

    count_2 == 3 && count_1 == 6
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_pairs_evenly() {
        // 14 15 25 26 37 38 48 58 59 69 79 89
        let pairs = [[1, 4], [1, 5], [2, 5], [2, 6], [3, 7], [3, 8], [4, 8], [5, 8], [5, 9], [6, 9], [7, 9], [8, 9]];

        let splits = split_pairs_evenly(pairs);
        assert!(!splits.is_empty());
    }
}
