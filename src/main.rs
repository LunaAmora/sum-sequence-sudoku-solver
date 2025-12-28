/// Limited cage pair combinations solver.
mod checker_v1;

/// Experimental full sudoku solver.
#[allow(dead_code)]
mod checker_v2;

mod combinations;
mod sums;

use checker_v1::fill_constraints;
use combinations::compute_combinations;
use indexmap::IndexMap;
use std::fs::{self, File};
use std::io::{BufWriter, Result, Write};
use sums::*;

fn main() -> Result<()> {
    let pair_sums = pair_sums();
    let triplet_sums = triplet_sums();

    let mut triplet_map = get_triplet_map(&triplet_sums);
    let mut pairs_sequence = get_pairs_sequence(&triplet_map);

    let combinations = get_combinations(&mut triplet_map, &mut pairs_sequence, &pair_sums);
    let solutions = get_solutions(&combinations);

    fs::create_dir_all("out")?;

    let mut file = BufWriter::new(File::create("out/1_triplet_map.txt")?);
    print_triple_map(&mut file, &triplet_map)?;
    file.flush()?;

    let mut file = BufWriter::new(File::create("out/2_pairs_sequence.txt")?);
    print_pairs_sequence(&mut file, &pairs_sequence)?;
    file.flush()?;

    let mut file = BufWriter::new(File::create("out/3_combinations.txt")?);
    print_combinations(&mut file, &combinations)?;
    file.flush()?;

    let mut file = BufWriter::new(File::create("out/4_solutions.txt")?);
    print_solutions(&mut file, &solutions)?;
    file.flush()?;

    Ok(())
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

fn print_combinations<W: Write>(w: &mut W, combinations: &IndexMap<Triplet, Vec<[Pair; 12]>>) -> Result<()> {
    for (triplet, combination) in combinations {
        if !combination.is_empty() {
            writeln!(w, "Found {} valid combination(s) for triplet {:?}", combination.len(), triplet)?;

            for sequence in combination {
                let mut digit_count = [0; 9];
                for &[a, b] in &sequence[0..6] {
                    digit_count[a as usize - 1] += 1;
                    digit_count[b as usize - 1] += 1;
                }
                let dups: Vec<usize> = (0..9).filter(|&digit| digit_count[digit] == 2).map(|i| i + 1).collect();

                write!(w, "  {:?} =", dups)?;
                for [a, b] in &sequence[0..6] {
                    write!(w, " {}{}", a, b)?;
                }
                write!(w, " |")?;
                for [a, b] in &sequence[6..12] {
                    write!(w, " {}{}", a, b)?;
                }
                writeln!(w)?;
            }
        }
    }

    Ok(())
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

fn print_solutions<W: Write>(w: &mut W, solutions: &IndexMap<Triplet, Vec<[Pair; 12]>>) -> Result<()> {
    for (triplet, solution) in solutions {
        if !solution.is_empty() {
            writeln!(w, "Found {} valid solution(s) for triplet {:?}", solution.len(), triplet)?;

            for sequence in solution {
                let mut digit_count = [0; 9];
                for &[a, b] in &sequence[0..6] {
                    digit_count[a as usize - 1] += 1;
                    digit_count[b as usize - 1] += 1;
                }
                let dups: Vec<usize> = (0..9).filter(|&digit| digit_count[digit] == 2).map(|i| i + 1).collect();

                write!(w, "  {:?} =", dups)?;
                for [a, b] in &sequence[0..6] {
                    write!(w, " {}{}", a, b)?;
                }
                write!(w, " |")?;
                for [a, b] in &sequence[6..12] {
                    write!(w, " {}{}", a, b)?;
                }
                writeln!(w)?;
            }
        }
    }

    Ok(())
}
