use crate::{arithmetic_commands::*, branching_commands::*, memory_commands::*};
use regex::Regex;
use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;
use std::path::Path;

mod arithmetic_commands;
mod branching_commands;
mod memory_commands;

fn main() {
    //    let path = "/Users/alexandersteen/repos/nand2tetris-vmtranslator/test.vm";
    ////let path = "/Users/alexandersteen/nand2tetris/projects/8/ProgramFlow/BasicLoop/BasicLoop.vm";
    let path = "/Users/alexandersteen/nand2tetris/projects/8/ProgramFlow/FibonacciSeries/FibonacciSeries.vm";

    let output_file = path.replace(".vm", ".asm");
    println!("Writing file: {output_file}");

    let filename_regex = Regex::new(r"([\w ]*)\.vm").unwrap();
    let static_filename = filename_regex.find(&path).unwrap().as_str();

    let lines = read_lines(&path);
    let mut output_vector: Vec<String> = Vec::new();

    let mut branch_counter = 0;

    // label label
    // goto label
    // if-goto label
    /*
        cond = pop
        if cond jumpt o execute command just after label
    */
    println!("File content");
    for line in lines {
        let tokens: Vec<&str> = line.split_whitespace().collect::<Vec<&str>>();
        if tokens.is_empty() {
            continue;
        }

        let result = match tokens[0] {
            "//" => continue,
            "push" => push(
                tokens[1],
                tokens[2].parse().unwrap(),
                static_filename.to_string(),
            ),
            "pop" => pop(
                tokens[1],
                tokens[2].parse().unwrap(),
                static_filename.to_string(),
            ),
            "label" => label(tokens[1]),
            "eq" => eq(&mut branch_counter).to_string(),
            "gt" => gt(&mut branch_counter).to_string(),
            "lt" => lt(&mut branch_counter).to_string(),
            "not" => not().to_string(),
            "or" => or().to_string(),
            "sub" => sub().to_string(),
            "add" => add().to_string(),
            "and" => and().to_string(),
            "neg" => neg().to_string(),
            "goto" => goto(tokens[1]),
            "if-goto" => if_goto(tokens[1]),
            _ => tokens[0].to_string(),
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
