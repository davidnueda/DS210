use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

pub struct Passenger {
    pub survived: f64,
    pub pclass: f64,
    pub sex: f64,
    pub age: f64,
}

// Move preprocess_data and read_data functions here
pub fn preprocess_data(input_path: &Path, output_path: &Path) -> Result<(), Box<dyn Error>> {
    let mut reader: csv::Reader<File> = csv::Reader::from_path(&input_path)?;
    let mut writer = csv::Writer::from_path(&output_path)?;

    writer.write_record(&["Survived", "Pclass", "Sex", "Age"])?;

    for result in reader.records() {
        let record = result?;

        let survived = &record[1]; // Column index for "Survived"
        let pclass = &record[2]; // Column index for "Pclass"
        let sex = &record[4]; // Column index for "Sex"
        let age = &record[5]; // Column index for "Age"

        // Filtering out rows with missing age values
        if !age.is_empty() {
            writer.write_record(&[survived, pclass, sex, age])?;
        }
    }

    writer.flush()?;

    Ok(())
}
pub fn read_data(filename: &str) -> Vec<Passenger> {
    let file = File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut passengers = Vec::new();

    for line in reader.lines().skip(1) {
        let line = line.unwrap();
        let values: Vec<&str> = line.split(',').collect();

        if let (Ok(survived), Ok(pclass), Ok(age)) = (
            f64::from_str(values[0]),
            f64::from_str(values[1]),
            f64::from_str(values[3]),
        ) {
            passengers.push(Passenger {
                survived,
                pclass,
                sex: if values[2] == "male" { 0.0 } else { 1.0 },
                age,
            });
        } else {
            eprintln!("Invalid row: {}", line);
        }
    }

    passengers

}

