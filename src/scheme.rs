use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Pin {
    name: String,
    role: Role,
}

#[derive(Debug, PartialEq)]
pub enum Role {
    Input,
    Output,
    Local,
}

#[derive(Debug, Clone)]
pub struct Instruction {
    scheme_name: String,
    pin_bindings: Vec<String>,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = String::from(&self.scheme_name);
        for p in &self.pin_bindings {
            result.push_str(&format!(" {}", p));
        }
        write!(f, "{}", result)
    }
}

impl Instruction {
    pub fn expand(&self, library: &Vec<Scheme>) {
        if self.scheme_name == "nand" {
            println!("  {}", self);
            return
        }

        let scheme = library.iter().find(|&s| s.name == self.scheme_name).unwrap();

        let mut pin_map: HashMap<String, String> = HashMap::new();
        for i in 0..scheme.pins.len() {
            if scheme.pins[i].role == Role::Local {
                pin_map.insert(scheme.pins[i].name.clone(), scheme.pins[i].name.clone()); // TODO: rename
            } else {
                pin_map.insert(scheme.pins[i].name.clone(), self.pin_bindings[i].clone());
            }
        }

        let mut modified: Vec<Instruction> = vec![];
        for instr in &scheme.body {
            let mut bindings: Vec<String> = vec![];
            for p in &instr.pin_bindings {
                let name = pin_map.get(p).unwrap();
                bindings.push(name.clone());
            }

            modified.push(Instruction {
                scheme_name: instr.scheme_name.clone(),
                pin_bindings: bindings
            });
        }
        modified.iter().for_each(|instr| {instr.expand(library);});
    }
}

#[derive(Debug)]
pub struct Scheme {
    name: String,
    pins: Vec<Pin>,
    body: Vec<Instruction>,
}

impl Scheme {
    fn flatten(&self,  library: &Vec<Scheme>) {
        for instr in &self.body {
            instr.expand(library)
        }
    }
}

mod test {
    use crate::scheme::{Instruction, Pin, Role, Scheme};

    #[test]
    fn test_simple_scheme() {
        // basis is NAND
        let nand_scheme = Scheme {
            name: "nand".into(),
            pins: vec![
                Pin { name: "a".into(), role: Role::Input },
                Pin { name: "b".into(), role: Role::Input },
                Pin { name: "x".into(), role: Role::Output },
            ],
            body: vec![]
        };
        let and_scheme = Scheme {
            name: "and".into(),
            pins: vec![
                Pin { name: "a".into(), role: Role::Input },
                Pin { name: "b".into(), role: Role::Input },
                Pin { name: "x".into(), role: Role::Output },
                Pin { name: "t".into(), role: Role::Local },
            ],
            body: vec![
                Instruction {
                    scheme_name: "nand".into(),
                    pin_bindings: vec!["a".into(), "b".into(), "t".into()],
                },
                Instruction {
                    scheme_name: "nand".into(),
                    pin_bindings: vec!["t".into(), "t".into(), "x".into(), ],
                },
            ]
        };
        let or_scheme = Scheme {
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
                    scheme_name: "nand".into(),
                    pin_bindings: vec!["a".into(), "a".into(), "t0".into()],
                },
                Instruction {
                    scheme_name: "nand".into(),
                    pin_bindings: vec!["b".into(), "b".into(), "t1".into()],
                },
                Instruction {
                    scheme_name: "nand".into(),
                    pin_bindings: vec!["t0".into(), "t1".into(), "x".into()],
                },
            ]
        };
        let not_scheme = Scheme {
            name: "not".into(),
            pins: vec![
                Pin { name: "a".into(), role: Role::Input },
                Pin { name: "x".into(), role: Role::Output },
            ],
            body: vec![
                Instruction {
                    scheme_name: "nand".into(),
                    pin_bindings: vec!["a".into(), "a".into(), "x".into()],
                }
            ]
        };
        let schemas = vec![nand_scheme, and_scheme, or_scheme, not_scheme];

        let xor_scheme = Scheme {
            name: "xor".into(),
            pins: vec![
                Pin { name: "a".into(), role: Role::Input },
                Pin { name: "b".into(), role: Role::Input },
                Pin { name: "x".into(), role: Role::Output },
            ],
            body: vec![
                Instruction {
                    scheme_name: "not".into(),
                    pin_bindings: vec!["a".into(), "e0".into()],
                },
                Instruction {
                    scheme_name: "and".into(),
                    pin_bindings: vec!["e0".into(), "b".into(), "e1".into()],
                },
                Instruction {
                    scheme_name: "not".into(),
                    pin_bindings: vec!["b".into(), "e2".into()],
                },
                Instruction {
                    scheme_name: "and".into(),
                    pin_bindings: vec!["a".into(), "e2".into(), "e3".into()],
                },
                Instruction {
                    scheme_name: "or".into(),
                    pin_bindings: vec!["e2".into(), "e3".into(), "x".into()],
                },
            ]
        };
        xor_scheme.flatten(&schemas);
    }
}
