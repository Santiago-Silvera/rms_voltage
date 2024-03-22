#![allow(warnings)]
use std::{error::Error, io, process};

#[derive(serde::Deserialize, Debug)]
struct Row {
    Time: String,
    Voltage: String,
    PeakDetect: String,
    Time2: String,
    Math: String,
}

fn read_csv() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let header = rdr.headers()?.clone();
    println!("{:?}", header);
    let mut n = 0.0;
    let mut v_sum: f32 = 0.0;
    for record in rdr.records() {
        let row: Row = record?.deserialize(Some(&header))?;
        // println!(
        //     "Time: {:?} \n\tVoltage: {:?}",
        //     row.Time.parse::<f32>().unwrap(),
        //     row.Voltage.parse::<f32>().unwrap()
        // );
        v_sum += f32::powf(row.Voltage.parse::<f32>().unwrap(), 2.0);
        n += 1.0;
    }
    let result = f32::sqrt((1.0 / n) * v_sum);
    println!("{:?}", result);
    Ok(())
}

fn main() {
    println!("Starting program");
    use std::time::Instant;
    let now = Instant::now();
    if let Err(err) = read_csv() {
        println!("error running example: {}", err);
        process::exit(1);
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
