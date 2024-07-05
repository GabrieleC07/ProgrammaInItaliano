use std::{env, fs::{self, remove_file, File}, io::{self, BufWriter, Write}, path::Path, process::Command};


pub fn get_arguments() -> Vec<String> {
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 && args.len() != 3 {
        panic!("\nPrevisto percorso del file come primo argomento, e l'opzione debug\n")
    }
    args
}
pub fn is_in_debug_mode(args: Vec<String>) -> bool {
    if args.len() == 3 {
        return true;
    }
    false
}
fn get_file_path(args: Vec<String>) -> Result<String, std::io::Error> {
    // The first element of the array is the program itself    
    Ok(args[1].clone().to_string())
}
fn get_extension(filename: &str) -> String {
    let path = Path::new(&filename).canonicalize().expect("\nE' previsto un file esistente\n");

    let filepath = path.to_str();
    let name = filepath.unwrap().split('/');
    let names: Vec<&str> = name.collect();
    let extension = names.last().expect("\nNon e' stato possibile leggere l'estensione del file\n");
    let extens: Vec<&str> = extension.split(".").collect();

    extens[1..(extens.len())].join(".").to_string()
}
pub fn get_file_contents() -> Result<String, std::io::Error> {
    let args = get_arguments();
    let path = get_file_path(args)?;
    
    if get_extension(&path) != "ita" {
        panic!("\nE' previsto un file con estensione .ita\n");
    }
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
        println!("Errore: {}", String::from_utf8(command.stderr).unwrap());
    }
}
pub fn remove_c_file(path: &str) -> io::Result<()> {
    remove_file(path)?;
    Ok(())
}