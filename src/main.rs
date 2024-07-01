use code_gen::code_gen::CodeGenerator;
use file::write_to_file;

mod file;

mod lexer;

mod parser;

mod code_gen;

mod test;

fn main() -> Result<(), std::io::Error> {
    let contents = file::get_file_contents()?;

    let tokens = lexer::lexer::run(contents);

    if tokens.is_none() {
        panic!("Lexer didnt found any tokens");
    }

    let mut parser = parser::parser::Parser::new(tokens.unwrap());
    let mut nodes = parser.parse_prog();

    println!("Nodes {:?}", nodes.as_mut().unwrap());

    let stmts = match nodes {
        Ok(val) => val,
        Err(e) => panic!("Compiling error! {}", e)
    };


    let mut code_generator = CodeGenerator::new();

    let code = code_generator.generate(stmts);

    println!("Code {:?}", code.clone());

    write_to_file("build/out.c", &code.clone())?;
    
    Ok(())
}