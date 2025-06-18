use crate::schematic::Design;

pub struct Simulator {
    design: Design
}

type Values = Vec<bool>;

impl Simulator {
    pub fn new(design: Design) -> Self {
        Self { design }
    }

    pub fn simulate(schematic_name: String) {
        todo!()
    }
}
