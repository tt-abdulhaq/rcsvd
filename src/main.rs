use std::io::{BufRead, BufReader, BufWriter, Write};
use std::{fs::File,path::Path};
use std::error::Error;

#[derive(Debug)]
struct CustomError(&'static str);

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Error for CustomError {}

#[derive(Debug)]
struct CsvHandler {
    data: Vec<Vec<String>>
}
impl CsvHandler {
    fn new()->Self{
        CsvHandler{
            data: Vec::new()
        }
    }

    fn read_csv(&mut self, file_path: &str, delimiter: &str) -> Result<(), Box<dyn Error>> {
        let path = Path::new(file_path);

        if !path.exists() {
            return Err(Box::new(CustomError("File not exist")));
        }

        let f = File::open(path)?;

        let reader = BufReader::new(f);

        for line in reader.lines() {
            let record = line?;
            let row: Vec<String> = record.split(delimiter).map(|s| s.trim().to_string()).collect();
            self.data.push(row);
        }

        Ok(())
    }

    fn to_csv(&self, file_path: &str, delimiter: &str) -> Result<(), Box<dyn Error>> {
        let path = Path::new(file_path);
        let file = File::create(path)?;

        let mut writer = BufWriter::new(file);

        for row in &self.data {
            let line: Vec<String> = row.iter().map(|item| item.to_string()).collect();
            let line = line.join(delimiter);
            writer.write(line.as_bytes())?;
            writer.write(b"\n")?;
        }

        Ok(())
    }
}

fn main() {
    let mut csv = CsvHandler::new();
    let _ = csv.read_csv("./src/test.csv", " ");
    println!("{:?}", csv.data);
    let _ = csv.to_csv("./test.csv", ",");


}
