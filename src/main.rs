use code_gen::code_gen::CodeGenerator;
use file::{compile_c_to_out, remove_c_file, write_to_file};

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

    println!("{:?}", tokens.clone());

    let mut parser = parser::parser::Parser::new(tokens.unwrap());
    let nodes_result = parser.parse_prog();


    let nodes = match nodes_result {
        Ok(val) => val,
        Err(e) => panic!("Compiling error! {}", e)
    };


    let mut code_generator = CodeGenerator::new();

    let code = code_generator.generate(nodes);

    let path_c = "build/out.c";
    let path_output = "build/a.out";
    write_to_file(&path_c, &code.clone())?;

    compile_c_to_out(&path_c, path_output);

    remove_c_file(&path_c)?;
    
    Ok(())
}