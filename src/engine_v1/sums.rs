use indexmap::IndexMap;
use std::io::{Result, Write};

pub type Pair = [u16; 2];
pub type Triplet = [u16; 3];
pub type CornerTriplets = [Triplet; 3];

pub fn pair_sums() -> Vec<Vec<Pair>> {
    vec![
        vec![[1, 3]],
        vec![[1, 4], [2, 3]],
        vec![[1, 5], [2, 4]],
        vec![[1, 6], [2, 5], [3, 4]],
        vec![[1, 7], [2, 6], [3, 5]],
        vec![[1, 8], [2, 7], [3, 6], [4, 5]],
        vec![[1, 9], [2, 8], [3, 7], [4, 6]],
        vec![[2, 9], [3, 8], [4, 7], [5, 6]],
        vec![[3, 9], [4, 8], [5, 7]],
        vec![[4, 9], [5, 8], [6, 7]],
        vec![[5, 9], [6, 8]],
        vec![[6, 9], [7, 8]],
        vec![[7, 9]],
        vec![[8, 9]],
    ]
}

pub fn get_pairs_sequence(triplet_map: &IndexMap<Triplet, Vec<CornerTriplets>>) -> IndexMap<Triplet, [u16; 12]> {
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

pub fn print_pairs_sequence<W: Write>(w: &mut W, pairs_sequence: &IndexMap<Triplet, [u16; 12]>) -> Result<()> {
    for (key, sequence) in pairs_sequence {
        let key = format!("{:?}", key);
        writeln!(w, "{:12} = {:2?}", key, sequence)?;
    }
    Ok(())
}

pub fn triplet_sums() -> Vec<Vec<Triplet>> {
    vec![
        vec![[1, 2, 3]],
        vec![[1, 2, 4]],
        vec![[1, 2, 5], [1, 3, 4]],
        vec![[1, 2, 6], [1, 3, 5], [2, 3, 4]],
        vec![[1, 2, 7], [1, 3, 6], [1, 4, 5], [2, 3, 5]],
        vec![[1, 2, 8], [1, 3, 7], [1, 4, 6], [2, 3, 6], [2, 4, 5]],
        vec![[1, 2, 9], [1, 3, 8], [1, 4, 7], [1, 5, 6], [2, 3, 7], [2, 4, 6], [3, 4, 5]],
        vec![[1, 3, 9], [1, 4, 8], [1, 5, 7], [2, 3, 8], [2, 4, 7], [2, 5, 6], [3, 4, 6]],
        vec![[1, 4, 9], [1, 5, 8], [1, 6, 7], [2, 3, 9], [2, 4, 8], [2, 5, 7], [3, 4, 7], [3, 5, 6]],
        vec![[1, 5, 9], [1, 6, 8], [2, 4, 9], [2, 5, 8], [2, 6, 7], [3, 4, 8], [3, 5, 7], [4, 5, 6]],
        vec![[1, 6, 9], [1, 7, 8], [2, 5, 9], [2, 6, 8], [3, 4, 9], [3, 5, 8], [3, 6, 7], [4, 5, 7]],
        vec![[1, 7, 9], [2, 6, 9], [2, 7, 8], [3, 5, 9], [3, 6, 8], [4, 5, 9], [4, 6, 7]],
        vec![[1, 8, 9], [2, 7, 9], [3, 6, 9], [3, 7, 8], [4, 5, 8], [4, 6, 7]],
    ]
}

pub fn get_triplet_map(triplet_sums: &[Vec<Triplet>]) -> IndexMap<Triplet, Vec<CornerTriplets>> {
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

pub fn print_triple_map<W: Write>(w: &mut W, triplet_map: &IndexMap<Triplet, Vec<CornerTriplets>>) -> Result<()> {
    for (sum, triplets) in triplet_map {
        writeln!(w, "{:?}", sum)?;
        for [[a1, a2, a3], [b1, b2, b3], [c1, c2, c3]] in triplets {
            writeln!(w, " {}{}{} {}{}{} {}{}{}", a1, a2, a3, b1, b2, b3, c1, c2, c3)?;
        }
        writeln!(w)?;
    }

    Ok(())
}
