use ready_set_boole::sat;

fn main() {
    let formula = std::env::args()
        .nth(1)
        .expect("Usage: cargo run -q --bin ex07 <formula>");
    let is_satisfiable = sat(&formula);
    println!(
        "\"{}\" {} satisfiable",
        formula,
        if is_satisfiable { "is" } else { "is not" }
    );
}
