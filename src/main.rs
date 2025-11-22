#![allow(dead_code)]

mod checker;
mod combinations;
mod sums;

use checker::fill_constraints;
use combinations::compute_solutions;
use indexmap::IndexMap;
use sums::*;

fn main() {
    let pair_sums = pair_sums();
    let triplet_sums = triplet_sums();

    let mut triplet_map = get_triplet_map(&triplet_sums);
    let pairs_sequence = get_pairs_sequence(&triplet_map);

    filter_triplets(&mut triplet_map, &pairs_sequence, &pair_sums);

    let pairs_sequence = get_pairs_sequence(&triplet_map);

    // print_triple_map(&triplet_map);
    // print_pairs_sequence(&pairs_sequence);
    // print_solutions(&pairs_sequence, &pair_sums);
    print_checked_sets(&pairs_sequence, &pair_sums);
}

type CornerTriplets = [Triplet; 3];

fn get_triplet_map(triplet_sums: &[Vec<Triplet>]) -> IndexMap<Triplet, Vec<CornerTriplets>> {
    let mut res: IndexMap<_, Vec<CornerTriplets>> = IndexMap::new();

    for i in 0..triplet_sums.len() - 1 {
        for j in (i + 1)..triplet_sums.len() - 1 {
            let k = 12;
            let triplets = find_unique_triplets(&triplet_sums[i], &triplet_sums[j], &triplet_sums[k]);
            for [a, b, c] in triplets {
                let e = [i as u16 + 6, j as u16 + 6, k as u16 + 6];
                res.entry(e).or_default().push([a, b, c]);
            }
        }
    }
    res
}

fn find_unique_triplets(s1: &[Triplet], s2: &[Triplet], s3: &[Triplet]) -> Vec<CornerTriplets> {
    let mut res = vec![];
    for t1 in s1 {
        for t2 in s2 {
            for t3 in s3 {
                let mut digits = vec![t1[0], t1[1], t1[2], t2[0], t2[1], t2[2], t3[0], t3[1], t3[2]];
                digits.sort_unstable();
                digits.dedup();
                if digits.len() == 9 {
                    res.push([*t1, *t2, *t3]);
                }
            }
        }
    }
    res
}

fn print_triple_map(triplet_map: &IndexMap<Triplet, Vec<CornerTriplets>>) {
    for (sum, triplets) in triplet_map {
        println!("{:?}", sum);
        for [[a1, a2, a3], [b1, b2, b3], [c1, c2, c3]] in triplets {
            println!(" {}{}{} {}{}{} {}{}{}", a1, a2, a3, b1, b2, b3, c1, c2, c3);
        }
        println!();
    }
}

fn get_pairs_sequence(triplet_map: &IndexMap<Triplet, Vec<CornerTriplets>>) -> IndexMap<Triplet, [u16; 12]> {
    let mut pairs_sequence: IndexMap<Triplet, [_; 12]> = IndexMap::new();

    for k @ [a, b, c] in triplet_map.keys().copied() {
        let mut sequence = [0; 12];
        let mut i = 11;
        let mut n = 17;

        loop {
            if n == c || n == b || n == a {
                n -= 1;
                continue;
            }

            sequence[i] = n;

            if i == 0 {
                break;
            }

            i -= 1;
            n -= 1;
        }

        pairs_sequence.insert(k, sequence);
    }

    pairs_sequence
}

fn print_pairs_sequence(pairs_sequence: &IndexMap<Triplet, [u16; 12]>) {
    for (key, sequence) in pairs_sequence {
        let key = format!("{:?}", key);
        println!("{:12} = {:2?}", key, sequence);
    }
}

fn filter_triplets(
    triplet_map: &mut IndexMap<Triplet, Vec<CornerTriplets>>,
    pairs_sequence: &IndexMap<Triplet, [u16; 12]>,
    pair_sums: &[Vec<Pair>],
) {
    for (tripplet, seq) in pairs_sequence {
        let solutions = compute_solutions(seq, pair_sums);
        if solutions.is_empty() {
            triplet_map.shift_remove(tripplet);
        }
    }
}

fn print_checked_sets(pairs_sequence: &IndexMap<Triplet, [u16; 12]>, pair_sums: &[Vec<Pair>]) {
    for (tripplet, seq) in pairs_sequence {
        let solutions = compute_solutions(seq, pair_sums);
        if !solutions.is_empty() {
            for solution in solutions {
                let mut digit_count = [0; 9];
                for [a, b] in solution.0 {
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
                let mut set = solution.0.to_vec();
                set.extend_from_slice(&solution.1);

                fill_constraints(&set, dups, &mut results);

                for result in results {
                    print!("{:?} - {:?}:", tripplet, dups,);
                    for [a, b] in result {
                        print!(" {}{}", a, b);
                    }
                    println!();
                }
            }
        }
    }
}

fn print_solutions(pairs_sequence: &IndexMap<Triplet, [u16; 12]>, pair_sums: &[Vec<Pair>]) {
    for (tripplet, seq) in pairs_sequence {
        let solutions = compute_solutions(seq, pair_sums);
        if !solutions.is_empty() {
            println!(
                "Found {} valid combination(s) for triplet {:?} with sum sequence {:?}",
                solutions.len(),
                tripplet,
                seq
            );
            for solution in &solutions {
                let mut digit_count = [0; 9];
                for [a, b] in solution.0 {
                    digit_count[a as usize - 1] += 1;
                    digit_count[b as usize - 1] += 1;
                }
                let dups: Vec<usize> = (0..9).filter(|&digit| digit_count[digit] == 2).map(|i| i + 1).collect();

                print!("  {:?}:", dups);
                for [a, b] in solution.0 {
                    print!(" {}{}", a, b);
                }
                print!(" |");
                for [a, b] in solution.1 {
                    print!(" {}{}", a, b);
                }
                println!();
            }
        }
    }
}
