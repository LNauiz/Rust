
#[derive(Debug, PartialEq)]
pub enum Order {
    Forward,
    Backward,
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
pub struct Number(pub i32);

#[derive(Debug, PartialEq)]
pub struct Command {
    pub order: Order,
    pub number: Number,
}

#[derive(Debug, PartialEq)]
pub enum Program {
    CommandThenProgram(Box<Command>, Box<Program>),
    Empty,
}



