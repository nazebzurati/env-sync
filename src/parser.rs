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
    let val = parts[1..].join("=");
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
    fn test_kv() {
        expect_value("NAME=ENVF", "NAME", "ENVF");
        expect_value("NAME=\"ENVF\"", "NAME", "\"ENVF\"");
        expect_value("NAME = \"ENVF#no\"", "NAME", "\"ENVF#no\"");
        expect_value("NAME = ", "NAME", "");
        expect_value("NAME=", "NAME", "");
        expect_value("NAME= ", "NAME", "");
        expect_value("NAME=# ", "NAME", "");
        expect_value("NAME= # ", "NAME", "");
        expect_value("NAME = \"ENVF#no=\"", "NAME", "\"ENVF#no=\"");
        expect_value("NAME = \"ENVF#no=comment\"", "NAME", "\"ENVF#no=comment\"");
        expect_value("NAME = \"=ENVF#no\"", "NAME", "\"=ENVF#no\"");
    }

    #[test]
    fn test_ignore_comment() {
        expect_none("#NAME=ENVF");
        expect_none("       #NAME=ENVF");
        expect_none("#        NAME=ENVF");
        expect_value("NAME = ENVF # this is a comment", "NAME", "ENVF");
        expect_value("NAME = ENVF#this is a comment", "NAME", "ENVF");
        expect_none("#NAME=");
        expect_none("#NAME= ");
    }
}
