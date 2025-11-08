#![allow(dead_code)]

use indexmap::IndexMap;

struct Sum<T>(Vec<T>);

type Pair = (usize, usize);
type Triplet = (usize, usize, usize);

fn main() {
    let triplet_sums = triplet_sums();
    let triplet_map = get_triplet_map(triplet_sums);
    let pairs_sequence = get_pairs_sequence(&triplet_map);

    let pair_sums = pair_sums(); // to access sums from their value, use pair_sums[sum - 3]

    for seq in pairs_sequence.values() {
        // I have 16 items of length 12
        // each sequence corresponds to a triplet sum (from 6 to 17) of the digits 1-9
        // I want to check if its possibe to choose specific combinations of pairs from pair_sums
        // such that 3 digits appears exactly 4 times, and the rest appears exactly 2 times
    }
}

fn get_triplet_map(triplet_sums: Vec<Sum<Triplet>>) -> IndexMap<Triplet, Vec<(Triplet, Triplet, Triplet)>> {
    let mut res: IndexMap<_, Vec<(_, _, _)>> = IndexMap::new();

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
                for (a, b, c) in triplets {
                    let e = (i + 6, j + 6, k + 6);
                    res.entry(e).or_default().push((a, b, c));
                }
            }
        }
    }
    res
}

fn find_unique_triplets(s1: &Sum<Triplet>, s2: &Sum<Triplet>, s3: &Sum<Triplet>) -> Vec<(Triplet, Triplet, Triplet)> {
    let mut res = vec![];
    for t1 in &s1.0 {
        for t2 in &s2.0 {
            for t3 in &s3.0 {
                let mut digits = vec![t1.0, t1.1, t1.2, t2.0, t2.1, t2.2, t3.0, t3.1, t3.2];
                digits.sort();
                digits.dedup();
                if digits.len() == 9 {
                    res.push((*t1, *t2, *t3));
                }
            }
        }
    }
    res
}

fn print_triple_map(res: &IndexMap<Triplet, Vec<(Triplet, Triplet, Triplet)>>) {
    for ((s1, s2, s3), triplets) in res {
        println!("|{s1} {s2} {s3}|");
        for ((a1, a2, a3), (b1, b2, b3), (c1, c2, c3)) in triplets {
            println!(" {}{}{} {}{}{} {}{}{}", a1, a2, a3, b1, b2, b3, c1, c2, c3);
        }
        println!();
    }
}

fn get_pairs_sequence(
    triplet_map: &IndexMap<Triplet, Vec<(Triplet, Triplet, Triplet)>>,
) -> IndexMap<Triplet, [usize; 12]> {
    let mut pairs_sequence: IndexMap<Triplet, [_; 12]> = IndexMap::new();

    for k @ (a, b, c) in triplet_map.keys().cloned() {
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

fn print_pairs_sequence(pairs_sequence: &IndexMap<(usize, usize, usize), [usize; 12]>) {
    for (key, sequence) in pairs_sequence {
        let (a, b, c) = key;
        println!("|{} {} {}| = {:?}", a, b, c, sequence);
    }
}

fn triplet_sums() -> Vec<Sum<Triplet>> {
    vec![
        Sum(vec![(1, 2, 3)]),
        Sum(vec![(1, 2, 4)]),
        Sum(vec![(1, 2, 5), (1, 3, 4)]),
        Sum(vec![(1, 2, 6), (1, 3, 5), (2, 3, 4)]),
        Sum(vec![(1, 2, 7), (1, 3, 6), (1, 4, 5), (2, 3, 5)]),
        Sum(vec![(1, 2, 8), (1, 3, 7), (1, 4, 6), (2, 3, 6), (2, 4, 5)]),
        Sum(vec![(1, 2, 9), (1, 3, 8), (1, 4, 7), (1, 5, 6), (2, 3, 7), (2, 4, 6), (3, 4, 5)]),
        Sum(vec![(1, 3, 9), (1, 4, 8), (1, 5, 7), (2, 3, 8), (2, 4, 7), (2, 5, 6), (3, 4, 6)]),
        Sum(vec![(1, 4, 9), (1, 5, 8), (1, 6, 7), (2, 3, 9), (2, 4, 8), (2, 5, 7), (3, 4, 7), (3, 5, 6)]),
        Sum(vec![(1, 5, 9), (1, 6, 8), (2, 4, 9), (2, 5, 8), (2, 6, 7), (3, 4, 8), (3, 5, 7), (4, 5, 6)]),
        Sum(vec![(1, 6, 9), (1, 7, 8), (2, 5, 9), (2, 6, 8), (3, 4, 9), (3, 5, 8), (3, 6, 7), (4, 5, 7)]),
        Sum(vec![(1, 7, 9), (2, 6, 9), (2, 7, 8), (3, 5, 9), (3, 6, 8), (4, 5, 9), (4, 6, 7)]),
        Sum(vec![(1, 8, 9), (2, 7, 9), (3, 6, 9), (3, 7, 8), (4, 5, 8), (4, 6, 7)]),
        Sum(vec![(2, 8, 9), (3, 7, 9), (4, 6, 9), (4, 7, 8)]),
        Sum(vec![(3, 8, 9), (4, 7, 9), (5, 6, 9), (5, 7, 8)]),
    ]
}

fn pair_sums() -> Vec<Sum<Pair>> {
    vec![
        Sum(vec![(1, 2)]),
        Sum(vec![(1, 3)]),
        Sum(vec![(1, 4), (2, 3)]),
        Sum(vec![(1, 5), (2, 4)]),
        Sum(vec![(1, 6), (2, 5), (3, 4)]),
        Sum(vec![(1, 7), (2, 6), (3, 5)]),
        Sum(vec![(1, 8), (2, 7), (3, 6), (4, 5)]),
        Sum(vec![(1, 9), (2, 8), (3, 7), (4, 6)]),
        Sum(vec![(2, 9), (3, 8), (4, 7), (5, 6)]),
        Sum(vec![(3, 9), (4, 8), (5, 7)]),
        Sum(vec![(4, 9), (5, 8), (6, 7)]),
        Sum(vec![(5, 9), (6, 8)]),
        Sum(vec![(6, 9), (7, 8)]),
        Sum(vec![(7, 9)]),
        Sum(vec![(8, 9)]),
    ]
}
