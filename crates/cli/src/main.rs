use std::fs::{read_to_string, write};
use std::path::Path;

use toml::Table;

struct Extension {
    name: String,
    path: String,
}

fn main() {
    println!("Rs-extensions CLI");
    let includes_path = Path::new(".extensions");
    let includes = match read_to_string(includes_path) {
        Ok(str) => str,
        Err(error) => {
            println!("Can't read .extensions: {:?}", error);
            return;
        },
    };
    for include in includes.lines() {
        println!("include ={:?}", include);
        let include_path = Path::new(include);
        if include_path.exists() && include_path.is_dir() {
            let dirs = match include_path.read_dir() {
                Ok(dirs) => dirs,
                Err(error) => {
                    println!("Error Include not found: {:?}", error);
                    return;
                },
            };
            let mut extensions: Vec<Extension> = Vec::new();
            for entry in dirs {
                let extension_path = match entry {
                    Ok(entry) => entry.path(),
                    Err(error) => {
                        println!("Error Extension not found: {:?}", error);
                        return;
                    },
                };
                println!("Extension path={:?}", extension_path);
                let cargo_toml = extension_path.join("Cargo.toml");
                if !cargo_toml.exists() {
                    println!("Error Cargo.tml not found: {:?}", extension_path);
                    return;
                }

                let result = match read_to_string(cargo_toml) {
                    Ok(str) => str.parse::<Table>(),
                    Err(error) => {
                        println!("Error Can't read Cargo.toml: {:?}", error);
                        return;
                    },
                };
                let name = match result {
                    Ok(cargo_toml) => cargo_toml["package"]["name"].as_str().unwrap_or("unknown").to_string(),
                    Err(error) => {
                    println!("Error Can't parse cargo.toml {:?}", error);
                    return;
                }};
                let mut path = extension_path.to_str().unwrap().to_string();
                if extension_path.is_relative() {
                    path = "../.".to_string() + &path;
                }
                extensions.push(Extension { name, path })
            }

            // Glue config extensions dependencies
            let path = Path::new("crates/extensions");
            let template = match read_to_string(path.join("Cargo.template.toml")) {
                Ok(str) => str,
                Err(error) => {
                    println!("Error Can't read Template Cargo.toml: {:?}", error);
                    return;
                },
            };
            let mut config = template + "\n";
            for ext in extensions {
                config += &format!("{} = {{ path = \"{}\" }}\n", ext.name, ext.path);
            }
            let config_path = path.join("Cargo.toml");
            if let Err(error ) = write(config_path, config) {
                println!("Error Can't write Cargo.toml: {:?}", error);
            }

        } else {
            println!("Error Include not found: {:?}", include);
        }
    }
}
