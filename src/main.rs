mod file;

mod lexer;
use lexer::lexer as lx;

mod parser;

mod code_gen;

fn main() -> Result<(), std::io::Error> {

    let contents = file::get_file_contents()?;
    let tokens = lx::run(contents);

    if tokens.is_none() {
        panic!("Lexer didnt found any tokens");
    }
    
    let node = parser::parser::parse(tokens.clone().unwrap());

    if node.is_none() {
        panic!("Parser didnt found any nodes");
    }

    code_gen::code_gen::gen(node.unwrap())?;

    Ok(())
}