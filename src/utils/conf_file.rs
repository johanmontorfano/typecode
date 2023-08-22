use std::{fs, path::Path};
use serde_yaml::from_slice;
use crate::config::TranspilerExternalConfig;

use super::file::try_read_file_to_bytes;

// Determines if a configuration file for TypeCode has been found within the 
// provided directory path.
pub fn try_detect_conf_file_within_provided_directory(dir: String) -> bool {
    println!("Looking for configuration files within directory {}.", dir);
    
    if let Ok(dir_reader) = fs::read_dir(dir) {
        let mut found = false;
        dir_reader.for_each(|item| {
            if let Ok(item) = item {
                if item.file_name().to_str().unwrap() == "tc.conf.yaml" {
                    found = true;
                }
            }
        }); 

        return found;
    } else { 
        println!("Failed to read directory.");
        return false;
    }
}

// Read the tc.conf.yaml file: basically unecessary but the existence of this
// function improves code's readability + directly parses the file from a YAML
// representation to the actual `TranspilerExternalConfig` struct.
pub fn read_configuration_from_to_config_struct(mut dir: String) 
        -> TranspilerExternalConfig {
    if !dir.ends_with("/") { dir = format!("{}/", dir) }
    let config_file = try_read_file_to_bytes(format!("{}tc.conf.yaml", dir))
        .expect("Failed to read tc.conf.yaml even if it has been found.");

    return from_slice(config_file.as_slice())
        .expect("Failed to parse the configuration file.");
}
