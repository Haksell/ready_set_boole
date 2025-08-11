use {itertools::Itertools, ready_set_boole::reverse_map};

fn main() {
    let args = std::env::args().skip(1).collect_vec();
    assert!(
        args.len() == 1,
        "Usage: cargo run -q --bin ex11 <float in [0, 1]>"
    );
    let z = args[0]
        .parse::<f64>()
        .expect(&format!("failed to parse \"{}\" as u16", args[0]));

    println!("{} -> {:?}", z, reverse_map(z));
}
