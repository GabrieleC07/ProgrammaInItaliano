use std::{env, fs::{self, remove_file, File}, io::{self, BufWriter, Write}, process::Command};

fn get_file_path() -> Result<String, std::io::Error> {
    // The first element of the array is the program itself
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        panic!("EXPECTED ONLY ONE ARGUMENT, THE FILE PATH")
    }

    Ok(args[1].clone())
}
pub fn get_file_contents() -> Result<String, std::io::Error> {
    let path = get_file_path()?;
    let data = fs::read_to_string(path)?;

    Ok(data)
}
pub fn write_to_file(path: &str, content: &str) -> io::Result<()> {
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    
    writer.write_all(content.as_bytes())?;
    writer.flush()?;
    
    Ok(())
}
pub fn compile_c_to_out(path_c_code: &str, path_output: &str) {
    let command = Command::new("gcc")
        .arg("-o")
        .arg(path_output)
        .arg(path_c_code)
        .output()
        .expect("Failed to execute command");

    if !command.status.success() {
        println!("err: {}", String::from_utf8(command.stderr).unwrap());
    }
}
pub fn remove_c_file(path: &str) -> io::Result<()> {
    remove_file(path)?;
    Ok(())
}