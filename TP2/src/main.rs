
/*
Ecrire un carré en Logo

pendown
repeat 4 [
    forward 10
    right
]
penup



*/


use::TP2::lexer_parser;

fn main() {
    println!("Hello, world!");
    let lexer_rules = lexer_parser::lexer_rules();
    let lexemes = santiago::lexer::lex(&lexer_rules,"forward 100").unwrap();
    println!("{:#?}", lexemes);
    let grammar = lexer_parser::grammar();
    let parse_trees = &santiago::parser::parse(&grammar, &lexemes).expect("syntax error")[0];
    println!("{:#?}", parse_trees);
    println!("{:?}", parse_trees.as_abstract_syntax_tree());

}
