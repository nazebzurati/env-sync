pub fn parse_env_line(line: &str) -> Option<(String, String)> {
    // remove comments
    let mut content = line.trim();
    if let Some(index_hash) = content.find('#') {
        if let Some(index_quote) = content.find('"') {
            if index_hash < index_quote {
                content = &content[..index_hash];
            }
        } else {
            content = &content[..index_hash];
        }
    }

    if content.is_empty() {
        return None;
    }

    // split key and value
    let parts: Vec<&str> = content.split('=').collect();
    if parts.len() < 2 {
        return None;
    }

    let key = parts[0].trim();
    let val = parts[1..].join("");
    let val = val.trim();

    return Some((key.to_string(), val.to_string()));
}

#[cfg(test)]
mod tests {
    use crate::parser::parse_env_line;

    fn expect_value(input: &str, key: &str, val: &str) {
        let res = parse_env_line(input);
        assert!(res.is_some());
        if let Some((k, v)) = res {
            assert_eq!(k, key);
            assert_eq!(v, val);
        }
    }

    fn expect_none(input: &str) {
        let res = parse_env_line(input);
        assert!(res.is_none());
    }

    #[test]
    fn test_kv1() {
        expect_value("NAME=ENVF", "NAME", "ENVF");
    }

    #[test]
    fn test_ignore_comment1() {
        expect_none("#NAME=ENVF");
    }

    #[test]
    fn test_ignore_comment2() {
        expect_none("       #NAME=ENVF");
    }

    #[test]
    fn test_ignore_comment3() {
        expect_none("#        NAME=ENVF");
    }

    #[test]
    fn test_ignore_comment4() {
        expect_value("NAME = ENVF # this is a comment", "NAME", "ENVF");
    }

    #[test]
    fn test_ignore_comment5() {
        expect_value("NAME = ENVF#this is a comment", "NAME", "ENVF");
    }
}
