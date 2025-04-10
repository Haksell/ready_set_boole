use rusty_set_boole::gray_code;

fn main() {
    for i in 0..21 {
        let gc = gray_code(i);
        println!("{:2} | {:2} | {:05b}", i, gc, gc);
    }
}
