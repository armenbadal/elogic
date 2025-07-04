use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Pin {
    name: String,
    role: Role,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    Input,
    Output,
    Local,
}

impl Pin {
    pub fn new(name: String, role: Role) -> Self {
        Self { name, role }
    }

    pub fn new_input(name: String) -> Self {
        Pin::new(name, Role::Input)
    }

    pub fn new_output(name: String) -> Self {
        Pin::new(name, Role::Output)
    }

    pub fn new_local(name: String) -> Self {
        Pin::new(name, Role::Local)
    }
}

#[derive(Debug, Clone)]
pub struct Instruction {
    schematic_name: String,
    pin_bindings: Vec<String>,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = String::from(&self.schematic_name);
        for p in &self.pin_bindings {
            result.push_str(&format!(" {}", p));
        }
        write!(f, "{}", result)
    }
}

impl Instruction {
    pub fn new(schematic_name: String, inputs: Vec<String>, outputs: Vec<String>) -> Self {
        let mut pin_bindings = Vec::<String>::new();
        pin_bindings.extend(inputs);
        pin_bindings.extend(outputs);
        Instruction { schematic_name, pin_bindings }
    }

    fn expand(&self, library: &Vec<Schematic>, ng: &mut NameGenerator) -> Vec<Instruction> {
        if self.schematic_name == "nand" {
            return vec![self.clone()]
        }

        let scheme = match library.iter().find(|&s| s.name == self.schematic_name) {
            Some(scheme) => scheme,
            None => return vec![],
        };

        let mut pin_map: HashMap<String, String> = HashMap::new();
        for i in 0..scheme.pins.len() {
            let pin = &scheme.pins[i];
            let map_to = if pin.role == Role::Local {
                ng.get_next_name()
            } else {
                self.pin_bindings[i].clone()
            };
            pin_map.insert(pin.name.clone(), map_to);
        }

        let mut result: Vec<Instruction> = vec![];
        for instr in &scheme.body {
            let mut bindings: Vec<String> = vec![];
            for p in &instr.pin_bindings {
                let name = pin_map.get(p).unwrap();
                bindings.push(name.clone());
            }

            let modified = Instruction {
                schematic_name: instr.schematic_name.clone(),
                pin_bindings: bindings
            };
            result.append(&mut modified.expand(&library, ng));
        }
        result
    }
}

struct NameGenerator {
    prefix: String,
    counter: i32,
}

impl NameGenerator {
    fn new(prefix: String) -> Self {
        NameGenerator {  prefix, counter: -1 }
    }

    fn get_next_name(&mut self) -> String {
        self.counter += 1;
        format!("{}.{}", self.prefix, self.counter)
    }
}

#[derive(Debug, Clone)]
pub struct Schematic {
    name: String,
    pins: Vec<Pin>,
    body: Vec<Instruction>,
}

impl Display for Schematic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let format_pins = |f: &mut Formatter<'_>, ps: &Vec<Pin>, r: Role|
                ps.iter()
                  .filter(|pin| pin.role == r)
                  .map(|pin| write!(f, " {}", pin.name))
                  .collect::<std::fmt::Result>();

        write!(f, "scheme {}", self.name)?;
        format_pins(f, &self.pins, Role::Input)?;
        write!(f, " ->")?;
        format_pins(f, &self.pins, Role::Output)?;
        write!(f, "\n")?;
        write!(f, "  -- Locals:")?;
        format_pins(f, &self.pins, Role::Local)?;
        write!(f, "\n")?;

        self.body.iter().for_each(|p| { let _ = p.fmt(f); } );

        write!(f, "end\n\n")
    }
}

impl Schematic {
    pub fn new(name: String, io_pins: (Vec<String>, Vec<String>), body: Vec<Instruction>) -> Self {
        let all_local_names: HashSet<String> = body.iter()
                .flat_map(|i| i.pin_bindings.iter().map(|n| n.clone()))
                .collect();
        // միայն IO-ները
        let inputs_and_outputs: HashSet<String> = io_pins.0.iter()
                .chain(io_pins.1.iter())
                .map(|n| n.clone())
                .collect();
        // լոկալներն առանց IO-ների
        let locals = all_local_names.difference(&inputs_and_outputs)
                                              .map(|n| Pin::new_local(n.to_string()));

        let mut pins: Vec<Pin> = io_pins.0.into_iter().map(Pin::new_input).collect();
        pins.extend(io_pins.1.into_iter().map(Pin::new_output));
        pins.extend(locals);

        Self {name, pins, body}
    }

    pub fn flatten(&self, library: &Vec<Schematic>) -> Self {
        let mut ng = NameGenerator::new("_t".to_string());
        self.flatten_internal(library, &mut ng)
    }

    fn flatten_internal(&self,  library: &Vec<Schematic>, ng: &mut NameGenerator) -> Self {
        let wrapper = Instruction {
            schematic_name: self.name.clone(),
            pin_bindings: self.pins
                .iter()
                .filter(|pin| pin.role != Role::Local)
                .map(|pin| pin.name.clone())
                .collect()
        };

        Schematic {
            name: self.name.clone(),
            pins: self.pins.clone(), // clone only inputs and outputs
            body: wrapper.expand(library, ng)
        }
        //Schematic::new()
    }
}

