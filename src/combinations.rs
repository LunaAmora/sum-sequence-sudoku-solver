use super::Pair;
use super::Sum;

pub fn compute_solutions(sequence: &[usize; 12], pair_sums: &[Sum<Pair>]) -> Vec<[Pair; 12]> {
    // Try all possible combinations of selecting one pair from each sum in the sequence
    let pair_options: [&Vec<Pair>; 12] =
        sequence.iter().map(|&sum_value| &pair_sums[sum_value - 3].0).collect::<Vec<&Vec<Pair>>>().try_into().unwrap();

    // Use backtracking to find all valid combinations
    let mut all_solutions = Vec::new();
    let mut selected_pairs = Vec::new();
    find_all_valid_combinations(&pair_options, 0, &mut selected_pairs, &mut all_solutions);
    all_solutions
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
    for &(a, b) in pairs {
        digit_count[a - 1] += 1;
        digit_count[b - 1] += 1;
    }

    // Count how many digits appear exactly 4 times and exactly 2 times
    let mut count_4 = 0;
    let mut count_2 = 0;

    for &count in &digit_count[0..9] {
        match count {
            4 => count_4 += 1,
            2 => count_2 += 1,
            0 => return false,
            _ => return false,
        }
    }

    // We need exactly 3 digits appearing 4 times, 6 digits appearing 2 times
    count_4 == 3 && count_2 == 6
}
