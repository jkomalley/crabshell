use std::{
    io::{stdin, stdout, Write},
    process,
};

fn main() {
    loop {
        // display prompt
        prompt();

        // read input
        let command = match Command::read_command_from_stdin() {
            Some(command) => command,
            None => continue,
        };

        // execute command
        let child = process::Command::new(&command.cmd)
            .args(&command.args)
            .spawn();

        // wait for command to complete
        match child {
            Ok(mut child) => {
                child.wait().unwrap_or_else(|_| {
                    panic!(
                        "Failed while waiting for command \"{}\" to finish",
                        &command.cmd
                    )
                });
            }
            Err(e) => eprintln!("{e}"),
        };
    }
}

fn prompt() {
    // print prompt
    print!("> ");

    // flush output to show prompt
    if let Err(e) = stdout().flush() {
        eprintln!("Error: {}", e);
        process::exit(
            e.raw_os_error()
                .expect("Could not retrieve raw error code."),
        );
    };
}

#[derive(Debug)]
struct Command {
    cmd: String,
    args: Vec<String>,
}

impl Command {
    fn read_command_from_stdin() -> Option<Command> {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut input = input.split_whitespace();

        let cmd = input.next().get_or_insert("").to_owned();

        if cmd.is_empty() {
            None
        } else {
            Some(Command {
                cmd,
                args: input.map(|a| a.to_owned()).collect(),
            })
        }
    }
}
