use std::{env, fs, process, io};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file name>", args[0]);
        process::exit(1);
    }

    let file_name: &str = &args[1];
    if let Err(e) = copy_file(file_name) {
        eprintln!("Error copying file: {}", e);
        process::exit(1);
    }

    println!("File copied successfully: {}", file_name);
}

fn copy_file(original: &str) -> io::Result<()> {
    let bak = format!("{}.bak", original);
    fs::copy(original, &bak)?;
    Ok(())
}

