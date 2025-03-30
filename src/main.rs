use crate::{arithmetic_commands::*, branching_commands::*, memory_commands::*};
use regex::Regex;
use std::env;
use std::fs;
use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::exit;

mod arithmetic_commands;
mod branching_commands;
mod memory_commands;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);

    let mut files_to_parse: Vec<String> = Vec::new();

    if path.is_dir() {
        match fs::read_dir(path) {
            Ok(directory) => {
                for dir in directory {
                    match dir {
                        Ok(file) => {
                            let extension =
                                String::from(file.path().extension().unwrap().to_str().unwrap());

                            if extension == "vm" {
                                files_to_parse.push(String::from(file.path().to_str().unwrap()));
                            }
                        }
                        Err(why) => panic!("{}", why),
                    }
                }
            }
            Err(why) => {
                println!("Failed to read directory: {why}")
            }
        };
    } else if path.is_file() {
        files_to_parse.push(path.to_str().unwrap().to_string());
    } else {
        println!("Provided argument is not a .vm file or directory");
        exit(0);
    };

    for f in files_to_parse {
        println!("Parsing {}", f.to_string())
    }

    let output_file = path.to_str().unwrap().replace(".vm", ".asm");
    println!("Writing file: {output_file}");

    let filename_regex = match Regex::new(r"([\w ]*)\.vm") {
        Ok(regex) => regex,
        Err(why) => panic!("Could not create regex: {}", why),
    };

    let static_filename = filename_regex
        .find(&path.to_str().unwrap())
        .unwrap()
        .as_str();
    println!("Static filename: {static_filename}");

    let lines = read_lines(&path.to_str().unwrap());
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
            // call
            // function
            // return
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
