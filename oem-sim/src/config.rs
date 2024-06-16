
use serde::{Serialize, Deserialize};
use serde_json;
use std::fs::File;
use std::io::{BufRead, Read};
use std::io::BufReader;
use yaml_rust::{YamlLoader, ScanError};

#[derive(Debug, Deserialize, Serialize)]
struct StorageConfig {
    #[serde(rename = "connection_string")]
    connection_string: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct SimulationConfig {
    #[serde(rename = "frequency")]
    frequency: u32,
    #[serde(rename = "max_peak")]
    max_peak: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AssetDef {
    asset_id: String,
    asset_oem_alarm : Vec<String>
}
#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub storage: StorageConfig,
    pub simulation: SimulationConfig,
}

pub fn process_asset_def() -> Result<Vec<AssetDef>, String> {
    // Read the JSON file
    // Open the file
    let mut file = match File::open("/Users/christophebuffard/Documents/Dev/github.com/OEM-sim/oem-sim/assets.json") {
        Ok(file) => file,
        Err(err) => return Err(err.to_string()),
    };

    // Create a buffered reader
    let reader = BufReader::new(file);

    let mut asset_def_list: Vec<AssetDef> = Vec::new();

    for line in reader.lines() {
        let line = line;
        match line {
            Ok(line) => {
            // Deserialize the JSON data from the file
                let asset_def: Result<AssetDef, serde_json::Error> = serde_json::from_str(&line);
                match asset_def {
                    Ok(def) => asset_def_list.push(def),
                    Err(err) => return Err(err.to_string()),
                }
            },
            Err(err) => return Err(err.to_string())
        }
    }

    Ok(asset_def_list)
}
pub fn alt_process_yaml() -> Result<AppConfig, String> {
    // Read the YAML file
    let mut file = match File::open("/Users/christophebuffard/Documents/Dev/github.com/OEM-sim/oem-sim/config.yml") {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            return Err(e.to_string());
        }
    };

    let mut contents = String::new();
    //file.read_to_string(&mut contents).expect("Failed to read file");
    if let Err(e) = file.read_to_string(&mut contents) {
        eprintln!("Error reading file: {}", e);
        return Err(e.to_string());
    }

    // Parse YAML into Rust structs
    //let docs = YamlLoader::load_from_str(&contents).expect("Failed to parse YAML");
    let docs_res = YamlLoader::load_from_str(&contents);
    if docs_res.is_err() {
        return Err(docs_res.err().unwrap().to_string());
    }
    let doc = &docs_res.unwrap()[0]; // Assuming only one document in the YAML file

    // Access and print the parsed values
    let storage = StorageConfig {
        connection_string: doc["mongodb"]["connection_string"]
            .as_str()
            .expect("Invalid connection string")
            .to_string(),
    };
    let simulation = SimulationConfig {
        frequency: doc["frequency"]["frequency"]
            .as_i64()
            .expect("Invalid frequency value") as u32,
        max_peak: doc["frequency"]["max_peak"]
            .as_i64()
            .expect("Invalid max_peak value") as u32,
    };

    let config = AppConfig { storage, simulation };
    Ok(config)
}