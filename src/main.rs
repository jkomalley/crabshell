use std::process;

use crabshell::{cd, exit, input::Input, prompt::Prompt};

fn main() {
    let prompt = Prompt::new("> ");
    loop {
        // display prompt
        prompt.show().expect("Failed to show prompt");

        // read input
        let line = match Input::from_stdin() {
            Some(line) => line,
            None => continue,
        };

        // execute command
        match line.cmd.as_str() {
            "cd" => cd(line.args),
            "exit" => exit(),
            _ => {
                // execute command
                let child = process::Command::new(&line.cmd).args(&line.args).spawn();

                // wait for command to complete
                match child {
                    Ok(mut child) => {
                        child.wait().unwrap_or_else(|_| {
                            panic!(
                                "Failed while waiting for command \"{}\" to finish",
                                &line.full
                            )
                        });
                    }
                    Err(e) => eprintln!("{e}"),
                };
            }
        }
    }
}
