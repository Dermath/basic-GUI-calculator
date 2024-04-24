#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tag {
    Error,
    Test,
    Num(u8),
    Op(Operation),
    Eq,
    Clear,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operation {
    Inactive,
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Backend {
    pub new: bool,
    pub num1: i128,
    pub num2: i128,
    pub operation: Operation,
}

impl Tag {
    pub fn click_action(&self, backend: &mut Backend) {
        match self {
            Tag::Test => test(),
            Tag::Clear => clear(backend),
            Tag::Op(inner) => operation(backend, inner),
            Tag::Eq => eq(backend),
            Tag::Num(inner) => num(backend, inner),
            _ => (),
        }
    }
}

fn test() {
    // eprintln!("TESTING BUTTON");
}

fn clear(backend: &mut Backend) {
    backend.new = true;
    backend.num1 = 0;
    backend.num2 = 0;
    backend.operation = Operation::Inactive;
}

fn num(backend: &mut Backend, num: &u8) {
    if backend.new == false && (backend.num2 as u128) < {2_u128.pow(127)/10} {
        // eprintln!("input: {}", num);
        backend.num2 = backend.num2 * 10 + *num as i128;
    }
    else {
        backend.num2 = *num as i128;
        backend.new = false;
    }
}

fn operation(backend: &mut Backend ,op: &Operation) {
    if *op == Operation::Addition {
        eq(backend);
        backend.operation = Operation::Addition;
        backend.num1 = backend.num2;
        backend.new = true;
    }
    else if *op == Operation::Subtraction {
        eq(backend);
        backend.operation = Operation::Subtraction;
        backend.num1 = backend.num2;
        backend.new = true;
    }
    else if *op == Operation::Multiplication {
        eq(backend);
        backend.operation = Operation::Multiplication;
        backend.num1 = backend.num2;
        backend.new = true;
    }
    else if *op == Operation::Division {
        eq(backend);
        backend.operation = Operation::Division;
        backend.num1 = backend.num2;
        backend.new = true;
    }
}

fn eq(backend: &mut Backend) {
    backend.num2 = match backend.operation {
        Operation::Addition => backend.num1 + backend.num2,
        Operation::Subtraction => backend.num1 - backend.num2,
        Operation::Multiplication => backend.num1 * backend.num2,
        Operation::Division => backend.num1 / backend.num2,
        Operation::Inactive => backend.num2,
    };
    backend.operation = Operation::Inactive;
    backend.num1 = 0;
}


