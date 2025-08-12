use {itertools::Itertools, ready_set_boole::powerset};

fn main() {
    let args = std::env::args().skip(1).collect_vec();
    let nums: Vec<i32> = args
        .iter()
        .map(|arg| {
            arg.parse()
                .expect(&format!("failed to parse \"{}\" as i32", arg))
        })
        .collect();

    for subset in powerset(nums) {
        println!("{:?}", subset);
    }
}
