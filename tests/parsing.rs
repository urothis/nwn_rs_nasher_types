use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;
use std::fs;
use termdiff::{ArrowsColorTheme, DrawDiff};

use nwn_nasher_types::*;

#[test_each::file(glob = "assets/testing/*.json", glob = "assets/testing/modules/howardslotr/src/**/*.json", name(segments = 3, index, extension))]
fn serialize(content: &str, path: PathBuf) {
  let nw: Result<NwType, _> = serde_json::from_str(content);
  match nw {
    Ok(_) => {}
    Err(e) => {
      panic!("Failed to deserialize {:?} {}", e, path.as_os_str().to_str().unwrap());
    }
  }
}

#[test_each::file(glob = "assets/testing/*.json", glob = "assets/testing/modules/**/src/**/*.json", name(segments = 3, index, extension))]
fn round_trip(content: &str, path: PathBuf) {
  let nw: Result<NwType, _> = serde_json::from_str(content);
  match nw {
    Ok(value) => {
      // serialize
      let serialized = match serde_json::to_string_pretty(&value) {
        Ok(s) => s + "\n",
        Err(e) => panic!("Failed to serialize: {}", e),
      };
      // get original data
      let original =
        fs::read_to_string(&path).expect("Failed to read file");

      if original != serialized {
        let diff = diff_strings(original.clone(), serialized.clone());

        // ignore the -0.0 to 0.0 differences
        for (line, l1, l2) in diff {
          if l1.ends_with("{") || l1.ends_with("}") {
            continue;
          }

          let mut tmp_json = l1.clone();
          if !l1.ends_with(",") {
            tmp_json = format!("{{{}}}", &l1.trim());
          } else {
            tmp_json = format!("{{{}}}", &l1.trim()[..l1.len() - 1]);
          }


          eprintln!("tmp_json: {}", tmp_json);
          let value: Result<Value, _> =
            match serde_json::from_str(&tmp_json) {
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
            eprintln!("line_diff: {}", line_diff);
            panic!(
              "Unexpected difference on line {}:\n{}",
              line, line_diff
            );
          }
        }
      }
    }
    Err(e) => {
      panic!("Failed to deserialize {:?}: {}", path, e);
    }
  }
}

const VARIANTS: [&str; 17] = [
  "are", "dlg", "fac", "gic", "git", "ifo", "itp", "jrl", "utc", "utd", "ute",
  "uti", "utm", "utp", "uts", "utt", "utw",
];

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
      (Some(l1), Some(l2)) if l1 != l2 => {
        diffs.push((i + 1, l1.to_string(), l2.to_string()))
      }
      (Some(l1), None) => diffs.push((i + 1, l1.to_string(), "".to_string())),
      (None, Some(l2)) => diffs.push((i + 1, "".to_string(), l2.to_string())),
      _ => (),
    }
  }
  diffs
}
