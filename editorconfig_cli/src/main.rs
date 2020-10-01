mod cmd_generate;
mod cmd_help;
mod cmd_version;

use cmd_generate::exec_generate_cmd;
use cmd_help::*;
use cmd_version::*;
use std::env;

#[derive(Copy, Clone)]
enum Cmd {
    Generate,
    Help,
    Version,
}

fn main() {
    let mut args = env::args().skip(1).peekable();

    let cmd = loop {
        let arg = match args.peek() {
            Some(arg) => arg,
            None => break Cmd::Help,
        };

        if string_is_help_flag(&arg) {
            break Cmd::Help;
        }

        if string_is_version_flag(&arg) {
            break Cmd::Version;
        }

        break Cmd::Generate;
    };

    match cmd {
        Cmd::Generate => exec_generate_cmd(args),
        Cmd::Help => exec_help_cmd(),
        Cmd::Version => exec_version_cmd(),
    }
}
