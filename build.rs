#![allow(dead_code)]
#[path = "./src/args/cli.rs"]
mod cmd;
#[path = "./src/error.rs"]
mod error;
#[path = "./src/formats/mod.rs"]
mod formats;
#[path = "./src/args/internal.rs"]
mod internal;
#[path = "./src/utils.rs"]
mod utils;

#[cfg(not(debug_assertions))]
fn main() {}

#[cfg(debug_assertions)]
fn main() {
    use clap::CommandFactory;

    let cmd = cmd::CliArgs::command();
    let cmd_name = cmd.get_name();
    let path = &format!("./man/man/{cmd_name}.1");
    debug_assert!(
            std::path::Path::new(path).exists(),
            "The man page for the top level command '{cmd_name}' doesn't exists. Create a man page giving an overview of this tool. The man page should be stored at '{path}'");

    let subcmds = cmd.get_subcommands().collect::<Vec<_>>();
    for cmd in subcmds {
        let cmd_name = cmd.get_name();
        let path = &format!("./man/man/archwik_rs-{cmd_name}.1");
        debug_assert!(
            std::path::Path::new(path).exists(),
            "The man page for the command with the name '{cmd_name}' doesn't exists. Create a man page explaining what this command does and how to use it. The man page should be stored at '{path}'");
    }
}
