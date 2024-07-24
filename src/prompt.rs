use std::io::{stdout, Result, Write};

pub struct Prompt {
    end_symbol: String,
}

impl Prompt {
    pub fn new(end_symbol: &str) -> Prompt {
        Prompt {
            end_symbol: end_symbol.to_owned(),
        }
    }

    pub fn show(&self) -> Result<()> {
        // print prompt
        print!("{}", self.end_symbol);

        // flush output to show prompt
        stdout().flush()
    }
}
