use std::{fs::File, io::{BufRead, BufReader, BufWriter, Error, Write}, path::Path};





struct CsvHandler {
    data: Vec<Vec<String>>,
    headers: Option<Vec<String>>
}

impl CsvHandler {
    fn new() -> Self {
        CsvHandler{
            data: Vec::new(),
            headers: None
        }
    }


    fn read_csv(&mut self, file_path: &str, delimeter: &str, header: bool) -> Result<(), Error> {
        let path = Path::new(file_path);
        let f = File::open(path)?;

        for (index, line) in BufReader::new(f).lines().enumerate(){
            let record = line?.split(delimeter).map(|s| s.trim().to_string()).collect();
            
            if header && index == 0 {
                self.headers = Some(record);
            }else {
                self.data.push(record);
            } 
        }

        Ok(())
    }
    fn to_csv(&mut self, file_path: &str, delimeter: &str) -> Result<(), Error> {
        let path = Path::new(file_path);

        let f = match File::create(path) {
            Ok(f) => f,
            Err(e) => return Err(e),
        };

        let mut writer = BufWriter::new(f);

        if let Some(headers) = &self.headers {
            let header_line = headers.join(delimeter);
            writeln!(writer, "{}", header_line)?;
        }

        for record in &self.data {
            let line: Vec<String> = record.iter().map(|s| s.trim().to_string()).collect();
            let line = line.join(delimeter);
            writeln!(writer, "{}", line)?;

        }

        Ok(())
    }
}


fn main() {
    let mut csv = CsvHandler::new();
    let _ = csv.read_csv("./src/test.csv", ",", true);
    println!("{:?}", csv.data);
    let _ = csv.to_csv("./test.csv", "-");
}