use serde_json::Value;
use std::collections::HashMap;
use std::{fs, path::Path};
use termdiff::{ArrowsColorTheme, DrawDiff};

use nwn_nasher_types::*;

#[cfg(test)]
mod area {
    #[test]
    fn serialize() {
        super::serialize("assets/testing/test.are.json".to_string());
    }

    #[test]
    fn round_trip() {
        super::round_trip("assets/testing/test.are.json".to_string());
    }
}

#[cfg(test)]
mod dialog {
    #[test]
    fn serialize() {
        super::serialize("assets/testing/test.dlg.json".to_string());
    }

    #[test]
    fn round_trip() {
        super::round_trip("assets/testing/test.dlg.json".to_string());
    }
}

#[cfg(test)]
mod faction {
    #[test]
    fn serialize() {
        super::serialize("assets/testing/test.fac.json".to_string());
    }

    #[test]
    fn round_trip() {
        super::round_trip("assets/testing/test.fac.json".to_string());
    }
}

#[cfg(test)]
mod area_comments {
    #[test]
    fn serialize() {
        super::serialize("assets/testing/test.gic.json".to_string());
    }

    #[test]
    fn round_trip() {
        super::round_trip("assets/testing/test.gic.json".to_string());
    }
}

#[cfg(test)]
mod area_info {
    #[test]
    fn serialize() {
        super::serialize("assets/testing/test.git.json".to_string());
    }

    #[test]
    fn round_trip() {
        super::round_trip("assets/testing/test.git.json".to_string());
    }
}

#[cfg(test)]
mod module_info {
    #[test]
    fn serialize() {
        super::serialize("assets/testing/test.ifo.json".to_string());
    }

    #[test]
    fn round_trip() {
        super::round_trip("assets/testing/test.ifo.json".to_string());
    }
}

#[cfg(test)]
mod palette {
    #[test]
    fn serialize() {
        super::serialize("assets/testing/test.itp.json".to_string());
    }

    #[test]
    fn round_trip() {
        super::round_trip("assets/testing/test.itp.json".to_string());
    }
}

#[cfg(test)]
mod journal {
    #[test]
    fn serialize() {
        super::serialize("assets/testing/test.jrl.json".to_string());
    }

    #[test]
    fn round_trip() {
        super::round_trip("assets/testing/test.jrl.json".to_string());
    }
}

#[cfg(test)]
mod creature {
    #[test]
    fn serialize() {
        super::serialize("assets/testing/test.utc.json".to_string());
    }

    #[test]
    fn round_trip() {
        super::round_trip("assets/testing/test.utc.json".to_string());
    }
}

#[cfg(test)]
mod door {
    #[test]
    fn serialize() {
        super::serialize("assets/testing/test.utd.json".to_string());
    }

    #[test]
    fn round_trip() {
        super::round_trip("assets/testing/test.utd.json".to_string());
    }
}

#[cfg(test)]
mod encounter {
    #[test]
    fn serialize() {
        super::serialize("assets/testing/test.ute.json".to_string());
    }

    #[test]
    fn round_trip() {
        super::round_trip("assets/testing/test.ute.json".to_string());
    }
}

#[cfg(test)]
mod item_blueprint {
    #[test]
    fn serialize() {
        super::serialize("assets/testing/test.uti.json".to_string());
    }

    #[test]
    fn round_trip() {
        super::round_trip("assets/testing/test.uti.json".to_string());
    }
}

mod store {
    #[test]
    fn serialize() {
        super::serialize("assets/testing/test.utm.json".to_string());
    }

    #[test]
    fn round_trip() {
        super::round_trip("assets/testing/test.utm.json".to_string());
    }
}

mod placeable {
    #[test]
    fn serialize() {
        super::serialize("assets/testing/test.utp.json".to_string());
    }

    #[test]
    fn round_trip() {
        super::round_trip("assets/testing/test.utp.json".to_string());
    }
}

mod sound {
    #[test]
    fn serialize() {
        super::serialize("assets/testing/test.uts.json".to_string());
    }

    #[test]
    fn round_trip() {
        super::round_trip("assets/testing/test.uts.json".to_string());
    }
}

mod trigger {
    #[test]
    fn serialize() {
        super::serialize("assets/testing/test.utt.json".to_string());
    }

    #[test]
    fn round_trip() {
        super::round_trip("assets/testing/test.utt.json".to_string());
    }
}

mod waypoint {
    #[test]
    fn serialize() {
        super::serialize("assets/testing/test.utw.json".to_string());
    }

    #[test]
    fn round_trip() {
        super::round_trip("assets/testing/test.utw.json".to_string());
    }
}

const VARIANTS: [&str; 17] = [
    "are", "dlg", "fac", "gic", "git", "ifo", "itp", "jrl", "utc", "utd", "ute", "uti", "utm",
    "utp", "uts", "utt", "utw",
];

