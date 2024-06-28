use std::{env, fs::{self, File}, io::{self, BufWriter, Write}};

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
pub fn write_to_file(filename: &str, content: &str) -> io::Result<()> {
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);
    
    writer.write_all(content.as_bytes())?;
    writer.flush()?;
    
    Ok(())
}