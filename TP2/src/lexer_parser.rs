use santiago::lexer::{Lexer, LexerRules};
use santiago::grammar::Grammar;

#[derive(Debug)]
pub enum AST {
    Program(Vec<AST>),
    Command(Vec<AST>),
    Order(Order),
    Number(i32),
    Empty,
}

#[derive(Debug)]
pub enum Order{
    Forward,
    Backward,
    Left,
    Right,
}

#[derive(Debug)]
pub enum Command {
    Order(Order),
    Number(Number),
}

#[derive(Debug)]
pub struct Number(pub i32);


pub fn lexer_rules() -> LexerRules{
    santiago::lexer_rules!(
        "DEFAULT" | "FORWARD" = string "forward";
        "DEFAULT" | "BACKWARD" = string "backward";
        "DEFAULT" | "RIGHT" = string "right";
        "DEFAULT" | "LEFT" = string "left";
        "DEFAULT" | "NUMBER" = pattern r"[0-9]+";
        "DEFAULT" | "WS" = pattern r"\s+" => |lexer| lexer.skip();
    )
}


pub fn grammar() -> Grammar<AST> {
    santiago::grammar!(
        "program" => rules "command" "program" => AST::Program;
        "program" => empty => |_| AST::Empty; //
        "command" => rules "order" "number" => AST::Command;
        "order" => lexemes "FORWARD"=> |lexemes| AST::Order(Order::Forward);
        "order" => lexemes "BACKWARD"=> |lexemes| AST::Order(Order::Backward);
        "order" => lexemes "LEFT"=> |lexemes| AST::Order(Order::Left);
        "order" => lexemes "RIGHT" => |lexemes| AST::Order(Order::Right);
        "number" => lexemes "NUMBER"=> |lexemes| {
            // &str to i32 conversion
            let value = str::parse::<i32>(&lexemes[0].raw).unwrap();

            AST::Number(value)
        };

    )
}

pub fn eval(ast: &AST) -> () {
    match *ast {
        AST::Empty => (),
        AST::Command() =>
            match *ast{

            }


        ,
        AST::Order(Order::Forward) => ,
        AST::Order(Order::Backward) => (),
        AST::Order(Order::Left) => (),
        AST::Order(Order::Right) => (),

    }
}


