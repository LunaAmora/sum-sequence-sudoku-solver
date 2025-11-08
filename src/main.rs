#![allow(dead_code)]

mod combinations;
mod sums;

use combinations::compute_solutions;
use indexmap::IndexMap;
use sums::*;

fn main() {
    let pair_sums = pair_sums();
    let triplet_sums = triplet_sums();

    let mut triplet_map = get_triplet_map(triplet_sums);
    let pairs_sequence = get_pairs_sequence(&triplet_map);

    filter_triplets(&mut triplet_map, &pairs_sequence, &pair_sums);

    let pairs_sequence = get_pairs_sequence(&triplet_map);

    // print_triple_map(&triplet_map);
    // print_pairs_sequence(&pairs_sequence);
    print_solutions(&pairs_sequence, &pair_sums);
}

type CornerTriplets = [Triplet; 3];

fn get_triplet_map(triplet_sums: Vec<Sum<Triplet>>) -> IndexMap<Triplet, Vec<CornerTriplets>> {
    let mut res: IndexMap<_, Vec<CornerTriplets>> = IndexMap::new();

    for i in 0..triplet_sums.len() {
        for j in (i + 1)..triplet_sums.len() {
            for k in (j + 1)..triplet_sums.len() {
                if k == 14 && (j != 13 || i != 12) {
                    continue;
                }
                if k == 13 && j != 12 {
                    continue;
                }

                let triplets = find_unique_triplets(&triplet_sums[i], &triplet_sums[j], &triplet_sums[k]);
                for [a, b, c] in triplets {
                    let e = [i + 6, j + 6, k + 6];
                    res.entry(e).or_default().push([a, b, c]);
                }
            }
        }
    }
    res
}

fn find_unique_triplets(s1: &Sum<Triplet>, s2: &Sum<Triplet>, s3: &Sum<Triplet>) -> Vec<CornerTriplets> {
    let mut res = vec![];
    for t1 in &s1.0 {
        for t2 in &s2.0 {
            for t3 in &s3.0 {
                let mut digits = vec![t1[0], t1[1], t1[2], t2[0], t2[1], t2[2], t3[0], t3[1], t3[2]];
                digits.sort();
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

fn get_pairs_sequence(triplet_map: &IndexMap<Triplet, Vec<CornerTriplets>>) -> IndexMap<Triplet, [usize; 12]> {
    let mut pairs_sequence: IndexMap<Triplet, [_; 12]> = IndexMap::new();

    for k @ [a, b, c] in triplet_map.keys().cloned() {
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

fn print_pairs_sequence(pairs_sequence: &IndexMap<Triplet, [usize; 12]>) {
    for (key, sequence) in pairs_sequence {
        let key = format!("{:?}", key);
        println!("{:12} = {:2?}", key, sequence);
    }
}

fn filter_triplets(
    triplet_map: &mut IndexMap<Triplet, Vec<CornerTriplets>>,
    pairs_sequence: &IndexMap<Triplet, [usize; 12]>,
    pair_sums: &[Sum<Pair>],
) {
    for (tripplet, seq) in pairs_sequence {
        let solutions = compute_solutions(seq, pair_sums);
        if solutions.is_empty() {
            triplet_map.shift_remove(tripplet);
        }
    }
}

fn print_solutions(pairs_sequence: &IndexMap<Triplet, [usize; 12]>, pair_sums: &[Sum<Pair>]) {
    for (tripplet, seq) in pairs_sequence {
        let solutions = compute_solutions(seq, pair_sums);
        if !solutions.is_empty() {
            println!(
                "✓ Found {} valid combination(s) for triplet {:?} with sum sequence {:?}",
                solutions.len(),
                tripplet,
                seq
            );
            for (i, solution) in solutions.iter().enumerate() {
                let mut digit_count = [0; 9];
                for [a, b] in solution.0 {
                    digit_count[a - 1] += 1;
                    digit_count[b - 1] += 1;
                }
                let duplicated: Vec<usize> = (0..9).filter(|&digit| digit_count[digit] == 2).map(|i| i + 1).collect();
                println!(
                    "  Solution {:3} {:?}: {} | {}",
                    i + 1,
                    duplicated,
                    Sum(solution.0.to_vec()),
                    Sum(solution.1.to_vec())
                );
            }
        }
    }
}
