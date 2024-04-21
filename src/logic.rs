#[derive(Debug, Clone, Copy)]
pub enum Tag {
    Test,
    Error,
    Clear,
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Inactive,
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

#[derive(Debug, Clone, Copy)]
pub struct Backend {
    Num1: i128,
    num2: i128,
    operation: Operation,
}

impl Backend {
    pub fn update(&self) {
        match self.operation {
            Operation::Inactive => (),
            Operation::Addition=> (),
            Operation::Subtraction => (),
            Operation::Multiplication => (),
            Operation::Division => (),
        }
    }
}

impl Tag {
    pub fn click_action(&self) {
        match self {
            Tag::Test => test(),
            _ => (),
        }
    }
}

fn test() {
    println!("TESTING BUTTON");
}
