mod file_utils;

fn main() -> std::io::Result<()> {
    let input_filename = "input.txt";
    let output_filename = "output.txt";
    let lines = file_utils::read_file(input_filename)?;
    let data = file_utils::parse_lines(lines, output_filename);
    match data {
        Ok(data) => {
            println!("Data processed successfully. Results:");
            for (name, value) in &data {
                println!("{}: {}", name, value);
            }
        }
        Err(err) => {
            eprintln!("Error processing data: {}", err);
        }
    }
    Ok(())
}
