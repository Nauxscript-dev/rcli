use csv::Reader;
use serde_json::Value;
use std::fs;

pub fn process_csv(input: &str, output: &str) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let header = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;
        let json_value = header.iter().zip(record.iter()).collect::<Value>();
        ret.push(json_value);
        // println!("{:?}", ret);
    }
    let json = serde_json::to_string_pretty(&ret)?;
    fs::write(output, json)?;
    Ok(())
}
