use std::iter::zip;

use crate::BooleanTree;

fn compute_truth_table(formula: &str) -> (Vec<char>, Vec<Vec<bool>>, Vec<bool>) {
    match BooleanTree::new(formula, true) {
        Ok(bt) => bt.compute_truth_table(),
        Err(err) => panic!("Failed to compute truth table for \"{formula}\": {err}"),
    }
}

pub fn print_truth_table(formula: &str) {
    let (variables, inputs, outputs) = compute_truth_table(formula);
    println!(
        "| {} | = |",
        variables
            .iter()
            .map(char::to_string)
            .collect::<Vec<String>>()
            .join(" | ")
    );
    println!("{}", ["|"].repeat(variables.len() + 2).join("---"));
    for (input, output) in zip(inputs.iter(), outputs.iter()) {
        println!(
            "| {} | {} |",
            input
                .iter()
                .map(|&b| (b as isize).to_string())
                .collect::<Vec<String>>()
                .join(" | "),
            *output as isize
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_truth_table() {
        assert_eq!(
            compute_truth_table("0"),
            (vec![], vec![vec![]], vec![false])
        );
        assert_eq!(
            compute_truth_table("01|1&"),
            (vec![], vec![vec![]], vec![true])
        );
        assert_eq!(
            compute_truth_table("Z!"),
            (vec!['Z'], vec![vec![false], vec![true]], vec![true, false])
        );
        assert_eq!(
            compute_truth_table("AB&C|"),
            (
                vec!['A', 'B', 'C'],
                vec![
                    vec![false, false, false],
                    vec![false, false, true],
                    vec![false, true, false],
                    vec![false, true, true],
                    vec![true, false, false],
                    vec![true, false, true],
                    vec![true, true, false],
                    vec![true, true, true],
                ],
                vec![false, true, false, true, false, true, true, true]
            )
        );
    }
}
