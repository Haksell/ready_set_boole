use ready_set_boole::print_truth_table;

fn main() {
    print_truth_table(&std::env::args().nth(1).expect(&format!(
        "Usage: cargo run -q --bin {} <formula>",
        std::env::args().nth(0).unwrap()
    )));
}
