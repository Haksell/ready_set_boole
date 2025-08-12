use ready_set_boole::gray_code;

fn main() {
    for i in 0..=42 {
        let gc = gray_code(i);
        println!("{:2} | {:2} | {:06b}", i, gc, gc);
    }
}
