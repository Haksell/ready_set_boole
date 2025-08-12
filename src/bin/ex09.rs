use {itertools::Itertools as _, ready_set_boole::eval_set};

fn main() {
    let mut args = std::env::args().skip(1);
    let formula = args
        .next()
        .expect("Usage: cargo run -q --bin ex10 <formula> \"<a1 a2 ...>\" \"<b1 b2 ...>\" ...");
    let sets: Vec<Vec<i32>> = args
        .map(|set| {
            set.split_whitespace()
                .map(|x| {
                    x.parse::<i32>()
                        .expect(&format!("failed to parse \"{}\" as i32", x))
                })
                .collect()
        })
        .collect();
    let res = eval_set(&formula, sets);
    println!("{}", res.iter().join(" "));
}
