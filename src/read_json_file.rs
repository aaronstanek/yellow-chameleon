use json::{parse, JsonValue};
use std::fs::read_to_string;
use std::path::Path;

pub(crate) fn read_json_file(filepath: &str) -> Result<Option<JsonValue>, String> {
    if !(Path::new(filepath).exists()) {
        return Ok(None);
    }
    let file_contents = match read_to_string(filepath) {
        Err(_e) => return Err(format!("Unable to read {filepath} as UTF-8 file.")),
        Ok(s) => s,
    };
    let json_blob = match parse(&file_contents) {
        Err(_e) => {
            return Err(format!("{filepath} contains invalid JSON"));
        }
        Ok(j) => j,
    };
    return Ok(Some(json_blob));
}
