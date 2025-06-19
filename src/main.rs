use std::env::args;

use crate::{parser::Parser, simulator::Simulator};

mod schematic;
mod parser;
mod simulator;

enum Action {
    Simulate
}

struct Command {
    action: Action,
    schematic: String,
    file: String,
    data: String,
}

impl Command {
    fn new(command_line: &Vec<String>) -> Result<Self, String> {
        if command_line.len() == 7 && command_line[1] == "simulate" {
            //    0      1       2     3      4     5       6
            // elogic simulate <name> from <fille> with <data-file>
            if command_line[3] == "from" && command_line[5] == "with" {
                let command = Command{
                    action: Action::Simulate,
                    schematic: command_line[2].clone(),
                    file: command_line[4].clone(),
                    data: command_line[6].clone(),
                };
                Ok(command)
            } else {
                Err("Invalid command line for simulation.".into())
            }
        } else if command_line.len() == 5 && command_line[1] == "flatten" {
            //    0      1      2     3     4
            // elogic flatten <name> from <fille>
            if command_line[3] == "from" {
                //
            }
            Err("Invalid command line for flattening.".into())
        } else {
            Err("Invalid command line.".into())
        }
    }

    fn run(&self) {
        let mut parser = Parser::new(&self.file);
        match parser.parse() {
            Ok(design) => {
                let simulator = Simulator::new(design);
                simulator.simulate(&self.schematic);
            }
            Err(message) => {
                eprintln!("Parse error: {}", message)
            }
        }
    }
}



fn main() {
    let cl = args().collect::<Vec<String>>();
    let command = Command::new(&cl);
    match command {
        Ok(com) => com.run(),
        Err(message) => eprintln!("ERROR: {}", message),
    }
}
