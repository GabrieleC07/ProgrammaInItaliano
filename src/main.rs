use code_gen::code_gen::CodeGenerator;
use file::*;

mod file;

mod lexer;

mod parser;

mod code_gen;

mod test;

fn main() -> Result<(), std::io::Error> {    
    let contents = file::get_file_contents()?;
    let args = file::get_arguments();
    let is_in_debug = file::is_in_debug_mode(args);

    let tokens = lexer::lexer::run(contents);

    if tokens.is_none() {
        panic!("Lexer didnt found any tokens");
    }

    if is_in_debug {
        println!("\ntokens: {:?}\n", tokens.clone());
    }


    let mut parser = parser::parser::Parser::new(tokens.unwrap());

    let nodes_result = parser.parse_prog();

    if is_in_debug {
        println!("\nnode result: {:?}\n", nodes_result)
    }


    let nodes = match nodes_result {
        Ok(val) => val,
        Err(e) => panic!("Parser error! {}", e)
    };


    let mut code_generator = CodeGenerator::new();

    let code = code_generator.generate(nodes);

    let path_c = "build/out.c";
    let path_output = "build/a.out";
    write_to_file(&path_c, &code.clone())?;

    compile_c_to_out(&path_c, path_output);

    if !is_in_debug {
        remove_c_file(&path_c)?;
    }
    
    Ok(())
}