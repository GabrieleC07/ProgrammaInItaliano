use crate::{file, parser::parser::NodeExit};

pub fn gen(node: NodeExit) -> std::io::Result<()> { 
    let code = format!("int main(int argc, char const *argv[]) \n{{
    return {};\n}}",
        node.expr.int_lit.value.expect("Could not find a value inside NodeExit token")
    );
    file::write_to_file("build/out.c", &code)?;
    

    Ok(())
}