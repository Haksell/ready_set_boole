use {itertools::Itertools, ready_set_boole::adder};

fn main() {
    let args = std::env::args().skip(1).collect_vec();
    assert!(
        args.len() >= 1,
        "Usage: cargo run -q --bin ex00 n1 n2 n3 ..."
    );
    let nums: Vec<u32> = args
        .iter()
        .map(|arg| {
            arg.parse()
                .expect(&format!("failed to parse \"{}\" as u32", arg))
        })
        .collect();

    let lhs = nums.iter().map(|n| n.to_string()).join(" + ");
    let rhs = nums.iter().fold(0, |sum, &n| adder(sum, n));
    println!("{lhs} = {rhs}");
}
