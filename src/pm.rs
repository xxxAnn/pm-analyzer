pub type Number = i32;

pub enum Attribute {
    Name,
    Input,
    Output,
    Construction,
    Labor,
    EfficiencyPerWorker,
    NetOutput,
    EfficiencyPerConstruction
}

#[derive(Debug)]
pub struct PM {
    name: String,
    input: Number,
    output: Number,
    construction: Number,
    labor: Number,
}

impl PM {

    pub fn new(name: String, input: Number, output: Number, construction: Number, labor: Number) -> PM {
        PM {
            name,
            input,
            output,
            construction,
            labor,
        }
    }

    pub fn name(&self) -> &String {
        self.get_name()
    }

    pub fn get(&self, key: Attribute) -> Number {
        match key {
            Attribute::Input => self.get_input(),
            Attribute::Output => self.get_output(),
            Attribute::Construction => self.get_construction(),
            Attribute::Labor => self.get_labor(),
            Attribute::NetOutput => self.get_net_output(),
            Attribute::EfficiencyPerConstruction => self.get_efficiency_per_construction(),
            Attribute::EfficiencyPerWorker => self.get_efficiency_per_worker(),
            _ => panic!("Invalid attribute")
        }
    }
} 

// Private impl
impl PM {

    fn get_name(&self) -> &String {
        &self.name
    }

    fn get_input(&self) -> Number {
        self.input
    }

    fn get_output(&self) -> Number {
        self.output
    }

    fn get_construction(&self) -> Number {
        self.construction
    }

    fn get_labor(&self) -> Number {
        self.labor
    }
    
    fn get_net_output(&self) -> Number {
        self.get_output() - self.get_input()
    }

    fn get_efficiency_per_worker(&self) -> Number {
        self.get_net_output() / self.get_labor()
    }

    fn get_efficiency_per_construction(&self) -> Number {
        self.get_net_output() / self.get_construction()
    }
}

