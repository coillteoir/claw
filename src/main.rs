use std::env;

struct Flag<T> {
    name: String,
    shorthand: String,
    description: String,
    value: T,
    default: T,
}

#[derive(Clone)]
struct Command {
    name: String,
    description: String,
    logic: fn() -> Result<String, String>,
}

impl Command {
    fn run(&self) -> Result<String, String> {
        (self.logic)()
    }
}

fn main() {
    let commands = [
        Command {
            name: String::from("root"),
            description: String::from("this command says hello"),
            logic: || {
                println!("Hello world");
                Ok(String::from("hello hello"))
            },
        },
        Command {
            name: String::from("doink"),
            description: String::from("shakaboink"),
            logic: || {
                println!("skaboink");
                Ok(String::from("hehe"))
            },
        },
    ];

    let args: Vec<String> = env::args().collect();
    parse(args.clone());
    if args.len() == 1 {
        commands[0].run();
        return;
    }

    let _ = commands
        .into_iter()
        .filter(|command: &Command| args[1] == command.name)
        .collect::<Vec<Command>>()[0]
        .run();
}

fn print_help(commands: Vec<Command>) {
    commands.into_iter().for_each(|command| {
        println!(
            "Name: {}\nDescription: {}",
            command.name, command.description
        )
    })
}

fn parse(args: Vec<String>) {
    args.into_iter().for_each(|item| println!("Item: {}", item))
}
