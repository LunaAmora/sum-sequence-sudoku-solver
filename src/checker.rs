use super::Pair;

const CONSTRAINTS: [[u8; 24]; 24] = [
    // 2, 4, 6, 7, 12, 13
    [0, 0, 1, 0, 1, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    // 3, 5, 6, 7, 12, 13
    [0, 0, 0, 1, 0, 1, 1, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    // 0, 4, 8, 9
    [1, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    // 1, 5, 8, 9, 13, 17, 20, 21
    [0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0],
    // 0, 2, 10, 11, 18, 19
    [1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
    // 1, 2, 3, 6, 7, 8, 9, 10, 11, 12, 13, 15, 18, 19, 20, 22, 23
    [0, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 1, 1, 1, 0, 1, 1],
    // 0, 1, 8, 10, 16, 17
    [1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0],
    // 0, 1, 9, 11, 16, 17
    [1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0],
    // 2, 3, 6, 10, 14, 15, 18, 22
    [0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 0],
    // 2, 3, 7, 11
    [0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    // 4, 5, 6, 8, 22, 23
    [0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
    // 4, 5, 7, 9, 22, 23
    [0, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1],
    // 0, 1, 14, 16, 18, 19
    [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0, 0, 0],
    // other set
    // 0, 1, 15, 17, 18, 19
    [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 0, 0, 0, 0],
    // 12, 16, 20, 21
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0],
    // 1, 5, 8, 9, 13, 17, 20, 21
    [0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 0],
    // 6, 7, 12, 14, 22, 23
    [0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1],
    // 1, 2, 3, 6, 7, 8, 9, 10, 11, 12, 13, 15, 18, 19, 20, 22, 23
    [0, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 1, 1, 1, 0, 1, 1],
    // 4, 5, 12, 13, 20, 22
    [0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0],
    // 4, 5, 12, 13, 21, 23
    [0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1],
    // 2, 3, 6, 10, 14, 15, 18, 22
    [0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 0],
    // 14, 15, 19, 23
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 1],
    // 10, 11, 16, 17, 18, 20
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 0],
    // 10, 11, 16, 17, 19, 21
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0],
];

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

    for (i, row) in CONSTRAINTS.iter().enumerate() {
        let d = digits[i];
        for (j, &val) in row.iter().enumerate() {
            if val == 1 && digits[j] == d {
                return false;
            }
        }
    }
    true
}

pub fn fill_contraints(
    mut current: [[isize; 2]; 12],
    index: usize,
    mut set1: Vec<Pair>,
    mut set2: Vec<Pair>,
    dups: [usize; 3],
    results: &mut Vec<[[isize; 2]; 12]>,
) {
    let set = if index < 6 { &set1 } else { &set2 };

    let pos_constraint_a = get_pos_constraint(&current, dups, index * 2);
    let pos_constraint_b = get_pos_constraint(&current, dups, index * 2 + 1);

    let [a, b] = current[index];
    let neg_constraint_a = if a < 0 { (1..=9).filter(|d| (-a & (1 << d)) == 0).collect() } else { Vec::new() };
    let neg_constraint_b = if b < 0 { (1..=9).filter(|d| (-b & (1 << d)) == 0).collect() } else { Vec::new() };

    for (i, [x, y]) in set.iter().enumerate() {
        for [a, b] in [[x, y], [y, x]] {
            let pas_a = pos_constraint_a.is_empty() || pos_constraint_a.contains(a);
            let pas_b = pos_constraint_b.is_empty() || pos_constraint_b.contains(b);

            let pas_neg_a = neg_constraint_a.is_empty() || !neg_constraint_a.contains(a);
            let pas_neg_b = neg_constraint_b.is_empty() || !neg_constraint_b.contains(b);

            if pas_a && pas_b && pas_neg_a && pas_neg_b {
                let mut new_current = current;
                new_current[index] = [*a as _, *b as _];

                if index == 11 {
                    results.push(new_current);
                    continue;
                }

                let mask_a = CONSTRAINTS[index * 2];
                let mask_b = CONSTRAINTS[index * 2 + 1];

                for (j, &val) in mask_a.iter().enumerate() {
                    if val == 1 {
                        let k = &mut new_current[j / 2][j % 2];
                        if *k <= 0 {
                            *k = -(-*k | (1 << a));
                        }
                    }
                }
                for (j, &val) in mask_b.iter().enumerate() {
                    if val == 1 {
                        let k = &mut new_current[j / 2][j % 2];
                        if *k <= 0 {
                            *k = -(-*k | (1 << b));
                        }
                    }
                }

                let mut new_set1 = set1.clone();
                let mut new_set2 = set2.clone();

                if index < 6 {
                    new_set1.remove(i);
                } else {
                    new_set2.remove(i);
                }

                fill_contraints(new_current, index + 1, new_set1, new_set2, dups, results);
            }
        }
    }
}

fn get_pos_constraint(current: &[[isize; 2]; 12], dups: [usize; 3], index: usize) -> Vec<usize> {
    match index {
        i @ (3 | 5 | 8 | 15 | 17 | 20) => {
            let y = (i + 12) % 24;
            let (div, rem) = (y / 2, y % 2);

            let value = current[div][rem];
            if value > 0 { vec![value as _] } else { dups.to_vec() }
        }
        _ => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_setup() {
        let mut results = vec![];

        let dups = [1, 6, 8];
        let set1 = vec![[1, 3], [1, 6], [2, 7], [4, 8], [5, 8], [6, 9]];
        let set2 = vec![[1, 4], [1, 5], [2, 6], [3, 8], [6, 8], [7, 9]];

        fill_contraints([[0, 0]; 12], 0, set1, set2, dups, &mut results);
        println!("{:?}", results);
    }

    #[test]
    #[should_panic(expected = "Constraint violated")]
    fn test_constraint() {
        // [1, 6, 8]: 13 16 27 48 58 69 | 14 15 26 38 68 79
        let digits = [1, 3, 1, 6, 2, 7, 4, 8, 5, 8, 6, 9, 1, 4, 1, 5, 2, 6, 3, 8, 6, 8, 7, 9];

        if !check_constraints(&digits, [1, 6, 8]) {
            panic!("Constraint violated");
        }
    }
}
