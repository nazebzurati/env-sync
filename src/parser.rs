fn remove_comment(mut line: &str) -> String {
    if let Some(index_hash) = line.find('#') {
        if let Some(index_quote) = line.find('"') {
            if index_hash < index_quote {
                line = &line[..index_hash];
            }
        } else {
            line = &line[..index_hash];
        }
    }

    String::from(line)
}

pub fn get_key_val(line: &str) -> Option<(String, String)> {
    // remove comments
    let content = remove_comment(line.trim());
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

    Some((key.to_string(), val.to_string()))
}

pub fn get_key(line: &str) -> Option<String> {
    // remove comments
    let content = remove_comment(line.trim());
    if content.is_empty() {
        return None;
    }

    // split key and value
    let parts: Vec<&str> = content.split('=').collect();
    if parts.is_empty() {
        return None;
    }

    let key = parts[0].trim();
    Some(key.to_string())
}

#[cfg(test)]
mod tests {
    use crate::parser::get_key_val;

    fn expect_value(input: &str, key: &str, val: &str) {
        let res = get_key_val(input);
        assert!(res.is_some());
        if let Some((k, v)) = res {
            assert_eq!(k, key);
            assert_eq!(v, val);
        }
    }

    fn expect_none(input: &str) {
        let res = get_key_val(input);
        assert!(res.is_none());
    }

    #[test]
    fn test_kv1() {
        expect_value("NAME=ENVF", "NAME", "ENVF");
    }

    #[test]
    fn test_kv2() {
        expect_value("NAME=\"ENVF\"", "NAME", "\"ENVF\"");
    }

    #[test]
    fn test_kv3() {
        expect_value(
            "NAME = \"ENVF#thisisnotacomment\"",
            "NAME",
            "\"ENVF#thisisnotacomment\"",
        );
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
