use std::process;

pub mod input;
pub mod prompt;

pub fn exit() {
    // cleanup
    // TODO

    // display exit message
    println!(":: Terminated crabshell ::");

    // exit crabshell
    process::exit(0);
}
