use rusty_set_boole::multiplier;

fn main() {
    assert!(
        std::env::args().len() >= 2,
        "Usage: cargo run -q --bin ex01 n1 n2 n3 ..."
    );
    let mut res = 1;
    for (i, arg) in std::env::args().skip(1).enumerate() {
        let n = arg.parse().expect("failed to parse u32");
        res = multiplier(res, n);
        print!("{}{} ", if i == 0 { "" } else { "* " }, n);
    }
    println!("= {res}");
}
