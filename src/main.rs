use crate::command::*;
//use std::error::Error;

mod command;

/*
 *  This project is a learning experience, it will be poorly architected
 *  and slow most likely :D
 */

fn main() {
    let claw: Command = init_command("Claw".to_string(), |args: Vec<String>| {
        println!("{:?}", args)
    });
    run_command(claw);
}
// could be better to use regex
fn is_posix(flag: &String) -> bool {
    flag[0..2].eq("--") && flag.len() > 2
}
