use std::fs::File;

use std::io;
use std::io::prelude::*;

use std::process::Command;

fn main() -> std::io::Result<()> {
    let mut input = String::new();

    loop {
        io::stdin().read_line(&mut input)?;

        input.pop();
        input += ";";

        let last_line = input
            .split('\n')
            .last()
            .expect("No line in input! Somehow!");

        input = input
            .split('\n')
            .map(|l| {
                if l == last_line
                /* || input.split('\n').count() == 1 */
                {
                    format!("    {}", l)
                } else {
                    l.to_string()
                }
            })
            .collect::<Vec<String>>()
            .join("\n")
            + "\n";

        println!("fn main() {{\n{}}}", input);

        let mut file = File::create("rustepl.rs")?;
        file.write_all(format!("fn main() {{\n{}}}\n", input).as_bytes())
            .expect("failed to write to temp rustepl file!");

        Command::new("rustc")
            .arg("rustepl.rs")
            .output()
            .expect("failed to run rustc");

        let output = String::from_utf8(
            Command::new("./rustepl")
                .output()
                .expect("failed to run compiled program")
                .stdout,
        )
        .expect("could not convert program output to string");

        println!("Output:\n{}", output);
    }
}
