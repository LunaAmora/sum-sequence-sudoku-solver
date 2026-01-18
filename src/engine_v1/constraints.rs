use super::Pair;

#[rustfmt::skip]
const CONSTRAINT_MASKS: [u32; 24] = [
    1<<2  | 1<<4  | 1<<6  | 1<<7  | 1<<12 | 1<<13,
    1<<3  | 1<<5  | 1<<6  | 1<<7  | 1<<12 | 1<<13,
    1<<0  | 1<<4  | 1<<5  | 1<<8  | 1<<9  | 1<<17,
    1<<1  | 1<<5  | 1<<8  | 1<<9  | 1<<13 | 1<<17 | 1<<20 | 1<<21,
    1<<0  | 1<<2  | 1<<10 | 1<<11 | 1<<18 | 1<<19,
    1<<1  | 1<<2  | 1<<3  | 1<<6  | 1<<7  | 1<<8  | 1<<9  | 1<<10 | 1<<11 | 1<<12 | 1<<13 | 1<<15 | 1<<18 | 1<<19 | 1<<20 | 1<<22 | 1<<23,
    1<<0  | 1<<1  | 1<<8  | 1<<10 | 1<<16 | 1<<17,
    1<<0  | 1<<1  | 1<<9  | 1<<11 | 1<<16 | 1<<17,
    1<<2  | 1<<3  | 1<<6  | 1<<10 | 1<<14 | 1<<15 | 1<<18 | 1<<22,
    1<<2  | 1<<3  | 1<<7  | 1<<11,
    1<<4  | 1<<5  | 1<<6  | 1<<8  | 1<<22 | 1<<23,
    1<<4  | 1<<5  | 1<<7  | 1<<9  | 1<<22 | 1<<23,
    1<<0  | 1<<1  | 1<<14 | 1<<16 | 1<<18 | 1<<19,
    1<<0  | 1<<1  | 1<<15 | 1<<17 | 1<<18 | 1<<19,
    1<<12 | 1<<16 | 1<<20 | 1<<21,
    1<<1  | 1<<5  | 1<<8  | 1<<9  | 1<<13 | 1<<17 | 1<<20 | 1<<21,
    1<<6  | 1<<7  | 1<<12 | 1<<14 | 1<<22 | 1<<23,
    1<<1  | 1<<2  | 1<<3  | 1<<6  | 1<<7  | 1<<8  | 1<<9  | 1<<10 | 1<<11 | 1<<12 | 1<<13 | 1<<15 | 1<<18 | 1<<19 | 1<<20 | 1<<22 | 1<<23,
    1<<4  | 1<<5  | 1<<12 | 1<<13 | 1<<20 | 1<<22,
    1<<4  | 1<<5  | 1<<12 | 1<<13 | 1<<21 | 1<<23,
    1<<2  | 1<<3  | 1<<6  | 1<<10 | 1<<14 | 1<<15 | 1<<18 | 1<<22,
    1<<14 | 1<<15 | 1<<19 | 1<<23,
    1<<10 | 1<<11 | 1<<16 | 1<<17 | 1<<18 | 1<<20,
    1<<10 | 1<<11 | 1<<16 | 1<<17 | 1<<19 | 1<<21,
];

pub fn fill_constraints(set: &[Pair; 12], dups: [u16; 3], results: &mut Vec<[[u16; 2]; 12]>) {
    fill_constraints_internal([[0, 0]; 12], 0, set, dups, results);
}

fn fill_constraints_internal(
    current: [[u16; 2]; 12],
    index: usize,
    pairs: &[Pair],
    dups: [u16; 3],
    results: &mut Vec<[[u16; 2]; 12]>,
) {
    let constraint_a = get_constraint(&current, dups, index * 2);
    let constraint_b = get_constraint(&current, dups, index * 2 + 1);

    let [a, b] = current[index];
    let mask_a = extract_forbidden_digit_mask(a);
    let mask_b = extract_forbidden_digit_mask(b);

    let available_pairs = if index < 6 { &pairs[0..pairs.len() - 6] } else { pairs };

    for (i, [x, y]) in available_pairs.iter().enumerate() {
        for [&a, &b] in [[x, y], [y, x]] {
            if is_pair_valid_for_position(a, b, constraint_a, constraint_b, mask_a, mask_b) {
                let mut new_current = current;
                new_current[index] = [a, b];

                if index == 11 {
                    results.push(new_current);
                    continue;
                }

                apply_constraints(&mut new_current, index, a, b);

                let mut remaining_pairs = pairs.to_vec();
                remaining_pairs.remove(i);

                fill_constraints_internal(new_current, index + 1, &remaining_pairs, dups, results);
            }
        }
    }
}

