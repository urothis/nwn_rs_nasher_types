use git2::Repository;
use serde_json::Value;
use std::collections::HashMap;
use std::{fs, path::Path};
use termdiff::{ArrowsColorTheme, DrawDiff};

use nwn_nasher_types::*;

#[test]
fn test_modules() {
    let test_start = std::time::Instant::now();

    let path = "assets/testing/modules/";

    let urls = vec![
        "https://github.com/b5635/the-frozen-north",
        "https://github.com/urothis/nwn-module-DungeonEternalX",
    ];

    // clone the repos
    for url in urls {
        // get the repo name
        let repo_name = match url.rsplit("/").next() {
            Some(name) => name,
            None => panic!("Failed to get repo name from url: {}", url),
        };
        // clone the repo
        match Repository::clone(url, path.to_owned() + repo_name) {
            Ok(repo) => repo,
            Err(_) => {
                // repo already exists
                continue;
            }
        };
    }

    // get the folder names
    let folder_names: Vec<String> = match fs::read_dir(path) {
        Ok(entries) => entries
            .filter_map(|entry| {
                entry.ok().and_then(|e| {
                    if e.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                        Some(e.file_name().to_string_lossy().to_string())
                    } else {
                        None
                    }
                })
            })
            .collect(),
        Err(err) => {
            eprintln!("Failed to read directory: {}", err);
            Vec::new()
        }
    };

    // files parsed
    let mut files_parsed: HashMap<String, Vec<(String, String)>> = HashMap::new();

    // test each module
    for folder_name in folder_names {
        let parsed = test_module(&(path.to_owned() + &folder_name));
        files_parsed.insert(folder_name, parsed);
    }

    // print the results
    for (module_name, parsed) in &files_parsed {
      eprintln!("Module: {}", module_name);
      for variant in VARIANTS.iter() {
        eprintln!("{}: {}", variant, parsed.iter().filter(|(v, _)| v == variant).count());
      }
    }

    // compare the time it took to run the tests
    let test_end = std::time::Instant::now();
    let test_duration = test_end.duration_since(test_start);
    println!("Test duration: {:?}", test_duration);
    panic!("Test Success");
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

fn test_module(module_name: &str) -> Vec<(String, String)> {
    let files = gather_files(module_name);

    for variant in VARIANTS {
        for (variant_, file) in files.iter().filter(|(v, _)| v == variant) {
            eprintln!("{} {}", variant_, file);
            serialize(file.to_string());
        }
    }

    files
}
