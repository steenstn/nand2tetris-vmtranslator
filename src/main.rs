use crate::commands::push;
use std::fs::read_to_string;

mod commands;

fn main() {
    let path = "/home/steen/nand2tetris/projects/07/MemoryAccess/StaticTest/StaticTest.vm";
    let filename = "StaticTest";
    let lines = read_lines(&path);

    println!("File content");
    for line in lines {
        let tokens: Vec<&str> = line.split_whitespace().collect::<Vec<&str>>();
        if tokens.is_empty() {
            continue;
        }
        let result = match tokens[0] {
            "//" => continue,
            "push" => push(tokens[1], tokens[2].parse().unwrap(), filename.to_string()),
            "pop" => "pop".to_string(),
            _ => "Something else".to_string(),
        };
        println!("{result}");
    }
}
fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}
