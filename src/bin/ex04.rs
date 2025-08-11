use ready_set_boole::print_truth_table;

fn main() {
    let formula = std::env::args()
        .nth(1)
        .expect("Usage: cargo run -q --bin ex04 <formula>");
    print_truth_table(&formula);
}
