use std::path::Path;

use crate::schematic::Design;

type DataVector = Vec<bool>;


struct SimulationData {
    input: Vec<DataVector>,
    expected: Vec<DataVector>,
    actual: Vec<DataVector>,
}

impl SimulationData {
    fn load<P: AsRef<Path>>(&mut self, file: P) {
        todo!()
    }

    fn store<P: AsRef<Path>>(&mut self, file: P) {
        todo!()
    }
}

pub struct Simulator {
    design: Design
}

impl Simulator {
    pub fn new(design: Design) -> Self {
        Self { design }
    }

    pub fn simulate(&self, schematic_name: &String) {
        let top_schematic = self.design.find(schematic_name);
        let flattened = top_schematic.unwrap().flatten(self.design.get_schematics());
        println!("=>\n{:?}", flattened);
    }
}
