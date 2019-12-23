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
                // NOTE: Should builtins just be separate
                //       branches like this?
                let trimmed_input = String::from(input.trim_end().trim_start());
                if trimmed_input == "exit" {
                    std::process::exit(0);
                } else {
                    let exit_code = execute_cmd(trimmed_input);
                    if exit_code != 0 {
                        println!("Error: {}", exit_code);
                    }
                }
            }
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1)
            },
        }
    }
}

// Return exit code, -8 if an error occurs
// or -7 if process signaled to stop
// If full_cmd_str is "ls -l dir"
fn execute_cmd(full_cmd_str: String) -> i32 {
    //println!("Full CMD: {}", full_cmd_str);
    // arg_arr = [ls, -l, dir]
    let arg_arr: Vec<&str> = full_cmd_str.splitn(2, ' ').collect();
    //println!("Argument Array: {:?}", arg_arr);
    // cmd_str = ls
    let cmd_str: String = if arg_arr.len() > 0 {
        String::from(arg_arr[0])
    } else {
        String::from("")
    };
    //println!("CMD: {}", cmd_str);
    // args = [-l, dir]
    let args = match arg_arr.split_first() {
        Some((_executable, arguments)) => arguments,
        None => &[""]
    };
    //println!("Args: {:?}", args);
    
    // Make command out of the executable
    let mut cmd: Command = Command::new(cmd_str);
    // Provide the args to it
    let cmd_w_args: &mut Command = cmd.args(args);
    let mut cmd_w_env: &mut Command = cmd_w_args;
    // Give the command all the envars that we have
    for (key, value) in env::vars_os() {
        cmd_w_env = cmd_w_env.env(key, value);
    }
    // Execute command and get exit status
    let cmd_status = cmd_w_env.status();
    // Return its error code, -7 if 
    // stopped using signal, -8 if an
    // unknown error occurred
    if !cmd_status.is_err() {
        match cmd_status.unwrap().code() {
            Some(code) => code,
            None => -7,
        }
    } else {
        -8
    }
}