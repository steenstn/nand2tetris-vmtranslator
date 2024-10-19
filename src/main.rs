use crate::arithmetic_commands::*;
use crate::memory_commands::*;
use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;
use std::path::Path;

mod arithmetic_commands;
mod memory_commands;

fn main() {
    let path = "/Users/alexandersteen/nand2tetris/projects/7/MemoryAccess/PointerTest/PointerTest";
    let filename = "PointerTest";
    let input_file = format!("{}.vm", path);
    let output_file = format!("{}.asm", path);

    let lines = read_lines(&input_file);
    let mut output_vector: Vec<String> = Vec::new();

    let mut branch_counter = 0;

    println!("File content");
    for line in lines {
        let tokens: Vec<&str> = line.split_whitespace().collect::<Vec<&str>>();
        if tokens.is_empty() {
            continue;
        }

        let result = match tokens[0] {
            "//" => continue,
            "push" => push(tokens[1], tokens[2].parse().unwrap(), filename.to_string()),
            "pop" => pop(tokens[1], tokens[2].parse().unwrap(), filename.to_string()),
            "eq" => eq(&mut branch_counter).to_string(),
            "gt" => gt(&mut branch_counter).to_string(),
            "lt" => lt(&mut branch_counter).to_string(),
            "not" => not().to_string(),
            "or" => or().to_string(),
            "sub" => sub().to_string(),
            "add" => add().to_string(),
            "and" => and().to_string(),
            "neg" => neg().to_string(),
            _ => "Something else".to_string(),
        };

        output_vector.push(result);
    }

    let output_path = Path::new(&output_file);
    let mut file = match File::create(&output_path) {
        Err(why) => panic!("could not create path: {}", why),
        Ok(file) => file,
    };

    for a in output_vector {
        let _ = file.write_all(a.as_bytes());
        println!("{a}");
    }
}
fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
