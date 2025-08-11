use ready_set_boole::negation_normal_form;

fn main() {
    let formula = std::env::args()
        .nth(1)
        .expect("Usage: cargo run -q --bin ex05 <formula>");
    println!("{}", negation_normal_form(&formula));
}
