use std::io::{self, Write};
use std::env;
use std::process::Command;

fn main() {
    // Init shell prompt.
    // If PS1 doesnt exist in the env
    // we can just use the default prompt
    let shell_prompt = match env::var("PS1") {
        Ok(val) => val,
        Err(_) => {
            String::from("nash > ")
        },
    };
    
    // https://doc.rust-lang.org/std/io/struct.Stdin.html#examples
    // The above could be useful for reading from stdin as in from a pipe later on

    loop {
        // Present to the user
        print!("{}", shell_prompt);
        match io::stdout().flush() {
            Ok(()) => {},
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1)
            }
        }

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                // TODO: When adding evaluation,
                //       this needs to be much
                //       more generalized
                if input == "exit\n" {
                    std::process::exit(0)
                }
                print!("{}", input)
            }
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1)
            },
        }
    }
}

fn execute_cmd(cmd_str: String) -> i32 {
    let cmd = Command::new(cmd_str);
    /*
    https://doc.rust-lang.org/std/process/struct.Command.html
    */
}