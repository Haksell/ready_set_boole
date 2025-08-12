use {
    itertools::Itertools,
    std::{
        collections::{HashMap, HashSet},
        sync::LazyLock,
    },
};

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

type SetOperation = fn(&HashSet<i32>, HashSet<i32>, HashSet<i32>) -> HashSet<i32>;

static SET_OPERATIONS: LazyLock<HashMap<char, SetOperation>> = LazyLock::new(|| {
    fn union_(_: &HashSet<i32>, mut a: HashSet<i32>, mut b: HashSet<i32>) -> HashSet<i32> {
        if a.len() > b.len() {
            a.extend(b);
            a
        } else {
            b.extend(a);
            b
        }
    }

    fn intersection(_: &HashSet<i32>, mut a: HashSet<i32>, mut b: HashSet<i32>) -> HashSet<i32> {
        if a.len() < b.len() {
            a.retain(|x| b.contains(x));
            a
        } else {
            b.retain(|x| a.contains(x));
            b
        }
    }

    fn symmetric_difference(
        _: &HashSet<i32>,
        mut a: HashSet<i32>,
        mut b: HashSet<i32>,
    ) -> HashSet<i32> {
        if a.len() < b.len() {
            [a, b] = [b, a];
        }
        for x in b {
            if a.contains(&x) {
                a.remove(&x);
            } else {
                a.insert(x);
            }
        }
        a
    }

    fn equivalence(universe: &HashSet<i32>, a: HashSet<i32>, b: HashSet<i32>) -> HashSet<i32> {
        universe
            .iter()
            .filter(|x| a.contains(&x) == b.contains(&x))
            .copied()
            .collect::<HashSet<i32>>()
    }

    fn material_condition(
        universe: &HashSet<i32>,
        a: HashSet<i32>,
        mut b: HashSet<i32>,
    ) -> HashSet<i32> {
        b.extend(universe.difference(&a));
        b
    }

    HashMap::from([
        ('|', union_ as SetOperation),
        ('&', intersection as SetOperation),
        ('^', symmetric_difference as SetOperation),
        ('=', equivalence as SetOperation),
        ('>', material_condition as SetOperation),
    ])
});

pub fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32> {
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
                    "got set {c} (index {i}) but only {} sets available",
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
            stack.push(set_operation(&universe, b, a));
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
    use super::*;

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

    #[test]
    fn test_eval_one_set() {
        assert_eq!(eval_set("A", vec![vec![0, 1, 2]]), vec![0, 1, 2]);
        assert_eq!(eval_set("A!", vec![vec![0, 1, 2]]), vec![]);
        assert_eq!(
            eval_set("A!", vec![vec![0, 1, 2], vec![2, 3, 4]]),
            vec![3, 4]
        );
        assert_eq!(eval_set("A!!", vec![vec![0, 1, 2]]), vec![0, 1, 2]);
    }

    #[test]
    fn test_eval_two_sets() {
        assert_eq!(
            eval_set("AB|", vec![vec![0, 1, 2], vec![0, 3, 4]]),
            [0, 1, 2, 3, 4]
        );
        assert_eq!(eval_set("AB&", vec![vec![0, 1, 2], vec![0, 3, 4]]), [0]);
        assert_eq!(
            eval_set("AB^", vec![vec![0, 1, 2], vec![0, 3, 4]]),
            [1, 2, 3, 4]
        );
        assert_eq!(eval_set("AB=", vec![vec![0, 1, 2], vec![0, 3, 4]]), [0]);
        assert_eq!(
            eval_set("AB>", vec![vec![0, 1, 2], vec![0, 3, 4]]),
            [0, 3, 4]
        );
        assert_eq!(
            eval_set("BC>", vec![vec![], vec![0, 1, 2], vec![0, 3, 4]]),
            [0, 3, 4]
        );
    }

    #[test]
    fn test_eval_three_sets() {
        let sets = vec![vec![1, 2], vec![2, 3], vec![3, 4]];
        assert_eq!(eval_set("ABC&&", sets.clone()), []);
        assert_eq!(eval_set("ABC||", sets.clone()), [1, 2, 3, 4]);
        assert_eq!(eval_set("ABC|&", sets.clone()), [2]);
        assert_eq!(eval_set("ABC&|", sets.clone()), [1, 2, 3]);
        assert_eq!(eval_set("ABC^^", sets.clone()), [1, 4]);
        assert_eq!(eval_set("ABC==", sets.clone()), [1, 4]);
        assert_eq!(eval_set("ABC=^", sets.clone()), [2, 3]);
        assert_eq!(eval_set("ABC^=", sets.clone()), [2, 3]);
        assert_eq!(eval_set("ABC>>", sets.clone()), [1, 3, 4]);
    }

    #[test]
    #[should_panic]
    fn test_eval_set_empty() {
        eval_set("", vec![]);
    }

    #[test]
    #[should_panic]
    fn test_eval_set_empty_not() {
        eval_set("!A", vec![vec![]]);
    }

    #[test]
    #[should_panic]
    fn test_eval_set_invalid_character() {
        eval_set("01&", vec![]);
    }

    #[test]
    #[should_panic]
    fn test_eval_set_too_many_operations() {
        eval_set("AB||", vec![vec![1, 2], vec![3, 4]]);
    }

    #[test]
    #[should_panic]
    fn test_eval_set_not_enough_operations() {
        eval_set("AAA|", vec![vec![42]]);
    }

    #[test]
    #[should_panic]
    fn test_eval_set_not_enough_sets() {
        eval_set("ABC&&", vec![vec![1, 2, 3], vec![3, 4, 5]]);
    }
}
