use crate::{arithmetic_commands::*, branching_commands::*, memory_commands::*};
use std::env;
use std::fs;
use std::fs::read_to_string;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

mod arithmetic_commands;
mod branching_commands;
mod memory_commands;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);

    let files_to_parse = get_files_to_parse(path);

    let output_file = path.with_extension("asm");
    let static_filename = output_file.file_stem().unwrap().to_str().unwrap();

    let output_path = Path::new(&output_file);
    match File::create(&output_path) {
        Err(why) => panic!("could not create path: {}", why),
        Ok(file) => file,
    };

    let mut append_file = match OpenOptions::new().append(true).open(output_path) {
        Ok(file) => file,
        Err(_) => panic!("Could not open file for appending"),
    };

    for current_file in files_to_parse {
        let mut branch_counter = 0;
        let mut function_call_counter = 0;
        println!("Parsing {}", current_file.to_str().unwrap());

        let lines = read_lines(current_file.to_str().unwrap());
        let mut output_vector: Vec<String> = Vec::new();
        let filename = current_file
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .replace(".vm", "");
        println!("Da filename {}", filename);
        let mut current_function_name: String = String::new();

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
                "label" => label(tokens[1], current_function_name.as_str()),
                "eq" => eq(&mut branch_counter).to_string(),
                "gt" => gt(&mut branch_counter).to_string(),
                "lt" => lt(&mut branch_counter).to_string(),
                "not" => not().to_string(),
                "or" => or().to_string(),
                "sub" => sub().to_string(),
                "add" => add().to_string(),
                "and" => and().to_string(),
                "neg" => neg().to_string(),
                "goto" => goto(tokens[1], current_function_name.as_str()),
                "if-goto" => if_goto(tokens[1], current_function_name.as_str()),
                "return" => return_asm(),
                "function" => {
                    current_function_name = tokens[1].to_string();
                    function_asm(tokens[1], tokens[2])
                }
                "call" => call(
                    tokens[1],
                    tokens[2],
                    current_file
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .replace(".vm", "")
                        .as_str(),
                    &mut function_call_counter,
                ),
                _ => tokens[0].to_string(),
            };

            output_vector.push(result);
        }

        for a in output_vector {
            let _ = append_file.write_all(a.as_bytes());
            println!("{a}");
        }
    }
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn get_files_to_parse(path: &Path) -> Vec<PathBuf> {
    let mut files_to_parse: Vec<PathBuf> = Vec::new();
    if path.is_dir() {
        match fs::read_dir(path) {
            Ok(directory) => {
                for dir in directory {
                    match dir {
                        Ok(file) => {
                            let extension =
                                String::from(file.path().extension().unwrap().to_str().unwrap());

                            if extension == "vm" {
                                files_to_parse.push(file.path());
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
        files_to_parse.push(path.to_path_buf());
    } else {
        println!("Provided argument is not a .vm file or directory");
    };
    return files_to_parse;
}
