use rusty_set_boole::eval_formula;

fn main() {
    let formula = std::env::args()
        .nth(1)
        .expect("Usage: cargo run -q --bin ex03 <formula>");
    println!(
        "{} is {}",
        formula,
        if eval_formula(&formula) { '1' } else { '0' }
    );
}
