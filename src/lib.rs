// TODO: nothing in lib.rs

mod boolean_tree;
mod eval_formula;
mod space_filling_curves;
mod truth_table;

use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    sync::LazyLock,
};

pub use {
    boolean_tree::BooleanTree,
    eval_formula::eval_formula,
    space_filling_curves::{map, reverse_map},
    truth_table::print_truth_table,
};

pub fn adder(a: u32, b: u32) -> u32 {
    let mut res = 0;
    let mut carry = 0;
    for i in 0..32 {
        let ai = (a >> i) & 1;
        let bi = (b >> i) & 1;
        res |= (ai ^ bi ^ carry) << i;
        carry = (ai & bi) | (ai & carry) | (bi & carry);
    }
    return res;
}

pub fn multiplier(a: u32, b: u32) -> u32 {
    let mut res = 0;
    for i in 0..32 {
        let ai = (a >> i) & 1;
        let mask = adder(!ai, 1);
        res = adder(res, (b & mask) << i);
    }
    res
}

pub fn gray_code(n: u32) -> u32 {
    n ^ n >> 1
}

fn parse_formula(formula: &str) -> BooleanTree {
    BooleanTree::new(&formula, true)
        .unwrap_or_else(|err| panic!("failed to parse formula \"{formula}\": {err}"))
}

pub fn negation_normal_form(formula: &str) -> String {
    let mut tree = parse_formula(formula);
    tree.make_nnf();
    tree.to_formula()
}

pub fn conjunctive_normal_form(formula: &str) -> String {
    let mut tree = parse_formula(formula);
    tree.make_cnf();
    tree.to_formula()
}

pub fn sat(formula: &str) -> bool {
    parse_formula(formula).is_satisfiable()
}

// &HashSet would make more sense as input but gotta respect the subject
pub fn powerset(set: Vec<i32>) -> Vec<Vec<i32>> {
    fn _powerset(set: &Vec<i32>, i: usize, current: &mut Vec<i32>, output: &mut Vec<Vec<i32>>) {
        if i == set.len() {
            output.push(current.clone());
            return;
        }
        _powerset(set, i + 1, current, output);
        current.push(set[i]);
        _powerset(set, i + 1, current, output);
        current.pop();
    }

    let mut output = vec![];
    _powerset(&set, 0, &mut vec![], &mut output);
    output
}

pub fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32> {
    type SetOperation = fn(HashSet<i32>, HashSet<i32>) -> HashSet<i32>;

    static SET_OPERATIONS: LazyLock<HashMap<char, SetOperation>> = LazyLock::new(|| {
        fn union_(a: HashSet<i32>, b: HashSet<i32>) -> HashSet<i32> {
            a.union(&b).copied().collect()
        }

        fn intersection(a: HashSet<i32>, b: HashSet<i32>) -> HashSet<i32> {
            a.intersection(&b).copied().collect()
        }

        HashMap::from([
            ('|', union_ as SetOperation),
            ('&', intersection as SetOperation),
        ])
    });

    let universe: HashSet<i32> =
        HashSet::from_iter(sets.iter().flat_map(|set| set.iter().copied()));
    let sets: Vec<HashSet<i32>> = sets
        .into_iter()
        .map(|v| HashSet::from_iter(v.iter().copied()))
        .collect_vec();
    let mut stack = vec![];
    for c in formula.chars() {
        if c.is_ascii_uppercase() {
            let i = c as usize - 'A' as usize;
            if i >= sets.len() {
                panic!(
                    "Got letter {c} (index {i}) but only {} sets available",
                    sets.len()
                );
            }
            stack.push(sets[i].clone());
        } else if c == '!' {
            if stack.is_empty() {
                panic!("no operand for set negation");
            }
            let set = stack.pop().unwrap();
            stack.push(universe.difference(&set).copied().collect());
        } else if let Some(set_operation) = SET_OPERATIONS.get(&c) {
            if stack.len() < 2 {
                panic!("not enough operands for set operation");
            }
            let a = stack.pop().unwrap();
            let b = stack.pop().unwrap();
            stack.push(set_operation(b, a));
        } else {
            panic!("invalid character: {}", c);
        }
    }

    match stack.len() {
        0 => panic!("empty formula"),
        1 => stack.pop().unwrap().into_iter().sorted_unstable().collect(),
        _ => panic!("not enough operators"),
    }
}

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

    #[test]
    fn test_sat() {
        assert!(!sat("0"));
        assert!(sat("1"));
        assert!(sat("0!"));
        assert!(!sat("1!"));
        assert!(sat("A"));
        assert!(!sat("AA!&"));
        assert!(sat("AB|"));
        assert!(sat("AB&"));
        assert!(!sat("AA^"));
        assert!(!sat("ABCD^^^ABCD===&"));
    }

    #[test]
    fn test_powerset() {
        assert_eq!(powerset(vec![]), vec![vec![]]);
        assert_eq!(powerset(vec![42]), vec![vec![], vec![42]]);
        assert_eq!(
            powerset(vec![1, 2, 3]),
            vec![
                vec![],
                vec![3],
                vec![2],
                vec![2, 3],
                vec![1,],
                vec![1, 3],
                vec![1, 2],
                vec![1, 2, 3],
            ]
        );
    }
}
