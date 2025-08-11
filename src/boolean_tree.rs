use {
    itertools::Itertools,
    std::{
        collections::{HashMap, HashSet},
        sync::LazyLock,
    },
};

pub type BinaryNode = fn(Box<BooleanTree>, Box<BooleanTree>) -> BooleanTree;

static BINARY_NODES: LazyLock<HashMap<char, BinaryNode>> = LazyLock::new(|| {
    HashMap::from([
        ('|', BooleanTree::Or as BinaryNode),
        ('&', BooleanTree::And as BinaryNode),
        ('^', BooleanTree::Xor as BinaryNode),
        ('>', BooleanTree::Implication as BinaryNode),
        ('=', BooleanTree::Equivalence as BinaryNode),
    ])
});

#[derive(Clone, Debug)]
pub enum BooleanTree {
    Value(bool),
    Variable(char),
    Not(Box<BooleanTree>),
    Or(Box<BooleanTree>, Box<BooleanTree>),
    And(Box<BooleanTree>, Box<BooleanTree>),
    Xor(Box<BooleanTree>, Box<BooleanTree>),
    Implication(Box<BooleanTree>, Box<BooleanTree>),
    Equivalence(Box<BooleanTree>, Box<BooleanTree>),
}

impl BooleanTree {
    pub fn new(formula: &str, is_algebraic: bool) -> Result<Self, &'static str> {
        let mut stack = vec![];
        for c in formula.chars() {
            if c == '0' {
                stack.push(Box::new(BooleanTree::Value(false)));
            } else if c == '1' {
                stack.push(Box::new(BooleanTree::Value(true)));
            } else if is_algebraic && c.is_ascii_uppercase() {
                stack.push(Box::new(BooleanTree::Variable(c)));
            } else if c == '!' {
                if stack.is_empty() {
                    return Err("no operand for binary not");
                }
                let opposite = stack.pop().unwrap();
                stack.push(Box::new(BooleanTree::Not(opposite)));
            } else if let Some(binary_node) = BINARY_NODES.get(&c) {
                if stack.len() < 2 {
                    return Err("not enough operands for binary operation");
                }
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(Box::new(binary_node(b, a)));
            } else {
                return Err("invalid character");
            }
        }
        match stack.len() {
            0 => Err("empty formula"),
            1 => Ok(*stack[0].clone()),
            _ => Err("not enough operators"),
        }
    }

    // TODO: hashmap of characters
    pub fn to_formula(&self) -> String {
        match self {
            BooleanTree::Value(false) => "0".to_string(),
            BooleanTree::Value(true) => "1".to_string(),
            BooleanTree::Variable(c) => c.to_string(),
            BooleanTree::Not(node) => format!("{}!", node.to_formula()),
            BooleanTree::Or(node1, node2) => {
                format!("{}{}|", node1.to_formula(), node2.to_formula())
            }
            BooleanTree::And(node1, node2) => {
                format!("{}{}&", node1.to_formula(), node2.to_formula())
            }
            BooleanTree::Xor(node1, node2) => {
                format!("{}{}^", node1.to_formula(), node2.to_formula())
            }
            BooleanTree::Implication(node1, node2) => {
                format!("{}{}>", node1.to_formula(), node2.to_formula())
            }
            BooleanTree::Equivalence(node1, node2) => {
                format!("{}{}=", node1.to_formula(), node2.to_formula())
            }
        }
    }

    pub fn evaluate(&self) -> bool {
        self.evaluate_with_variables(&HashMap::new())
    }

    fn evaluate_with_variables(&self, values: &HashMap<char, bool>) -> bool {
        match self {
            BooleanTree::Value(b) => *b,
            BooleanTree::Variable(c) => *values
                .get(c)
                .unwrap_or_else(|| panic!("no value found for variable '{c}'")),
            BooleanTree::Not(node) => !node.evaluate_with_variables(values),
            BooleanTree::Or(node1, node2) => {
                node1.evaluate_with_variables(values) || node2.evaluate_with_variables(values)
            }
            BooleanTree::And(node1, node2) => {
                node1.evaluate_with_variables(values) && node2.evaluate_with_variables(values)
            }
            BooleanTree::Xor(node1, node2) => {
                node1.evaluate_with_variables(values) ^ node2.evaluate_with_variables(values)
            }
            BooleanTree::Implication(node1, node2) => {
                !node1.evaluate_with_variables(values) || node2.evaluate_with_variables(values)
            }
            BooleanTree::Equivalence(node1, node2) => {
                node1.evaluate_with_variables(values) == node2.evaluate_with_variables(values)
            }
        }
    }

    fn get_variables(&self, variables: &mut HashSet<char>) {
        match self {
            BooleanTree::Value(_) => {}
            BooleanTree::Variable(c) => {
                variables.insert(*c);
            }
            BooleanTree::Not(node) => node.get_variables(variables),
            BooleanTree::Or(node1, node2)
            | BooleanTree::And(node1, node2)
            | BooleanTree::Xor(node1, node2)
            | BooleanTree::Implication(node1, node2)
            | BooleanTree::Equivalence(node1, node2) => {
                node1.get_variables(variables);
                node2.get_variables(variables);
            }
        }
    }

    fn fill_truth_table(
        &self,
        variables: &[char],
        values: &mut HashMap<char, bool>,
        truth_table: &mut Vec<Vec<bool>>,
    ) {
        if values.len() == variables.len() {
            let line: Vec<bool> = variables
                .iter()
                .map(|c| values[c])
                .chain(std::iter::once(self.evaluate_with_variables(values)))
                .collect();
            truth_table.push(line);
            return;
        }
        let variable = variables[values.len()];
        values.insert(variable, false);
        self.fill_truth_table(variables, values, truth_table);
        values.insert(variable, true);
        self.fill_truth_table(variables, values, truth_table);
        values.remove_entry(&variable);
    }

    pub fn compute_truth_table(&self) -> (Vec<char>, Vec<Vec<bool>>) {
        let mut variables = HashSet::new();
        self.get_variables(&mut variables);
        let variables = variables.into_iter().sorted().collect_vec();
        let mut truth_table = vec![];
        self.fill_truth_table(&variables, &mut HashMap::new(), &mut truth_table);
        (variables, truth_table)
    }

    pub fn is_nnf(&self) -> bool {
        match self {
            BooleanTree::Value(_) | BooleanTree::Variable(_) => true,
            BooleanTree::Not(node) => match **node {
                BooleanTree::Value(_) | BooleanTree::Variable(_) => true,
                _ => false,
            },
            BooleanTree::Or(node1, node2) | BooleanTree::And(node1, node2) => {
                node1.is_nnf() && node2.is_nnf()
            }
            BooleanTree::Xor(_, _)
            | BooleanTree::Implication(_, _)
            | BooleanTree::Equivalence(_, _) => false,
        }
    }

    fn remove_forbidden_operations(&mut self) {
        match self {
            BooleanTree::Value(_) | BooleanTree::Variable(_) => {}
            BooleanTree::Not(node) => node.remove_forbidden_operations(),
            BooleanTree::Or(node1, node2) | BooleanTree::And(node1, node2) => {
                node1.remove_forbidden_operations();
                node2.remove_forbidden_operations();
            }
            BooleanTree::Xor(node1, node2) => {
                node1.remove_forbidden_operations();
                node2.remove_forbidden_operations();
                *self = BooleanTree::Or(
                    Box::new(BooleanTree::And(
                        node1.clone(),
                        Box::new(BooleanTree::Not(node2.clone())),
                    )),
                    Box::new(BooleanTree::And(
                        Box::new(BooleanTree::Not(node1.clone())),
                        node2.clone(),
                    )),
                );
            }
            BooleanTree::Implication(node1, node2) => {
                node1.remove_forbidden_operations();
                node2.remove_forbidden_operations();
                *self = BooleanTree::Or(Box::new(BooleanTree::Not(node1.clone())), node2.clone());
            }
            BooleanTree::Equivalence(node1, node2) => {
                node1.remove_forbidden_operations();
                node2.remove_forbidden_operations();
                *self = BooleanTree::Or(
                    Box::new(BooleanTree::And(node1.clone(), node2.clone())),
                    Box::new(BooleanTree::And(
                        Box::new(BooleanTree::Not(node1.clone())),
                        Box::new(BooleanTree::Not(node2.clone())),
                    )),
                );
            }
        }
    }

    fn apply_de_morgan(&mut self) -> bool {
        match self {
            BooleanTree::Value(_) | BooleanTree::Variable(_) => false,
            BooleanTree::Not(child) => match *child.clone() {
                BooleanTree::Value(_) | BooleanTree::Variable(_) => false,
                BooleanTree::Not(_) => child.apply_de_morgan(),
                BooleanTree::Or(grandchild1, grandchild2) => {
                    let mut left = BooleanTree::Not(grandchild1);
                    let mut right = BooleanTree::Not(grandchild2);
                    left.apply_de_morgan();
                    right.apply_de_morgan();
                    *self = BooleanTree::And(Box::new(left), Box::new(right));
                    true
                }
                BooleanTree::And(grandchild1, grandchild2) => {
                    let mut left = BooleanTree::Not(grandchild1);
                    let mut right = BooleanTree::Not(grandchild2);
                    left.apply_de_morgan();
                    right.apply_de_morgan();
                    *self = BooleanTree::Or(Box::new(left), Box::new(right));
                    true
                }
                _ => unreachable!(),
            },
            BooleanTree::Or(child1, child2) | BooleanTree::And(child1, child2) => {
                let b1 = child1.apply_de_morgan();
                let b2 = child2.apply_de_morgan();
                b1 || b2
            }
            _ => unreachable!(),
        }
    }

    fn remove_double_negation(&mut self) {
        match self {
            BooleanTree::Value(_) | BooleanTree::Variable(_) => {}
            BooleanTree::Not(child) => match *child.clone() {
                BooleanTree::Value(_) | BooleanTree::Variable(_) => {}
                BooleanTree::Not(grandchild) => {
                    *self = *grandchild.clone();
                    self.remove_double_negation();
                }
                _ => unreachable!(),
            },
            BooleanTree::Or(child1, child2) | BooleanTree::And(child1, child2) => {
                child1.remove_double_negation();
                child2.remove_double_negation();
            }
            _ => unreachable!(),
        }
    }

    pub fn make_nnf(&mut self) {
        self.remove_forbidden_operations();
        while self.apply_de_morgan() {}
        self.remove_double_negation();
    }

    // pub fn _is_cnf(&self, and_allowed: bool) -> bool {
    //     match self {
    //         BooleanTree::Value(_) | BooleanTree::Variable(_) => true,
    //         BooleanTree::Not(node) => match **node {
    //             BooleanTree::Value(_) | BooleanTree::Variable(_) => true,
    //             _ => false,
    //         },
    //         BooleanTree::Or(node1, node2) => node1._is_cnf(false) && node2._is_cnf(false),
    //         BooleanTree::And(node1, node2) => {
    //             if and_allowed {
    //                 node1._is_cnf(true) && node2._is_cnf(true)
    //             } else {
    //                 false
    //             }
    //         }
    //         BooleanTree::Xor(_, _)
    //         | BooleanTree::Implication(_, _)
    //         | BooleanTree::Equivalence(_, _) => false,
    //     }
    // }

    // pub fn is_cnf(&self) -> bool {
    //     self._is_cnf(true)
    // }

    // pub fn to_cnf(&mut self) -> Self {
    //     self.clone()
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: test new, to_formula

    #[test]
    fn test_is_nnf() {
        assert!(!BooleanTree::new("A!!", true).unwrap().is_nnf());
        assert!(BooleanTree::new("A", true).unwrap().is_nnf());

        assert!(!BooleanTree::new("A!!!", true).unwrap().is_nnf());
        assert!(BooleanTree::new("A!", true).unwrap().is_nnf());

        assert!(!BooleanTree::new("AB>", true).unwrap().is_nnf());
        assert!(BooleanTree::new("A!B|", true).unwrap().is_nnf());

        assert!(!BooleanTree::new("AB=", true).unwrap().is_nnf());
        assert!(BooleanTree::new("AB&A!B!&|", true).unwrap().is_nnf());

        assert!(!BooleanTree::new("AB|!", true).unwrap().is_nnf());
        assert!(BooleanTree::new("A!B!&", true).unwrap().is_nnf());

        assert!(!BooleanTree::new("AB&!", true).unwrap().is_nnf());
        assert!(BooleanTree::new("A!B!|", true).unwrap().is_nnf());

        assert!(!BooleanTree::new("AB|C&!", true).unwrap().is_nnf());
        assert!(BooleanTree::new("A!B!&C!|", true).unwrap().is_nnf());
    }

    #[test]
    fn test_make_nnf() {
        fn check_nnf(formula: &str) -> bool {
            let mut bt = BooleanTree::new(formula, true).unwrap();
            let truth_table_before = bt.compute_truth_table();
            bt.make_nnf();
            let truth_table_after = bt.compute_truth_table();
            bt.is_nnf() && truth_table_before == truth_table_after
        }

        assert!(check_nnf("A"));
        assert!(check_nnf("A!"));
        assert!(check_nnf("A!!"));
        assert!(check_nnf("A!!!"));
        assert!(check_nnf("A!!!!"));
        assert!(check_nnf("A!!!!!"));
        assert!(check_nnf("A!!!!!!"));
        assert!(check_nnf("AB>"));
        assert!(check_nnf("A!B|"));
        assert!(check_nnf("AB="));
        assert!(check_nnf("AB&A!B!&|"));
        assert!(check_nnf("AB|!"));
        assert!(check_nnf("A!B!&"));
        assert!(check_nnf("AB&!"));
        assert!(check_nnf("A!B!|"));
        assert!(check_nnf("AB|C&!"));
        assert!(check_nnf("A!B!&C!|"));
        assert!(check_nnf("AB|C&!D!&"));
        assert!(check_nnf("ABCDE>>>>"));
        assert!(check_nnf("ABCDE===="));
        assert!(check_nnf("ABCDE^^^^"));
        assert!(check_nnf("A!B!!C!!!D!!!!E!!!!!>>>>"));
        assert!(check_nnf("A!B!!C!!!D!!!!E!!!!!===="));
        assert!(check_nnf("A!B!!C!!!D!!!!E!!!!!^^^^"));
    }

    // #[test]
    // fn test_is_cnf() {
    //     assert!(!BooleanTree::new("ABCD&|&", true).unwrap().is_cnf());
    //     assert!(BooleanTree::new("ABC|BD|&&", true).unwrap().is_cnf());

    //     assert!(!BooleanTree::new("AB&!", true).unwrap().is_cnf());
    //     assert!(BooleanTree::new("A!B!|", true).unwrap().is_cnf());

    //     assert!(!BooleanTree::new("AB|!", true).unwrap().is_cnf());
    //     assert!(BooleanTree::new("A!B!&", true).unwrap().is_cnf());

    //     assert!(BooleanTree::new("AB|C&", true).unwrap().is_cnf());

    //     assert!(BooleanTree::new("AB|C|D|", true).unwrap().is_cnf());
    //     assert!(BooleanTree::new("ABCD|||", true).unwrap().is_cnf());

    //     assert!(BooleanTree::new("AB&C&D&", true).unwrap().is_cnf());
    //     assert!(BooleanTree::new("ABCD&&&", true).unwrap().is_cnf());

    //     assert!(!BooleanTree::new("AB&!C!|", true).unwrap().is_cnf());
    //     assert!(BooleanTree::new("A!B!C!||", true).unwrap().is_cnf());

    //     assert!(!BooleanTree::new("AB|!C!&", true).unwrap().is_cnf());
    //     assert!(BooleanTree::new("A!B!C!&&", true).unwrap().is_cnf());

    //     assert!(BooleanTree::new("ABC||DEF||&", true).unwrap().is_cnf());
    //     assert!(!BooleanTree::new("ABC&&DEF&&|", true).unwrap().is_cnf());
    // }

    // #[test]
    // fn test_to_cnf() {
    //     fn check_cnf(formula: &str) -> bool {
    //         let mut bt = BooleanTree::new(formula, true).unwrap();
    //         let truth_table_before = bt.compute_truth_table();
    //         let cnf = bt.to_cnf();
    //         println!("{} -> {}", formula, cnf.to_formula());
    //         let truth_table_after = cnf.compute_truth_table();
    //         println!(
    //             "{} {}",
    //             cnf.is_cnf(),
    //             truth_table_before == truth_table_after
    //         );
    //         cnf.is_cnf() && truth_table_before == truth_table_after
    //     }

    //     assert!(check_cnf("A"));
    //     assert!(check_cnf("A!"));
    //     assert!(check_cnf("A!!"));
    //     assert!(check_cnf("A!!!"));
    //     assert!(check_cnf("A!!!!"));
    //     assert!(check_cnf("A!!!!!"));
    //     assert!(check_cnf("A!!!!!!"));
    //     assert!(check_cnf("AB>"));
    //     assert!(check_cnf("A!B|"));
    //     assert!(check_cnf("AB="));
    //     assert!(check_cnf("AB&A!B!&|"));
    //     assert!(check_cnf("AB|!"));
    //     assert!(check_cnf("A!B!&"));
    //     assert!(check_cnf("AB&!"));
    //     assert!(check_cnf("A!B!|"));
    //     assert!(check_cnf("AB|C&!"));
    //     assert!(check_cnf("A!B!&C!|"));
    //     assert!(check_cnf("AB|C&!D!&"));
    //     assert!(check_cnf("ABCDE>>>>"));
    //     assert!(check_cnf("ABCDE===="));
    //     assert!(check_cnf("ABCDE^^^^"));
    //     assert!(check_cnf("A!B!!C!!!D!!!!E!!!!!>>>>"));
    //     assert!(check_cnf("A!B!!C!!!D!!!!E!!!!!===="));
    //     assert!(check_cnf("A!B!!C!!!D!!!!E!!!!!^^^^"));
    //     assert!(check_cnf("AB&CD&|"));
    //     assert!(check_cnf("AC>BCD&&!&"));
    // }
}
