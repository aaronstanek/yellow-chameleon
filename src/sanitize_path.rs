pub(crate) fn sanitize(s: String) -> Option<String> {
    let mut parts: Vec<&str> = Vec::new();
    for part in s.split("/") {
        match part {
            "" | "." | ".." => {}
            _ => parts.push(part),
        }
    }
    let sanitized = parts.join("/");
    if sanitized.len() > 0 {
        Some(sanitized)
    } else {
        None
    }
}
