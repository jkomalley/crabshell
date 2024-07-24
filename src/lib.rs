use std::process;

pub mod prompt;
pub mod input;

pub fn exit() {
    // cleanup
    // TODO

    // display exit message
    println!(":: Terminated crabshell ::");

    // exit crabshell
    process::exit(0);
}
