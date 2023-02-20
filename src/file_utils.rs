use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

pub fn read_file(input_filename: &str) -> Result<Vec<String>, std::io::Error> {
    let path = Path::new(input_filename);
    if !path.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("File not found: {}", path.display()),
        ));
    }
    if !path.is_file() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("{} is not a file.", path.display()),
        ));
    }

    let file = File::open(input_filename)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    Ok(lines)
}

pub fn parse_lines(
    lines: Vec<String>,
    output_filename: &str,
) -> Result<Vec<(String, i32)>, std::io::Error> {
    let output_file = File::create(output_filename)?;
    let mut writer = BufWriter::new(output_file);
    let mut result = Vec::new();
    for line in lines {
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 2 {
            let name = parts[0].trim().to_string();
            let value = parts[1].trim().parse::<i32>().unwrap_or(0);
            result.push((name.clone(), value));
            if value >= 10 {
                writeln!(writer, "{},{}", name, value)?;
            }
        }
    }
    Ok(result)
}
