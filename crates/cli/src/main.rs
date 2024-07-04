use std::fs::{read_to_string, write};
use std::path::Path;

use toml::map::Map;
use toml::{Table, Value};

struct Extension {
    name: String,
    path: String,
}

fn main() {
    println!("Rs-extensions CLI");
    let extension_prefix = "extension-";
    let includes_path = Path::new(".extensions");
    let includes = match read_to_string(includes_path) {
        Ok(str) => str,
        Err(error) => {
            println!("Can't read .extensions: {:?}", error);
            return;
        }
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
                }
            };
            let mut extensions: Vec<Extension> = Vec::new();
            for entry in dirs {
                let extension_path = match entry {
                    Ok(entry) => entry.path(),
                    Err(error) => {
                        println!("Error Extension not found: {:?}", error);
                        return;
                    }
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
                    }
                };
                let name = match result {
                    Ok(cargo_toml) => cargo_toml["package"]["name"]
                        .as_str()
                        .unwrap_or("unknown")
                        .to_string(),
                    Err(error) => {
                        println!("Error Can't parse cargo.toml {:?}", error);
                        return;
                    }
                };
                let mut path = extension_path.to_str().unwrap().to_string();
                if extension_path.is_relative() {
                    path = "../.".to_string() + &path;
                }
                if name.starts_with(extension_prefix) {
                    extensions.push(Extension { name, path });
                }
            }

            // Glue config extensions dependencies
            let path = Path::new("crates/extensions");
            let mut config: Map<String, Value>;
            let config_path = path.join("Cargo.toml");
            if !config_path.exists() {
                match read_to_string(path.join("Cargo.template.toml")) {
                    Ok(template) => {
                        config = match template.parse::<Table>() {
                            Ok(table) => table,
                            Err(error) => {
                                println!("Error Can't parse Cargo.toml: {:?}", error);
                                return;
                            }
                        };
                    }
                    Err(_) => {
                        // Default config
                        config = toml::toml! {
                        [package]
                            name = "extensions"
                            version = "0.1.0"
                            edition = "2021"

                        [dependencies]
                        };
                    }
                }
            } else {
                match read_to_string(config_path.clone()) {
                    Ok(template) => {
                        config = match template.parse::<Table>() {
                            Ok(table) => table,
                            Err(error) => {
                                println!("Error Can't parse Cargo.toml: {:?}", error);
                                return;
                            }
                        };
                    }
                    Err(_) => {
                        // Default config
                        config = toml::toml! {
                        [package]
                            name = "extensions"
                            version = "0.1.0"
                            edition = "2021"

                        [dependencies]
                        };
                    }
                }
            }

            let mut dependencies: Map<String, Value> = Map::new();
            if let Value::Table(deps) = &config["dependencies"] {
                for (name, value) in deps {
                    if !name.starts_with(extension_prefix) {
                        // Inline table hack
                        dependencies.insert(name.to_string(), Value::String(value.to_string()));
                    }
                }
            }
            for ext in extensions {
                // Not inlined
                // let mut dep = Map::new();
                // dep.insert("path".to_owned(), Value::String(ext.path));
                // Inline table hack
                dependencies.insert(
                    ext.name,
                    Value::String(format!("{{ path = \"{}\" }}", ext.path)),
                );
            }
            config.insert("dependencies".to_owned(), toml::Value::Table(dependencies));
            // Inline table hack using replace
            if let Err(error) = write(config_path, config.to_string().replace("'", "")) {
                println!("Error Can't write Cargo.toml: {:?}", error);
            }
        } else {
            println!("Error Include not found: {:?}", include);
        }
    }
}
