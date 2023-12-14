use std::env;

fn main() {
    let args: Vec<_> = env::args().map(is_posix).collect();
    println!("{:?}", args);
}

fn is_posix(flag: String) -> bool {
    flag[0..2].eq("--")
}
