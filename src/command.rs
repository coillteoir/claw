#[derive(Debug)]
pub struct Command {
    name: String,
    run: fn(args: Vec<String>),
}

pub fn init_command(name: String, run: fn(args: Vec<String>)) -> Command {
    Command { name, run }
}

pub fn run_command(command: Command) {
    (command.run)(vec![])
}
