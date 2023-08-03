use std::{fs, io::Result};

mod ast;
mod emitter;
mod error;
mod modifiers;
mod parser;
mod semantics;
mod span;
mod tokenizer;
mod utils;
mod visitor;

pub fn run(file_name: &str) -> Result<()> {
    let text = fs::read_to_string(file_name)?;

    if let Err(err) = scaffold(&text) {
        eprintln!("{}", err);
        std::process::exit(1);
    }

    Ok(())
}

fn scaffold(text: &str) -> error::Result<()> {
    let tokens = tokenizer::Tokenizer::new().tokenize(&text)?;
    println!("Tokens:\n {:#?}", tokens);

    let ast = parser::Parser::new().parse(&text, &tokens)?;
    println!("AST:\n {:#?}", ast);

    match ast {
        ast::Ast::Root(ref root) => {
            let mut analyzer = semantics::SemanticAnalyzer::new(&text);
            let errors = analyzer.analyze(&root)?;
            println!("errors:\n {:#?}", errors);
        }
        _ => unreachable!(),
    }

    let mut discoverer = modifiers::ModifierDiscoverer::new();
    let modifiers = discoverer.discover(&ast);
    println!("modifiers:\n {:#?}", modifiers);

    let solcode = emitter::Emitter::new(true, 2).emit(&ast, &modifiers);
    println!("solcode:\n{}", solcode);

    Ok(())
}
