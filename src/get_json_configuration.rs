use crate::read_json_file::read_json_file;
use crate::sanitize_path::sanitize;
use json::JsonValue;

pub(crate) struct SourceConfiguration {
    pub ignore_list: Vec<String>,
}

pub(crate) struct DestinationConfiguration {
    pub lock_list: Vec<String>,
    pub path: Option<String>,
}

fn unwrap_array(
    mut output: Vec<String>,
    input: Vec<JsonValue>,
    key_name_on_error: &str,
    filename_on_error: &str,
) -> Result<Vec<String>, String> {
    for array_element in input.into_iter() {
        match array_element {
            JsonValue::String(s) => match sanitize(s) {
                None => {},
                Some(sanitized) => {
                    output.push(sanitized)
                }
            },
            JsonValue::Short(s) => match sanitize(String::from(s.as_str())) {
                None => {},
                Some(sanitized) => {
                    output.push(sanitized)
                }
            },
            _ => {
                return Err(format!("Expected sub-elements of {key_name_on_error} element of {filename_on_error} to be strings"))
            },
        }
    }
    Ok(output)
}

fn unwrap_object(
    mut output: Vec<String>,
    obj: &mut json::object::Object,
    key: &str,
    filename_on_error: &str,
) -> Result<Vec<String>, String> {
    match obj.remove(key) {
        None => Ok(output),
        Some(value_at_key) => match value_at_key {
            JsonValue::Array(array) => unwrap_array(output, array, key, filename_on_error),
            _ => Err(format!(
                "Expected {key} element of {filename_on_error} to be an array",
            )),
        },
    }
}

fn read_path_key(
    obj: &mut json::object::Object,
    filename_on_error: &str,
) -> Result<Option<String>, String> {
    match obj.remove("path") {
        None => Ok(None),
        Some(value_at_key) => match value_at_key {
            JsonValue::String(s) => Ok(sanitize(s)),
            JsonValue::Short(s) => Ok(sanitize(String::from(s.as_str()))),
            _ => Err(format!(
                "Expected path element of {filename_on_error} to be a string"
            )),
        },
    }
}

const SOURCE_CONFIG_FILE_NAME: &str = ".yellow-chameleon-source.json";

pub(crate) fn get_source_configuration(
    source_path: &String,
) -> Result<SourceConfiguration, String> {
    let read_path = format!("{source_path}/{SOURCE_CONFIG_FILE_NAME}");
    let json_blob_option = match read_json_file(&read_path) {
        Err(e) => return Err(e),
        Ok(j) => j,
    };
    let mut initial_ignore_list: Vec<String> =
        vec![String::from(".git"), String::from(SOURCE_CONFIG_FILE_NAME)];
    let final_ignore_list_result = match json_blob_option {
        None => Ok(initial_ignore_list),
        Some(json_blob) => match json_blob {
            JsonValue::Object(mut obj) => unwrap_object(
                initial_ignore_list,
                &mut obj,
                "ignore",
                SOURCE_CONFIG_FILE_NAME,
            ),
            _ => Err(format!(
                "Expected top-level element of {SOURCE_CONFIG_FILE_NAME} to be an object"
            )),
        },
    };
    match final_ignore_list_result {
        Err(e) => Err(e),
        Ok(list) => Ok(SourceConfiguration { ignore_list: list }),
    }
}

const DEST_CONFIG_FILE_NAME: &str = ".yellow-chameleon-destination.json";

pub(crate) fn get_destination_configuration() -> Result<DestinationConfiguration, String> {
    let json_blob_option =
        match read_json_file(format!("destination/{DEST_CONFIG_FILE_NAME}").as_str()) {
            Err(e) => return Err(e),
            Ok(j) => j,
        };
    let mut initial_lock_list: Vec<String> =
        vec![String::from(".git"), String::from(DEST_CONFIG_FILE_NAME)];
    match json_blob_option {
        None => Ok(DestinationConfiguration {
            lock_list: initial_lock_list,
            path: None,
        }),
        Some(json_blob) => match json_blob {
            JsonValue::Object(mut obj) => {
                let final_lock_list =
                    match unwrap_object(initial_lock_list, &mut obj, "lock", DEST_CONFIG_FILE_NAME)
                    {
                        Err(e) => return Err(e),
                        Ok(vec) => vec,
                    };
                let path = match read_path_key(&mut obj, DEST_CONFIG_FILE_NAME) {
                    Err(e) => return Err(e),
                    Ok(p) => p,
                };
                Ok(DestinationConfiguration {
                    lock_list: final_lock_list,
                    path: path,
                })
            }
            _ => Err(format!(
                "Expected top-level element of {DEST_CONFIG_FILE_NAME} to be an object"
            )),
        },
    }
}
