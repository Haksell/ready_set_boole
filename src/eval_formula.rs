use crate::BooleanTree;

pub fn eval_formula(formula: &str) -> bool {
    match BooleanTree::new(formula, false) {
        Err(err) => panic!("Failed to evaluate formula \"{formula}\": {err}"),
        Ok(tree) => tree.evaluate(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_formula_unary() {
        assert!(!eval_formula("0"));
        assert!(eval_formula("1"));
        assert!(eval_formula("0!"));
        assert!(!eval_formula("1!"));
    }

    #[test]
    fn test_eval_formula_binary() {
        assert!(!eval_formula("00|"));
        assert!(eval_formula("01|"));
        assert!(eval_formula("10|"));
        assert!(eval_formula("11|"));

        assert!(!eval_formula("00&"));
        assert!(!eval_formula("01&"));
        assert!(!eval_formula("10&"));
        assert!(eval_formula("11&"));

        assert!(!eval_formula("00^"));
        assert!(eval_formula("01^"));
        assert!(eval_formula("10^"));
        assert!(!eval_formula("11^"));

        assert!(eval_formula("00>"));
        assert!(eval_formula("01>"));
        assert!(!eval_formula("10>"));
        assert!(eval_formula("11>"));

        assert!(eval_formula("00="));
        assert!(!eval_formula("01="));
        assert!(!eval_formula("10="));
        assert!(eval_formula("11="));
    }

    #[test]
    fn test_eval_formula_complex() {
        assert!(eval_formula("10|1&"));
        assert!(eval_formula("1011||="));
        assert!(eval_formula("1!1|"));
        assert!(eval_formula("111^^"));
        assert!(!eval_formula("1111^^^"));
        assert!(!eval_formula("000=="));
        assert!(eval_formula("111=="));
    }

    #[test]
    fn test_eval_formula_two_material_implications() {
        assert!(eval_formula("000>>"));
        assert!(eval_formula("001>>"));
        assert!(eval_formula("010>>"));
        assert!(eval_formula("011>>"));
        assert!(eval_formula("100>>"));
        assert!(eval_formula("101>>"));
        assert!(!eval_formula("110>>"));
        assert!(eval_formula("111>>"));
    }

    #[test]
    #[should_panic]
    fn test_eval_formula_empty() {
        eval_formula("");
    }

    #[test]
    #[should_panic]
    fn test_eval_formula_empty_not() {
        eval_formula("!1");
    }

    #[test]
    #[should_panic]
    fn test_eval_formula_invalid_character() {
        eval_formula("42&");
    }

    #[test]
    #[should_panic]
    fn test_eval_formula_too_many_operations() {
        eval_formula("10||");
    }

    #[test]
    #[should_panic]
    fn test_eval_formula_not_enough_operations() {
        eval_formula("111|");
    }
}
