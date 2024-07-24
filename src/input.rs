use std::io::stdin;

#[derive(Debug)]
pub struct Input {
    pub cmd: String,
    pub args: Vec<String>,
    pub full: String
}

impl Input {
    pub fn from_stdin() -> Option<Input> {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let full = input.clone();

        let mut input = input.split_whitespace();

        let cmd = input.next().get_or_insert("").to_owned();

        if cmd.is_empty() {
            None
        } else {
            Some(Input {
                cmd,
                args: input.map(|a| a.to_owned()).collect(),
                full,
            })
        }
    }
}