pub fn serialize(path: String) {
    let path = Path::new(&path);
    match path.exists() {
        true => {
            let file = fs::File::open(&path).expect("Failed to open file");
            let nw: Result<NwType, _> = serde_json::from_reader(file);
            match nw {
                Ok(_) => {}
                Err(e) => {
                    panic!("Failed to deserialize {:?}: {}", path, e);
                }
            }
        }
        false => {
            panic!("File does not exist at path: {:?}", path);
        }
    }
}

pub fn round_trip(path: String) {
    let path = Path::new(&path);
    match path.exists() {
        true => {
            let file = fs::File::open(&path).expect("Failed to open file");

            let nw: Result<NwType, _> = serde_json::from_reader(file);
            match nw {
                Ok(value) => {
                    // serialize
                    let serialized = match serde_json::to_string_pretty(&value) {
                        Ok(s) => s + "\n",
                        Err(e) => panic!("Failed to serialize: {}", e),
                    };
                    // get original data
                    let original = fs::read_to_string(&path).expect("Failed to read file");

                    if original != serialized {
                        let diff = diff_strings(original.clone(), serialized.clone());

                        // ignore the -0.0 to 0.0 differences
                        for (line, l1, l2) in diff {
                            let tmp_json = format!("{{{}}}", &l1.trim());
                            let value: Result<Value, _> = match serde_json::from_str(&tmp_json) {
                                Ok(v) => Ok(v),
                                Err(e) => Err(e),
                            };

                            if !(l1.clone().trim() == "\"value\": -0.0"
                                && l2.trim() == "\"value\": 0.0"
                                || !value.unwrap().as_f64().is_some())
                            {
                                let line_diff = format!(
                                    "{}",
                                    DrawDiff::new(&l1, &l2, &ArrowsColorTheme::default())
                                );
                                panic!("Unexpected difference on line {}:\n{}", line, line_diff);
                            }
                        }
                    }
                }
                Err(e) => {
                    panic!("Failed to deserialize {:?}: {}", path, e);
                }
            }
        }
        false => {
            panic!("File does not exist at path: {:?}", path);
        }
    }
}

fn diff_strings(s1: String, s2: String) -> Vec<(usize, String, String)> {
    let lines1: Vec<&str> = s1.lines().collect();
    let lines2: Vec<&str> = s2.lines().collect();

    let mut map1: HashMap<usize, &str> = HashMap::new();
    let mut map2: HashMap<usize, &str> = HashMap::new();

    for (i, line) in lines1.iter().enumerate() {
        map1.insert(i, line);
    }

    for (i, line) in lines2.iter().enumerate() {
        map2.insert(i, line);
    }

    let max_lines = std::cmp::max(map1.len(), map2.len());

    let mut diffs = Vec::new();
    for i in 0..max_lines {
        let line1 = map1.get(&i);
        let line2 = map2.get(&i);
        match (line1, line2) {
            (Some(l1), Some(l2)) if l1 != l2 => diffs.push((i + 1, l1.to_string(), l2.to_string())),
            (Some(l1), None) => diffs.push((i + 1, l1.to_string(), "".to_string())),
            (None, Some(l2)) => diffs.push((i + 1, "".to_string(), l2.to_string())),
            _ => (),
        }
    }
    diffs
}

pub fn gather_files(module_path: &str) -> Vec<(String, String)> {
    let module_path = module_path.to_owned() + "/src";
    eprintln!("Module path: {}", module_path);
    let mut files: Vec<(String, String)> = Vec::new();

    for variant in VARIANTS {
        if Path::new(&module_path).is_dir() && Path::new(&module_path).join(variant).exists() {
            let variant_path = Path::new(&module_path).join(variant);
            // iterate all files in the variant folder
            for file in fs::read_dir(&variant_path).unwrap() {
                match &file {
                    Ok(file) => {
                        if file.clone().file_type().unwrap().is_dir() {
                            continue;
                        }
                        files.push((
                            variant.to_string(),
                            module_path.to_string()
                                + "/"
                                + variant
                                + "/"
                                + &file.clone().file_name().to_string_lossy().to_string(),
                        ));
                    }
                    Err(err) => {
                        println!("Error: {}", err);
                    }
                }
            }
        } else {
            for file in fs::read_dir(&module_path).unwrap() {
                match &file {
                    Ok(file) => {
                        if file.clone().file_type().unwrap().is_dir() {
                            continue;
                        }

                        let f = file.file_name().into_string().unwrap();
                        let parts: Vec<&str> = f.split('.').collect();
                        eprintln!("{:?} {}", parts, parts.len());
                        if parts.len() >= 3 {
                            let second_value = parts[1];

                            if second_value == variant {
                                let path = file.path();
                                let path = path.to_str().expect("Failed to convert path to string");
                                files.push((variant.to_string(), path.to_string()));
                            }
                        } else {
                            println!("Invalid input format {} {} {}", module_path, f, parts.len());
                        }
                    }
                    Err(err) => {
                        println!("Error: {}", err);
                    }
                }
            }
        }
    }
    files
}
