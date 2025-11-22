#![allow(dead_code)]

mod checker;
mod combinations;
mod sums;

use checker::fill_constraints;
use combinations::compute_combinations;
use indexmap::IndexMap;
use sums::*;

fn main() {
    let pair_sums = pair_sums();
    let triplet_sums = triplet_sums();

    let mut triplet_map = get_triplet_map(&triplet_sums);
    let mut pairs_sequence = get_pairs_sequence(&triplet_map);

    let combinations = get_combinations(&mut triplet_map, &mut pairs_sequence, &pair_sums);
    let solutions = get_solutions(&combinations);

    // print_triple_map(&triplet_map);
    // print_pairs_sequence(&pairs_sequence);
    // print_combinations(&combinations);
    print_solutions(&solutions);
}

fn get_combinations(
    triplet_map: &mut IndexMap<Triplet, Vec<CornerTriplets>>,
    pairs_sequence: &mut IndexMap<Triplet, [u16; 12]>,
    pair_sums: &[Vec<Pair>],
) -> IndexMap<Triplet, Vec<[Pair; 12]>> {
    let mut results = IndexMap::new();
    let mut to_remove = vec![];

    for (triplet, seq) in &*pairs_sequence {
        let combinations = compute_combinations(seq, pair_sums);
        if combinations.is_empty() {
            to_remove.push(*triplet);
        } else {
            results.insert(*triplet, combinations);
        }
    }

    for triplet in to_remove {
        triplet_map.shift_remove(&triplet);
        pairs_sequence.shift_remove(&triplet);
    }

    results
}

fn print_combinations(combinations: &IndexMap<Triplet, Vec<[Pair; 12]>>) {
    for (triplet, combination) in combinations {
        if !combination.is_empty() {
            println!("Found {} valid combination(s) for triplet {:?}", combination.len(), triplet);

            for sequence in combination {
                let mut digit_count = [0; 9];
                for &[a, b] in &sequence[0..6] {
                    digit_count[a as usize - 1] += 1;
                    digit_count[b as usize - 1] += 1;
                }
                let dups: Vec<usize> = (0..9).filter(|&digit| digit_count[digit] == 2).map(|i| i + 1).collect();

                print!("  {:?} =", dups);
                for [a, b] in &sequence[0..6] {
                    print!(" {}{}", a, b);
                }
                print!(" |");
                for [a, b] in &sequence[6..12] {
                    print!(" {}{}", a, b);
                }
                println!();
            }
        }
    }
}

fn get_solutions(combinations: &IndexMap<Triplet, Vec<[Pair; 12]>>) -> IndexMap<Triplet, Vec<[Pair; 12]>> {
    let mut solutions: IndexMap<_, Vec<_>> = IndexMap::new();

    for (triplet, combination) in combinations {
        for sequence in combination {
            let mut digit_count = [0; 9];
            for &[a, b] in &sequence[0..6] {
                digit_count[a as usize - 1] += 1;
                digit_count[b as usize - 1] += 1;
            }

            let dups = (0..9)
                .filter(|&digit| digit_count[digit as usize] == 2)
                .map(|i| i + 1)
                .collect::<Vec<u16>>()
                .try_into()
                .unwrap();

            let mut results = vec![];
            fill_constraints(sequence, dups, &mut results);

            solutions.entry(*triplet).or_default().append(&mut results);
        }
    }

    solutions
}

fn print_solutions(solutions: &IndexMap<Triplet, Vec<[Pair; 12]>>) {
    for (triplet, solution) in solutions {
        if !solution.is_empty() {
            println!("Found {} valid solution(s) for triplet {:?}", solution.len(), triplet);

            for sequence in solution {
                let mut digit_count = [0; 9];
                for &[a, b] in &sequence[0..6] {
                    digit_count[a as usize - 1] += 1;
                    digit_count[b as usize - 1] += 1;
                }
                let dups: Vec<usize> = (0..9).filter(|&digit| digit_count[digit] == 2).map(|i| i + 1).collect();

                print!("  {:?} =", dups);
                for [a, b] in &sequence[0..6] {
                    print!(" {}{}", a, b);
                }
                print!(" |");
                for [a, b] in &sequence[6..12] {
                    print!(" {}{}", a, b);
                }
                println!();
            }
        }
    }
}
