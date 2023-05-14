use std::io;
mod command_history;

use command_history::CommandHistory;
use std::io::{BufRead, BufReader, Read, Write};
use std::process;
fn main() {
    let mut history = CommandHistory::new(5);
    loop {
        print!("> ");
        if let Err(e) = io::stdout().flush() {
            eprintln!("Error flushing stdout: {}", e);
            process::exit(1);
        }

        match read_input(io::stdin()) {
            Ok(input) => {
                let trimed_input = input.trim();
                // `exit` to quit
                if trimed_input == "exit" {
                    break;
                }
                // if input is `history`, print history
                if trimed_input == "history" {
                    for command in history.get_history() {
                        println!("{}", command);
                    }
                    continue;
                }
                history.add(trimed_input.to_string());
                if let Err(e) = echo_input(&mut io::stdout(), &input) {
                    eprintln!("Error writing to stdout: {}", e);
                    process::exit(1);
                }
            }
            Err(e) => {
                eprintln!("error reading input: {}", e);
                process::exit(1);
            }
        }
    }
}

fn read_input<R: Read>(reader: R) -> io::Result<String> {
    let mut buffer = BufReader::new(reader);
    let mut input = String::new();
    buffer.read_line(&mut input)?;
    Ok(input)
}

fn echo_input<W: Write>(writer: &mut W, input: &str) -> io::Result<()> {
    write!(writer, "{}", input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_input() {
        let input = "Hello, world!\n";
        let result = read_input(input.as_bytes()).unwrap();
        assert_eq!(result, "Hello, world!\n");
    }

    #[test]
    fn test_echo_input() {
        let input = "Hello, world!";
        let mut output = Vec::new();
        echo_input(&mut output, input).unwrap();
        assert_eq!(String::from_utf8(output).unwrap(), "Hello, world!");
    }
}
