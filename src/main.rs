use std::env;

#[derive(Clone)]
struct Flag {
    name: String,
    shorthand: String,
    description: String,
    value: String,
    default: String,
}

#[derive(Clone)]
struct Command {
    name: String,
    description: String,
    flags: Vec<Flag>,
    logic: fn(Vec<String>) -> Result<String, String>,
}

impl Command {
    fn run(&self, args: Vec<String>) -> Result<String, String> {
        (self.logic)(args)
    }
}

fn main() {
    let commands = [
        Command {
            name: String::from("root"),
            description: String::from("this command says hello"),
            logic: |_| {
                println!("Hello world");
                Ok(String::from("hello hello"))
            },
            flags: Vec::<Flag>::new(),
        },
        Command {
            name: String::from("doink"),
            description: String::from("shakaboink"),
            logic: |_| {
                println!("skaboink");
                Ok(String::from("hehe"))
            },
            flags: [Flag {
                name: String::from("init"),
                shorthand: String::from("i"),
                description: String::from("initializes a sandwich"),
                default: String::from("0"),
                value: String::from("0"),
            }]
            .to_vec(),
        },
    ];

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        let _ = commands[0].run(Vec::<String>::new());
        return;
    }

    if args[1] == "help" {
        print_help(commands.to_vec());
        return;
    }

    let to_run = commands
        .into_iter()
        .filter(|command: &Command| args[1] == command.name)
        .collect::<Vec<Command>>();

    if to_run.len() == 1 {
        let _ = to_run[0].run(Vec::<String>::new());
    }
}

fn print_help(commands: Vec<Command>) {
    commands.into_iter().for_each(|command| {
        println!(
            "Command: {} Description: {}",
            command.name, command.description
        );
        command.flags.into_iter().for_each(|flag| {
            println!(
                "Flag: --{} -{} Description: {} Default: {}",
                flag.name, flag.shorthand, flag.description, flag.default
            )
        })
    })
}
