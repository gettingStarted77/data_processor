use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Error, ErrorKind, Write};
use std::path::{Path, PathBuf};

pub fn read_file(input_filename: &str) -> Result<Vec<String>, Error> {
    let file = match File::open(input_filename) {
        Ok(file) => file,
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                eprintln!("Input file not found: {}", input_filename);
            } else {
                eprintln!("Error opening input file: {}", e);
            }
            return Err(e);
        }
    };
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    Ok(lines)
}

pub fn parse_line(line: &str) -> Option<(String, i32)> {
    let re = Regex::new(r#"^\s*(?P<name>\S+)\s*,\s*(?P<value>-?\d+)\s*$"#).unwrap();
    let caps = re.captures(line)?;
    let name = caps["name"].to_string();
    let value = caps["value"].parse().unwrap();
    Some((name, value))
}

pub fn parse_lines(lines: Vec<String>, output_filename: &str) -> Result<Vec<(String, i32)>, Error> {
    let path = Path::new(output_filename);
    let output_file = if path.exists() && path.is_file() {
        let new_path = generate_new_filename(&path);
        println!(
            "The file {} already exists. Saving to {} instead.",
            path.display(),
            new_path.display()
        );
        File::create(new_path)?
    } else {
        File::create(output_filename)?
    };

    let mut writer = BufWriter::new(output_file);
    let mut result = Vec::new();
    for line in lines {
        if let Some((name, value)) = parse_line(&line) {
            result.push((name.clone(), value));
            if value >= 10 {
                writeln!(writer, "{},{}", name, value)?;
            }
        }
    }
    Ok(result)
}

fn generate_new_filename(path: &Path) -> PathBuf {
    let new_filename = format!(
        "{}_{}.txt",
        path.file_stem().unwrap().to_str().unwrap(),
        chrono::Utc::now().format("%Y-%m-%d_%H-%M-%S")
    );
    path.with_file_name(new_filename)
}
