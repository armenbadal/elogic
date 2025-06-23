use std::env::args;

use crate::{parser::Parser, simulator::Simulator};

mod schematic;
mod parser;
mod simulator;

enum Action {
    Simulate,
    Flatten
}

struct Command {
    action: Action,
    schematic: String,
    file: String,
    data: String,
}

impl Command {
    pub fn new(command_line: &Vec<String>) -> Result<Self, String> {
        if command_line.len() == 7 && command_line[1] == "simulate" {
            Command::simulation_command(&command_line[2..=6])
        } else if command_line.len() == 5 && command_line[1] == "flatten" {
            Command::flattening_command(&command_line[2..=4])
        } else {
            Err("Invalid command line.".into())
        }
    }

    //    0      1       2     3      4     5       6
    // elogic simulate <name> from <fille> with <data-file>
    fn simulation_command(arguments: &[String]) -> Result<Self, String> {
        if arguments[1] == "from" && arguments[3] == "with" {
            let command = Command{
                action: Action::Simulate,
                schematic: arguments[0].clone(),
                file: arguments[2].clone(),
                data: arguments[4].clone(),
            };
            Ok(command)
        } else {
            Err("Invalid command line for simulation.".into())
        }
    }

    //    0      1      2     3     4
    // elogic flatten <name> from <fille>
    fn flattening_command(arguments: &[String]) -> Result<Self, String> {
        if arguments[1] == "from" {
            let command = Command {
                action: Action::Flatten,
                schematic: arguments[0].clone(),
                file: arguments[2].clone(),
                data: String::new(),
            };
            Ok(command)
        } else {
            Err("Invalid command line for flattening.".into())
        }
    }

    pub fn run(&self) {
        let mut parser = Parser::new(&self.file);
        match parser.parse() {
            Ok(design) => {
                println!("===>\n{}", design);
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
    //let cl = args().collect::<Vec<String>>();
    let cl = vec![
        "elogic".to_string(),
        "flatten".to_string(),
        "xor".to_string(),
        "from".to_string(),
        "schematics/example01.elogic".to_string(),
    ];
    let command = Command::new(&cl);
    match command {
        Ok(com) => com.run(),
        Err(message) => eprintln!("ERROR: {}", message),
    }
}
