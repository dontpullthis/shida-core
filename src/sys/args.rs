use std::collections::HashMap;

pub fn string_to_keyvalue(string: &String) -> Option<(String, String)> {
    let pos = match string.find("=") {
        Some(position) => position,
        None => return None,
    };
    Some((String::from(&string[..pos]), String::from(&string[pos+1..])))
}

pub fn hashmap_to_string_vec(hashmap: &HashMap<String, String>) -> Vec<String> {
    hashmap.iter().map(|(k, v)| format!("{}={}", k, v)).collect()
}