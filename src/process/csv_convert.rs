use csv::Reader;
use serde_json::Value;
use std::fs;

use crate::opts::OutputFormat;

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    let header = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;
        let json_value = header.iter().zip(record.iter()).collect::<Value>();
        ret.push(json_value);
        // println!("{:?}", ret);
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };

    fs::write(output, content)?;
    Ok(())
}
