mod file_utils;

fn main() -> std::io::Result<()> {
    let data = file_utils::read_and_parse_file("lines.txt")?;
    for (name, value) in data {
        if value >= 10 {
            println!("{}: {}", name, value);
        }
    }
    Ok(())
}
