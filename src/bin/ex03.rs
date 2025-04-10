use rusty_set_boole::eval_formula;

fn main() {
    let formula = std::env::args().nth(1).expect(&format!(
        "Usage: cargo run -q --bin {} <formula>",
        std::env::args().nth(0).unwrap()
    ));
    println!(
        "{} is {}",
        formula,
        if eval_formula(&formula) { '1' } else { '0' }
    );
}
