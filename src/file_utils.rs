use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_and_parse_file(filename: &str) -> Result<Vec<(String, i32)>, std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut result = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 2 {
            let name = parts[0].trim().to_string();
            let value = parts[1].trim().parse::<i32>().unwrap_or(0);
            result.push((name, value));
        }
    }
    Ok(result)
}