#[derive(Debug)]
pub struct Design {
    schematics: Vec<Schematic>
}

impl Design {
    pub fn new(schematics: Vec<Schematic>) -> Self {
        Self { schematics}
    }

    pub fn find(&self, name: &String) -> Option<&Schematic> {
        for sc in &self.schematics {
            if sc.name == *name {
                return Some(sc)
            }
        }

        None
    }

    pub fn get_schematics(&self) -> &Vec<Schematic> {
        &self.schematics
    }
}

impl Display for Design {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.schematics.iter().for_each(|sc| { let _ = sc.fmt(f); });
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use crate::schematic::{Instruction, Pin, Role, Schematic};

    #[test]
    fn test_simple_scheme() {
        // basis is NAND
        let nand_scheme = Schematic {
            name: "nand".into(),
            pins: vec![
                Pin { name: "a".into(), role: Role::Input },
                Pin { name: "b".into(), role: Role::Input },
                Pin { name: "x".into(), role: Role::Output },
            ],
            body: vec![]
        };
        let and_scheme = Schematic {
            name: "and".into(),
            pins: vec![
                Pin { name: "a".into(), role: Role::Input },
                Pin { name: "b".into(), role: Role::Input },
                Pin { name: "x".into(), role: Role::Output },
                Pin { name: "t".into(), role: Role::Local },
            ],
            body: vec![
                Instruction {
                    schematic_name: "nand".into(),
                    pin_bindings: vec!["a".into(), "b".into(), "t".into()],
                },
                Instruction {
                    schematic_name: "nand".into(),
                    pin_bindings: vec!["t".into(), "t".into(), "x".into(), ],
                },
            ]
        };
        let or_scheme = Schematic {
            name: "or".into(),
            pins: vec![
                Pin { name: "a".into(), role: Role::Input },
                Pin { name: "b".into(), role: Role::Input },
                Pin { name: "x".into(), role: Role::Output },
                Pin { name: "t0".into(), role: Role::Local },
                Pin { name: "t1".into(), role: Role::Local },
            ],
            body: vec![
                Instruction {
                    schematic_name: "nand".into(),
                    pin_bindings: vec!["a".into(), "a".into(), "t0".into()],
                },
                Instruction {
                    schematic_name: "nand".into(),
                    pin_bindings: vec!["b".into(), "b".into(), "t1".into()],
                },
                Instruction {
                    schematic_name: "nand".into(),
                    pin_bindings: vec!["t0".into(), "t1".into(), "x".into()],
                },
            ]
        };
        let not_scheme = Schematic {
            name: "not".into(),
            pins: vec![
                Pin { name: "a".into(), role: Role::Input },
                Pin { name: "x".into(), role: Role::Output },
            ],
            body: vec![
                Instruction {
                    schematic_name: "nand".into(),
                    pin_bindings: vec!["a".into(), "a".into(), "x".into()],
                }
            ]
        };
        let xor_scheme = Schematic {
            name: "xor".into(),
            pins: vec![
                Pin { name: "a".into(), role: Role::Input },
                Pin { name: "b".into(), role: Role::Input },
                Pin { name: "x".into(), role: Role::Output },
                Pin { name: "e0".into(), role: Role::Local },
                Pin { name: "e1".into(), role: Role::Local },
                Pin { name: "e2".into(), role: Role::Local },
                Pin { name: "e3".into(), role: Role::Local },
            ],
            body: vec![
                Instruction {
                    schematic_name: "not".into(),
                    pin_bindings: vec!["a".into(), "e0".into()],
                },
                Instruction {
                    schematic_name: "and".into(),
                    pin_bindings: vec!["e0".into(), "b".into(), "e1".into()],
                },
                Instruction {
                    schematic_name: "not".into(),
                    pin_bindings: vec!["b".into(), "e2".into()],
                },
                Instruction {
                    schematic_name: "and".into(),
                    pin_bindings: vec!["a".into(), "e2".into(), "e3".into()],
                },
                Instruction {
                    schematic_name: "or".into(),
                    pin_bindings: vec!["e2".into(), "e3".into(), "x".into()],
                },
            ]
        };
        let schemas = vec![xor_scheme, nand_scheme, and_scheme, or_scheme, not_scheme];

        let flattened = schemas[0].flatten(&schemas);
        println!("->\n{}", flattened)
    }

    #[test]
    fn test_flatten_primitive_schematic() {
        let not_scheme = Schematic::new(
            String::from("not"),
            (vec!["a".to_string()], vec!["x".to_string()]),
            vec![
                Instruction {
                    schematic_name: "nand".into(),
                    pin_bindings: vec!["a".into(), "a".into(), "x".into()],
                }
            ]);

        let library = vec![not_scheme];
        let flattened = &library[0].flatten(&library);
        println!("->\n{}", flattened)
    }
}
