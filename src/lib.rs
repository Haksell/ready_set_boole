// TODO: nothing in lib.rs

mod boolean_tree;
mod eval_formula;
mod truth_table;

pub use {boolean_tree::BooleanTree, eval_formula::eval_formula, truth_table::print_truth_table};

pub fn adder(a: u32, b: u32) -> u32 {
    let mut res = 0;
    let mut carry = 0;
    for i in 0..32 {
        let da = a >> i & 1;
        let db = b >> i & 1;
        res |= (da ^ db ^ carry) << i;
        carry = (da & db) | (da & carry) | (db & carry);
    }
    return res;
}

pub fn multiplier(a: u32, b: u32) -> u32 {
    let mut res = 0;
    for i in 0..32 {
        let da = a >> i & 1;
        let mask = adder(!da, 1);
        res = adder(res, (b & mask) << i);
    }
    res
}

pub fn gray_code(n: u32) -> u32 {
    n ^ n >> 1
}

pub fn negation_normal_form(formula: &str) -> String {
    let mut tree = BooleanTree::new(&formula, true)
        .unwrap_or_else(|err| panic!("failed to parse formula \"{formula}\": {err}"));
    tree.make_nnf();
    tree.to_formula()
}

// pub fn conjunctive_normal_form(formula: &str) -> String {
//     let mut tree = BooleanTree::new(&formula, true)
//         .unwrap_or_else(|err| panic!("failed to parse formula \"{formula}\": {err}"));
//     tree.to_cnf().to_formula()
// }

#[cfg(test)]
mod tests {
    use {super::*, rand::Rng};

    #[test]
    fn test_addition_table() {
        for i in 0..10 {
            for j in 0..10 {
                assert_eq!(adder(i, j), i + j);
            }
        }
    }

    #[test]
    fn test_adder_random() {
        let mut rng = rand::rng();
        for _ in 0..42 {
            let i = rng.random_range(0..u32::MAX);
            let j = rng.random_range(0..u32::MAX);
            assert_eq!(adder(i, j), i.wrapping_add(j));
        }
    }

    #[test]
    fn test_multiplication_table() {
        for i in 1..10 {
            for j in 3..10 {
                println!("{i}*{j}={}", multiplier(i, j));
                assert_eq!(multiplier(i, j), i * j);
            }
        }
    }

    #[test]
    fn test_multiplier_random() {
        let mut rng = rand::rng();
        for _ in 0..42 {
            let i = rng.random_range(0..u32::MAX);
            let j = rng.random_range(0..u32::MAX);
            println!("{i}*{j}={}", multiplier(i, j));
            assert_eq!(multiplier(i, j), i.wrapping_mul(j));
            println!();
        }
    }

    #[test]
    fn test_gray_code() {
        assert_eq!(gray_code(0), 0);
        assert_eq!(gray_code(1), 1);
        assert_eq!(gray_code(2), 3);
        assert_eq!(gray_code(3), 2);
        assert_eq!(gray_code(4), 6);
        assert_eq!(gray_code(5), 7);
        assert_eq!(gray_code(6), 5);
        assert_eq!(gray_code(7), 4);
        assert_eq!(gray_code(8), 12);
        assert_eq!(gray_code(9), 13);
        assert_eq!(gray_code(10), 15);
        assert_eq!(gray_code(11), 14);
        assert_eq!(gray_code(12), 10);
        assert_eq!(gray_code(13), 11);
        assert_eq!(gray_code(14), 9);
        assert_eq!(gray_code(15), 8);
        assert_eq!(gray_code(16), 24);
        assert_eq!(gray_code(17), 25);
        assert_eq!(gray_code(18), 27);
        assert_eq!(gray_code(19), 26);
        assert_eq!(gray_code(20), 30);
    }
}
