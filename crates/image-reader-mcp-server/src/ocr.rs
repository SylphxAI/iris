use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::process::Command;

/// OCR runs only when the caller names it. `include_ocr` wins over `profile`.
pub fn ocr_requested(args: &Value) -> bool {
    if let Some(flag) = args.get("include_ocr").and_then(Value::as_bool) {
        return flag;
    }
    args.get("profile").and_then(Value::as_str) == Some("quality")
}

pub fn run_opt_in_ocr(path: &str, args: &Value) -> Value {
    let languages = ocr_languages(args);
    let language_arg = languages.join("+");
    match Command::new("tesseract")
        .args([path, "stdout", "-l", &language_arg, "tsv", "--psm", "3"])
        .output()
    {
        Err(err) => gap(&languages, format!("Tesseract is not available ({err}).")),
        Ok(output) if !output.status.success() => {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            let reason = if stderr.is_empty() {
                format!("Tesseract exited with status {}.", output.status)
            } else {
                stderr
            };
            gap(&languages, reason)
        }
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let lines = parse_tesseract_tsv(&stdout);
            json!({
                "available": true,
                "route": "tesseract_tsv",
                "languages": languages,
                "line_count": lines.len(),
                "lines": lines,
            })
        }
    }
}

fn ocr_languages(args: &Value) -> Vec<String> {
    let parsed = args
        .get("ocr_languages")
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(Value::as_str)
                .filter(|lang| {
                    !lang.is_empty()
                        && lang
                            .chars()
                            .all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
                })
                .map(str::to_string)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    if parsed.is_empty() {
        vec!["eng".to_string()]
    } else {
        parsed
    }
}

fn gap(languages: &[String], reason: String) -> Value {
    json!({
        "available": false,
        "route": "tesseract_tsv",
        "languages": languages,
        "line_count": 0,
        "lines": [],
        "skipped_reason": reason,
    })
}

struct LineAcc {
    words: Vec<String>,
    min_x: u32,
    min_y: u32,
    max_right: u32,
    max_bottom: u32,
    confidence_sum: f64,
    confidence_count: u32,
}

/// Group Tesseract TSV word rows (level 5) into lines.
pub fn parse_tesseract_tsv(raw: &str) -> Vec<Value> {
    let mut rows = raw.lines().filter(|line| !line.trim().is_empty());
    let Some(header) = rows.next() else {
        return Vec::new();
    };
    let columns: BTreeMap<&str, usize> = header
        .split('\t')
        .enumerate()
        .map(|(index, name)| (name, index))
        .collect();
    let Some(&level_at) = columns.get("level") else {
        return Vec::new();
    };
    let required = [
        "block_num",
        "par_num",
        "line_num",
        "left",
        "top",
        "width",
        "height",
        "conf",
        "text",
    ];
    if required.iter().any(|name| !columns.contains_key(name)) {
        return Vec::new();
    }
    let mut grouped: BTreeMap<(u32, u32, u32), LineAcc> = BTreeMap::new();
    for row in rows {
        let cells: Vec<&str> = row.split('\t').collect();
        let cell = |name: &str| cells.get(*columns.get(name).unwrap()).copied().unwrap_or("");
        if cell("level") != "5" {
            continue;
        }
        let text = cell("text").trim();
        if text.is_empty() {
            continue;
        }
        let Ok(confidence) = cell("conf").parse::<f64>() else {
            continue;
        };
        if confidence < 0.0 {
            continue;
        }
        let (Ok(left), Ok(top), Ok(width), Ok(height)) = (
            cell("left").parse::<u32>(),
            cell("top").parse::<u32>(),
            cell("width").parse::<u32>(),
            cell("height").parse::<u32>(),
        ) else {
            continue;
        };
        let key = (
            cell("block_num").parse::<u32>().unwrap_or(0),
            cell("par_num").parse::<u32>().unwrap_or(0),
            cell("line_num").parse::<u32>().unwrap_or(0),
        );
        let entry = grouped.entry(key).or_insert_with(|| LineAcc {
            words: Vec::new(),
            min_x: left,
            min_y: top,
            max_right: left.saturating_add(width),
            max_bottom: top.saturating_add(height),
            confidence_sum: 0.0,
            confidence_count: 0,
        });
        entry.words.push(text.to_string());
        entry.min_x = entry.min_x.min(left);
        entry.min_y = entry.min_y.min(top);
        entry.max_right = entry.max_right.max(left.saturating_add(width));
        entry.max_bottom = entry.max_bottom.max(top.saturating_add(height));
        entry.confidence_sum += confidence;
        entry.confidence_count += 1;
        let _ = level_at;
    }
    grouped
        .into_values()
        .filter(|line| !line.words.is_empty())
        .map(|line| {
            json!({
                "text": line.words.join(" "),
                "bbox": {
                    "x": line.min_x,
                    "y": line.min_y,
                    "width": line.max_right.saturating_sub(line.min_x),
                    "height": line.max_bottom.saturating_sub(line.min_y),
                },
                "confidence": if line.confidence_count == 0 {
                    0.0
                } else {
                    line.confidence_sum / f64::from(line.confidence_count)
                },
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn groups_word_rows_into_one_line() {
        let raw = "\
level\tpage_num\tblock_num\tpar_num\tline_num\tword_num\tleft\ttop\twidth\theight\tconf\ttext
5\t1\t1\t1\t1\t1\t10\t20\t30\t12\t90\tHello
5\t1\t1\t1\t1\t2\t42\t20\t28\t12\t80\tworld
";
        let lines = parse_tesseract_tsv(raw);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0]["text"], "Hello world");
        assert_eq!(lines[0]["bbox"]["x"], 10);
        assert_eq!(lines[0]["bbox"]["width"], 60);
        assert_eq!(lines[0]["bbox"]["height"], 12);
        assert_eq!(lines[0]["confidence"], 85.0);
    }

    #[test]
    fn omitted_ocr_flag_stays_off_and_quality_opts_in() {
        assert!(!ocr_requested(&json!({ "path": "a.png" })));
        assert!(!ocr_requested(&json!({ "profile": "fast", "include_ocr": false })));
        assert!(ocr_requested(&json!({ "profile": "quality" })));
        assert!(ocr_requested(&json!({ "include_ocr": true })));
        assert!(!ocr_requested(&json!({ "profile": "quality", "include_ocr": false })));
    }
}
