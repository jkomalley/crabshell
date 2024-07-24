use std::{
    io::{stdout, Write},
    process,
};

pub struct Prompt {
    end_symbol: String,
}

impl Prompt {
    pub fn new(end_symbol: &str) -> Prompt {
        Prompt {
            end_symbol: end_symbol.to_owned(),
        }
    }

    pub fn show(&self) {
        // print prompt
        print!("{}", self.end_symbol);

        // flush output to show prompt
        if let Err(e) = stdout().flush() {
            eprintln!("Error: {}", e);
            process::exit(
                e.raw_os_error()
                    .expect("Could not retrieve raw error code."),
            );
        };
    }
}
