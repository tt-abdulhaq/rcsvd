use std::{error::Error, fs::File, io::{BufRead, BufReader, BufWriter, Write}, path::Path};





struct CsvHandler {
    data: Vec<Vec<String>>,
    headers: Option<Vec<String>>
}


#[allow(dead_code)]
impl CsvHandler {
    fn new() -> Self {
        CsvHandler{
            data: Vec::new(),
            headers: None
        }
    }


    fn read_csv(&mut self, file_path: &str, delimeter: &str, header: bool) -> Result<(), Box<dyn Error>> {
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
    
    fn to_csv(&mut self, file_path: &str, delimeter: &str) -> Result<(), Box<dyn Error>> {
        let path = Path::new(file_path);

        let f = match File::create(path) {
            Ok(f) => f,
            Err(e) => return Err(Box::new(e)),
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

    fn shape(&self) -> (usize, usize) {
        let row = self.data.len();
        let col = self.data[0].len();
        (row, col)
    }

    fn get_row(&self, i:usize) -> Option<&Vec<String>> {
        let row = self.data.get(i)?;
        Some(row)
    }

    fn head(&self) -> Option<&[Vec<String>]>{

        let (row, _ ) = &self.shape();
        if row > &4 {
            Some(&self.data[0..4])
        }else{
            Some(&self.data[0..*row])
        }
    }
}



fn main() {
    let mut csv = CsvHandler::new();
    let _ = csv.read_csv("./src/test1.csv", ",", true);
    println!("{:?}", csv.head());
    
}