fn is_pair_valid_for_position(
    digit_a: u16,
    digit_b: u16,
    constraint_mask_a: u16,
    constraint_mask_b: u16,
    forbidden_mask_a: u16,
    forbidden_mask_b: u16,
) -> bool {
    satisfies_positive_constraints(digit_a, constraint_mask_a)
        && satisfies_positive_constraints(digit_b, constraint_mask_b)
        && !is_digit_forbidden(digit_a, forbidden_mask_a)
        && !is_digit_forbidden(digit_b, forbidden_mask_b)
}

fn satisfies_positive_constraints(digit: u16, allowed_mask: u16) -> bool {
    allowed_mask == 0 || (allowed_mask & (1 << digit)) != 0
}

fn is_digit_forbidden(digit: u16, forbidden_mask: u16) -> bool {
    forbidden_mask != 0 && (forbidden_mask & (1 << digit)) != 0
}

fn apply_constraints(current: &mut [[u16; 2]; 12], index: usize, digit_a: u16, digit_b: u16) {
    apply_digit_constraints_to_positions(current, CONSTRAINT_MASKS[index * 2], digit_a);
    apply_digit_constraints_to_positions(current, CONSTRAINT_MASKS[index * 2 + 1], digit_b);
}

fn apply_digit_constraints_to_positions(current: &mut [[u16; 2]; 12], mut position_mask: u32, forbidden_digit: u16) {
    while position_mask != 0 {
        let position = position_mask.trailing_zeros() as usize;
        position_mask &= position_mask - 1;
        let constraint = &mut current[position / 2][position % 2];
        add_forbidden_digit_to_constraint(constraint, forbidden_digit);
    }
}

/// Bits 1-9 for Sudoku digits
const DIGIT_MASK: u16 = 0x3FE;
/// Bit 15: constraint type flag
const FORBIDDEN_FLAG: u16 = 0x8000;

fn add_forbidden_digit_to_constraint(constraint: &mut u16, digit: u16) {
    if *constraint & FORBIDDEN_FLAG != 0 || *constraint == 0 {
        let current_forbidden_mask = *constraint & DIGIT_MASK;
        let new_forbidden_mask = current_forbidden_mask | (1u16 << digit);
        *constraint = FORBIDDEN_FLAG | new_forbidden_mask;
    }
}

fn extract_forbidden_digit_mask(constraint_value: u16) -> u16 {
    if constraint_value & FORBIDDEN_FLAG != 0 { constraint_value & DIGIT_MASK } else { 0 }
}

const INDEXES: [usize; 6] = [3, 5, 8, 15, 17, 20];

fn get_constraint(current: &[[u16; 2]; 12], dups: [u16; 3], index: usize) -> u16 {
    if INDEXES.contains(&index) {
        let y = (index + 12) % 24;
        let (div, rem) = (y / 2, y % 2);

        let constraint_value = current[div][rem];

        if constraint_value > 0 && constraint_value & FORBIDDEN_FLAG == 0 {
            1 << constraint_value
        } else {
            dups.iter().fold(0u16, |mask, &digit| mask | (1 << digit))
        }
    } else {
        0
    }
}

#[allow(dead_code)]
fn check_constraints(solution: &[[u16; 2]; 12], dups: [u16; 3]) -> bool {
    let get_digit = |index: usize| -> u16 { solution[index / 2][index % 2] };

    let (a1, b1, c1) = (get_digit(INDEXES[0]), get_digit(INDEXES[1]), get_digit(INDEXES[2]));
    let (a2, b2, c2) = (get_digit(INDEXES[3]), get_digit(INDEXES[4]), get_digit(INDEXES[5]));

    if a1 != a2 || b1 != b2 || c1 != c2 {
        return false;
    }

    if a1 == b1 || b1 == c1 || c1 == a1 {
        return false;
    }

    if !dups.contains(&a1) || !dups.contains(&b1) || !dups.contains(&c1) {
        return false;
    }

    for (i, &constraint_mask) in CONSTRAINT_MASKS.iter().enumerate() {
        let digit_i = get_digit(i);

        for j in 0..24 {
            if (constraint_mask & (1 << j)) != 0 && get_digit(j) == digit_i {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_setup() {
        let mut results = vec![];

        let dups = [1, 6, 8];
        let set = [[1, 3], [1, 6], [2, 7], [4, 8], [5, 8], [6, 9], [1, 4], [1, 5], [2, 6], [3, 8], [6, 8], [7, 9]];

        fill_constraints(&set, dups, &mut results);

        assert!(!results.is_empty());
        for res in results {
            assert!(check_constraints(&res, dups));
        }
    }

    #[test]
    fn test_constraint() {
        let solution = [[6, 1], [4, 8], [9, 6], [5, 8], [1, 3], [2, 7], [7, 9], [6, 8], [2, 6], [4, 1], [1, 5], [3, 8]];

        assert!(check_constraints(&solution, [1, 6, 8]));
    }
}
