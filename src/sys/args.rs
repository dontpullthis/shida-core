pub fn string_to_keyvalue(string: &String) -> Option<(String, String)> {
    let pos = match string.find("=") {
        Some(position) => position,
        None => return None,
    };
    Some((String::from(&string[..pos]), String::from(&string[pos+1..])))
}