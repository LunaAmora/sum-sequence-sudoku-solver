use super::Pair;
use super::Sum;

pub fn compute_solutions(sequence: &[usize; 12], pair_sums: &[Sum<Pair>]) -> Vec<([Pair; 6], [Pair; 6])> {
    // Try all possible combinations of selecting one pair from each sum in the sequence
    let pair_options: [&Vec<Pair>; 12] =
        sequence.iter().map(|&sum_value| &pair_sums[sum_value - 4].0).collect::<Vec<&Vec<Pair>>>().try_into().unwrap();

    // Use backtracking to find all valid combinations
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
        // Check if this combination satisfies the constraint
        if check_frequency_constraint(selected_pairs) {
            let value = selected_pairs.to_vec().try_into().unwrap();
            all_solutions.push(value);
        }
        return;
    }

    // Try each pair option for the current position
    for &pair in pair_options[index] {
        selected_pairs.push(pair);
        find_all_valid_combinations(pair_options, index + 1, selected_pairs, all_solutions);
        selected_pairs.pop();
    }
}

fn check_frequency_constraint(pairs: &[Pair]) -> bool {
    let mut digit_count = [0; 9];

    // Count occurrences of each digit
    for &[a, b] in pairs {
        digit_count[a - 1] += 1;
        digit_count[b - 1] += 1;
    }

    let mut dup_sum = 0;
    let mut count_2 = 0;

    for (i, count) in digit_count.iter().enumerate() {
        match count {
            4 => dup_sum += i + 1,
            2 => count_2 += 1,
            0 => return false,
            _ => return false,
        }
    }

    // We need exactly 3 digits appearing 4 times, 6 digits appearing 2 times
    count_2 == 6 && dup_sum == 15
}

/// Splits a Vec<Pair> into 2 groups of 6 pairs each, maintaining frequency constraints.
/// Returns all valid ways to split the pairs.
/// Each group should have: 3 digits appearing 2 times, 6 digits appearing once.
pub fn split_pairs_evenly(pairs: [Pair; 12]) -> Vec<([Pair; 6], [Pair; 6])> {
    let mut results = Vec::new();

    // Generate all combinations of 6 items from 12 using bit manipulation
    // We need combinations of 6 out of 12, which is C(12,6) = 924 combinations
    // To avoid duplicates, we only consider masks where the first bit is set
    // This ensures we only generate each unique split once
    for mask in 0u32..(1u32 << 12) {
        if mask.count_ones() == 6 && (mask & 1) == 1 {
            let group1: [Pair; 6] = (0..12)
                .filter(|&i| (mask & (1 << i)) != 0)
                .map(|i| pairs[i])
                .collect::<Vec<Pair>>()
                .try_into()
                .unwrap();

            let group2: [Pair; 6] = (0..12)
                .filter(|&i| (mask & (1 << i)) == 0)
                .map(|i| pairs[i])
                .collect::<Vec<Pair>>()
                .try_into()
                .unwrap();

            // Check if both groups satisfy the constraint
            if check_group_constraint(&group1) && check_group_constraint(&group2) {
                results.push((group1, group2));
            }
        }
    }

    results
}

/// Checks if a group of 6 pairs satisfies the constraint:
/// 3 digits appearing 2 times, 6 digits appearing once
fn check_group_constraint(pairs: &[Pair; 6]) -> bool {
    let mut digit_count = [0; 9];

    // Count occurrences of each digit
    for &[a, b] in pairs {
        digit_count[a - 1] += 1;
        digit_count[b - 1] += 1;
    }

    // Count how many digits appear exactly 2 times and exactly 1 time
    let mut count_2 = 0;
    let mut count_1 = 0;

    for &count in &digit_count[0..9] {
        match count {
            2 => count_2 += 1,
            1 => count_1 += 1,
            0 => return false,
            _ => return false,
        }
    }

    // We need exactly 3 digits appearing 2 times, 6 digits appearing 1 time
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

        println!("Found {} valid splits:", splits.len());
        for (i, (group1, group2)) in splits.iter().enumerate() {
            println!("Split {}:", i + 1);
            println!("  Group 1: {:?}", group1);
            println!("  Group 2: {:?}", group2);
        }
    }
}
