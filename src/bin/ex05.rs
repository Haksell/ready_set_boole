use ready_set_boole::negation_normal_form;

fn main() {
    let formula = std::env::args().nth(1).expect(&format!(
        "Usage: cargo run -q --bin {} <formula>",
        std::env::args().nth(0).unwrap()
    ));
    println!("{}", negation_normal_form(&formula));
}
