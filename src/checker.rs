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

pub fn fill_constraints(set: &[Pair], dups: [u16; 3], results: &mut Vec<[[i32; 2]; 12]>) {
    fill_constraints_internal([[0, 0]; 12], 0, set, dups, results);
}

fn fill_constraints_internal(
    current: [[i32; 2]; 12],
    index: usize,
    set: &[Pair],
    dups: [u16; 3],
    results: &mut Vec<[[i32; 2]; 12]>,
) {
    let constraint_a = get_constraint(&current, dups, index * 2);
    let constraint_b = get_constraint(&current, dups, index * 2 + 1);

    let [a, b] = current[index];
    let mask_a = if a < 0 { -a as _ } else { 0 };
    let mask_b = if b < 0 { -b as _ } else { 0 };

    let set_iter = if index < 6 { &set[0..set.len() - 6] } else { set }.iter().enumerate();

    for (i, [x, y]) in set_iter {
        for [&a, &b] in [[x, y], [y, x]] {
            if check_constraints_pass(a, b, &constraint_a, &constraint_b, mask_a, mask_b) {
                let mut new_current = current;
                new_current[index] = [a.into(), b.into()];

                if index == 11 {
                    results.push(new_current);
                    continue;
                }

                apply_constraints(&mut new_current, index, a, b);

                let mut new_set = set.to_vec();
                new_set.remove(i);

                fill_constraints_internal(new_current, index + 1, &new_set, dups, results);
            }
        }
    }
}

fn check_constraints_pass(
    a: u16,
    b: u16,
    pos_constraint_a: &[u16],
    pos_constraint_b: &[u16],
    neg_mask_a: u16,
    neg_mask_b: u16,
) -> bool {
    (pos_constraint_a.is_empty() || pos_constraint_a.contains(&a))
        && (pos_constraint_b.is_empty() || pos_constraint_b.contains(&b))
        && (neg_mask_a == 0 || (neg_mask_a & (1 << a)) == 0)
        && (neg_mask_b == 0 || (neg_mask_b & (1 << b)) == 0)
}

fn apply_constraints(current: &mut [[i32; 2]; 12], index: usize, a: u16, b: u16) {
    let mask_a = CONSTRAINT_MASKS[index * 2];
    let mask_b = CONSTRAINT_MASKS[index * 2 + 1];

    for j in 0..24 {
        if (mask_a & (1 << j)) != 0 {
            let k = &mut current[j / 2][j % 2];
            if *k <= 0 {
                *k = -(-*k | (1 << a));
            }
        }
        if (mask_b & (1 << j)) != 0 {
            let k = &mut current[j / 2][j % 2];
            if *k <= 0 {
                *k = -(-*k | (1 << b));
            }
        }
    }
}

fn get_constraint(current: &[[i32; 2]; 12], dups: [u16; 3], index: usize) -> Vec<u16> {
    match index {
        i @ (3 | 5 | 8 | 15 | 17 | 20) => {
            let y = (i + 12) % 24;
            let (div, rem) = (y / 2, y % 2);

            let value = current[div][rem];
            if value > 0 { vec![value.try_into().unwrap()] } else { dups.to_vec() }
        }
        _ => Vec::new(),
    }
}

fn check_constraints(digits: &[usize; 24], dups: [usize; 3]) -> bool {
    let (a1, b1, c1) = (digits[3], digits[5], digits[8]);
    let (a2, b2, c2) = (digits[15], digits[17], digits[20]);

    if a1 != a2 || b1 != b2 || c1 != c2 {
        return false;
    }

    if a1 == b1 || b1 == c1 || c1 == a1 {
        return false;
    }

    if !dups.contains(&a1) || !dups.contains(&b1) || !dups.contains(&c1) {
        return false;
    }

    for (i, d) in digits.iter().enumerate() {
        let constraint_mask = CONSTRAINT_MASKS[i];
        for (j, digit) in digits.iter().enumerate() {
            if (constraint_mask & (1 << j)) != 0 && digit == d {
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
        let set = vec![[1, 3], [1, 6], [2, 7], [4, 8], [5, 8], [6, 9], [1, 4], [1, 5], [2, 6], [3, 8], [6, 8], [7, 9]];

        fill_constraints(&set, dups, &mut results);
        println!("{:?}", results);
    }

    #[test]
    fn test_constraint() {
        // [[6, 1], [4, 8], [9, 6], [5, 8], [1, 3], [2, 7], [7, 9], [6, 8], [2, 6], [4, 1], [1, 5], [3, 8]]
        let digits = [6, 1, 4, 8, 9, 6, 5, 8, 1, 3, 2, 7, 7, 9, 6, 8, 2, 6, 4, 1, 1, 5, 3, 8];

        assert!(check_constraints(&digits, [1, 6, 8]));
    }
}
