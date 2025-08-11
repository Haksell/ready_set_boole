use itertools::Itertools;

fn main() {
    let (x, y) = std::env::args()
        .skip(1)
        .map(|arg| {
            arg.parse::<u16>()
                .expect(&format!("failed to parse \"{}\" as u16", arg))
        })
        .collect_tuple()
        .expect("Usage: cargo run -q --bin ex10 x y");

    println!("x={x} y={y}");
}
