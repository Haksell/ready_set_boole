use {
    itertools::Itertools,
    std::{
        collections::{HashMap, HashSet},
        sync::LazyLock,
    },
};

pub type BinaryNode = fn(Box<BooleanTree>, Box<BooleanTree>) -> BooleanTree;

// TODO: ExpressionTree<T>
// where T is bool or HashSet<...>

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
        static BINARY_NODES: LazyLock<HashMap<char, BinaryNode>> = LazyLock::new(|| {
            HashMap::from([
                ('|', BooleanTree::Or as BinaryNode),
                ('&', BooleanTree::And as BinaryNode),
                ('^', BooleanTree::Xor as BinaryNode),
                ('>', BooleanTree::Implication as BinaryNode),
                ('=', BooleanTree::Equivalence as BinaryNode),
            ])
        });

        let mut stack = vec![];
        for c in formula.chars() {
            if c == '0' {
                stack.push(BooleanTree::Value(false));
            } else if c == '1' {
                stack.push(BooleanTree::Value(true));
            } else if is_algebraic && c.is_ascii_uppercase() {
                stack.push(BooleanTree::Variable(c));
            } else if c == '!' {
                if stack.is_empty() {
                    return Err("no operand for binary not");
                }
                let opposite = stack.pop().unwrap();
                stack.push(BooleanTree::Not(Box::new(opposite)));
            } else if let Some(binary_node) = BINARY_NODES.get(&c) {
                if stack.len() < 2 {
                    return Err("not enough operands for binary operation");
                }
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                stack.push(binary_node(Box::new(b), Box::new(a)));
            } else {
                return Err("invalid character");
            }
        }

        match stack.len() {
            0 => Err("empty formula"),
            1 => Ok(stack.pop().unwrap()),
            _ => Err("not enough operators"),
        }
    }

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
        inputs: &mut Vec<Vec<bool>>,
        outputs: &mut Vec<bool>,
    ) {
        if values.len() == variables.len() {
            let line: Vec<bool> = variables.iter().map(|c| values[c]).collect();
            inputs.push(line);
            outputs.push(self.evaluate_with_variables(values));
            return;
        }
        let variable = variables[values.len()];
        values.insert(variable, false);
        self.fill_truth_table(variables, values, inputs, outputs);
        values.insert(variable, true);
        self.fill_truth_table(variables, values, inputs, outputs);
        values.remove_entry(&variable);
    }

    pub fn compute_truth_table(&self) -> (Vec<char>, Vec<Vec<bool>>, Vec<bool>) {
        let mut variables = HashSet::new();
        self.get_variables(&mut variables);
        let variables = variables.into_iter().sorted().collect_vec();
        let mut inputs = vec![];
        let mut outputs = vec![];
        self.fill_truth_table(&variables, &mut HashMap::new(), &mut inputs, &mut outputs);
        (variables, inputs, outputs)
    }

    pub fn is_nnf(&self) -> bool {
        match self {
            BooleanTree::Value(_) | BooleanTree::Variable(_) => true,
            BooleanTree::Not(node) => {
                matches!(**node, BooleanTree::Value(_) | BooleanTree::Variable(_))
            }
            BooleanTree::Or(node1, node2) | BooleanTree::And(node1, node2) => {
                node1.is_nnf() && node2.is_nnf()
            }
            BooleanTree::Xor(..) | BooleanTree::Implication(..) | BooleanTree::Equivalence(..) => {
                false
            }
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
                // store in variables to avoid short-circuiting
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

    fn is_cnf_term(&self) -> bool {
        match self {
            BooleanTree::Value(_) | BooleanTree::Variable(_) => true,
            BooleanTree::Not(node) => {
                matches!(**node, BooleanTree::Value(_) | BooleanTree::Variable(_))
            }
            BooleanTree::Or(node1, node2) => node1.is_cnf_term() && node2.is_cnf_term(),
            _ => false,
        }
    }

    pub fn is_cnf(&self) -> bool {
        match self {
            BooleanTree::Value(_) | BooleanTree::Variable(_) => true,
            BooleanTree::Not(node) => {
                matches!(**node, BooleanTree::Value(_) | BooleanTree::Variable(_))
            }
            BooleanTree::Or(node1, node2) => node1.is_cnf_term() && node2.is_cnf_term(),
            BooleanTree::And(node1, node2) => node1.is_cnf() && node2.is_cnf(),
            _ => false,
        }
    }

    // Assumes the tree is already in NNF
    fn apply_distributivity(&mut self) -> bool {
        match self {
            BooleanTree::Value(_) | BooleanTree::Variable(_) | BooleanTree::Not(_) => false,
            BooleanTree::And(child1, child2) => {
                // store in variables to avoid short-circuiting
                let b1 = child1.apply_distributivity();
                let b2 = child2.apply_distributivity();
                b1 || b2
            }
            BooleanTree::Or(child1, child2) => {
                if let BooleanTree::And(grandchild1, grandchild2) = child1.as_ref() {
                    let mut new_child1 = BooleanTree::Or(grandchild1.clone(), child2.clone());
                    new_child1.apply_distributivity();
                    let mut new_child2 = BooleanTree::Or(grandchild2.clone(), child2.clone());
                    new_child2.apply_distributivity();
                    *self = BooleanTree::And(Box::new(new_child1), Box::new(new_child2));
                    true
                } else if let BooleanTree::And(grandchild1, grandchild2) = child2.as_ref() {
                    let mut new_child1 = BooleanTree::Or(grandchild1.clone(), child1.clone());
                    new_child1.apply_distributivity();
                    let mut new_child2 = BooleanTree::Or(grandchild2.clone(), child1.clone());
                    new_child2.apply_distributivity();
                    *self = BooleanTree::And(Box::new(new_child1), Box::new(new_child2));
                    true
                } else {
                    // store in variables to avoid short-circuiting
                    let b1 = child1.apply_distributivity();
                    let b2 = child2.apply_distributivity();
                    b1 || b2
                }
            }
            _ => unreachable!(),
        }
    }

    pub fn make_cnf(&mut self) {
        self.make_nnf();
        while self.apply_distributivity() {}
    }

    pub fn is_satisfiable(&self) {}
}

// TODO: avoid clones in make_{...}

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
        fn check_nnf(formula: &str) {
            let mut bt = BooleanTree::new(formula, true).unwrap();
            let initial_formula = bt.to_formula();
            let truth_table_before = bt.compute_truth_table();
            bt.make_nnf();
            assert!(
                bt.is_nnf(),
                "{:?}.to_nnf() = {:?} is not in negation normal form",
                initial_formula,
                bt.to_formula()
            );
            let truth_table_after = bt.compute_truth_table();
            assert!(
                truth_table_before == truth_table_after,
                "{:?} (before) and {:?} (after) do not have the same truth table",
                initial_formula,
                bt.to_formula()
            );
        }

        check_nnf("A");
        check_nnf("A!");
        check_nnf("A!!");
        check_nnf("A!!!");
        check_nnf("A!!!!");
        check_nnf("A!!!!!");
        check_nnf("A!!!!!!");
        check_nnf("AB>");
        check_nnf("A!B|");
        check_nnf("AB=");
        check_nnf("AB&A!B!&|");
        check_nnf("AB|!");
        check_nnf("A!B!&");
        check_nnf("AB&!");
        check_nnf("A!B!|");
        check_nnf("AB|C&!");
        check_nnf("A!B!&C!|");
        check_nnf("AB|C&!D!&");
        check_nnf("ABCDE>>>>");
        check_nnf("ABCDE====");
        check_nnf("ABCDE^^^^");
        check_nnf("A!B!!C!!!D!!!!E!!!!!>>>>");
        check_nnf("A!B!!C!!!D!!!!E!!!!!====");
        check_nnf("A!B!!C!!!D!!!!E!!!!!^^^^");
    }

    #[test]
    fn test_is_cnf() {
        assert!(!BooleanTree::new("ABCD&|&", true).unwrap().is_cnf());
        assert!(BooleanTree::new("ABC|BD|&&", true).unwrap().is_cnf());

        assert!(!BooleanTree::new("AB&!", true).unwrap().is_cnf());
        assert!(BooleanTree::new("A!B!|", true).unwrap().is_cnf());

        assert!(!BooleanTree::new("AB|!", true).unwrap().is_cnf());
        assert!(BooleanTree::new("A!B!&", true).unwrap().is_cnf());

        assert!(BooleanTree::new("AB|C&", true).unwrap().is_cnf());

        assert!(BooleanTree::new("AB|C|D|", true).unwrap().is_cnf());
        assert!(BooleanTree::new("ABCD|||", true).unwrap().is_cnf());

        assert!(BooleanTree::new("AB&C&D&", true).unwrap().is_cnf());
        assert!(BooleanTree::new("ABCD&&&", true).unwrap().is_cnf());

        assert!(!BooleanTree::new("AB&!C!|", true).unwrap().is_cnf());
        assert!(BooleanTree::new("A!B!C!||", true).unwrap().is_cnf());

        assert!(!BooleanTree::new("AB|!C!&", true).unwrap().is_cnf());
        assert!(BooleanTree::new("A!B!C!&&", true).unwrap().is_cnf());

        assert!(BooleanTree::new("ABC||DEF||&", true).unwrap().is_cnf());
        assert!(!BooleanTree::new("ABC&&DEF&&|", true).unwrap().is_cnf());
    }

    #[test]
    fn test_make_cnf() {
        fn check_cnf(formula: &str) {
            let mut bt = BooleanTree::new(formula, true).unwrap();
            let initial_formula = bt.to_formula();
            let truth_table_before = bt.compute_truth_table();
            bt.make_cnf();
            assert!(
                bt.is_cnf(),
                "{:?}.to_cnf() = {:?} is not in conjunctive normal form",
                initial_formula,
                bt.to_formula()
            );
            let truth_table_after = bt.compute_truth_table();
            assert!(
                truth_table_before == truth_table_after,
                "{:?} (before) and {:?} (after) do not have the same truth table",
                initial_formula,
                bt.to_formula()
            );
        }

        check_cnf("A");
        check_cnf("A!");
        check_cnf("A!!");
        check_cnf("A!!!");
        check_cnf("A!!!!");
        check_cnf("A!!!!!");
        check_cnf("A!!!!!!");
        check_cnf("AB>");
        check_cnf("A!B|");
        check_cnf("AB=");
        check_cnf("AB&A!B!&|");
        check_cnf("AB|!");
        check_cnf("A!B!&");
        check_cnf("AB&!");
        check_cnf("A!B!|");
        check_cnf("AB|C&!");
        check_cnf("A!B!&C!|");
        check_cnf("AB|C&!D!&");
        check_cnf("ABCDE>>>>");
        check_cnf("ABCDE====");
        check_cnf("ABCDE^^^^");
        check_cnf("AB&CD&|EF&GH&||");
        check_cnf("ABCDE&&&|");
        check_cnf("A!B!!C!!!D!!!!E!!!!!>>>>");
        check_cnf("A!B!!C!!!D!!!!E!!!!!====");
        check_cnf("A!B!!C!!!D!!!!E!!!!!^^^^");
        check_cnf("AB&CD&|");
        check_cnf("AC>BCD&&!&");
    }
}
