use crate::BooleanTree;

fn compute_truth_table(formula: &str) -> (Vec<char>, Vec<Vec<bool>>) {
    match BooleanTree::new(formula, true) {
        Ok(bt) => bt.compute_truth_table(),
        Err(err) => panic!("Failed to compute truth table for \"{formula}\": {err}"),
    }
}

pub fn print_truth_table(formula: &str) {
    let (variables, truth_table) = compute_truth_table(formula);
    println!(
        "| {}= |",
        variables
            .iter()
            .map(|c| format!("{c} | "))
            .collect::<Vec<String>>()
            .join("")
    );
    println!("{}", ["|"].repeat(variables.len() + 2).join("---"));
    for line in truth_table {
        println!(
            "| {}",
            line.iter()
                .map(|&b| if b { "1 | " } else { "0 | " }.to_string())
                .collect::<Vec<String>>()
                .join(""),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_truth_table() {
        assert_eq!(compute_truth_table("0"), (vec![], vec![vec![false]]));
        assert_eq!(compute_truth_table("01|1&"), (vec![], vec![vec![true]]));
        assert_eq!(
            compute_truth_table("Z!"),
            (vec!['Z'], vec![vec![false, true], vec![true, false]])
        );
        assert_eq!(
            compute_truth_table("AB&C|"),
            (
                vec!['A', 'B', 'C'],
                vec![
                    vec![false, false, false, false],
                    vec![false, false, true, true],
                    vec![false, true, false, false],
                    vec![false, true, true, true],
                    vec![true, false, false, false],
                    vec![true, false, true, true],
                    vec![true, true, false, true],
                    vec![true, true, true, true],
                ]
            )
        );
    }
}
