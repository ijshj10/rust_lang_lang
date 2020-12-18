pub (crate) fn take_while(accept: impl Fn(char) -> bool, s: &str) -> (&str, &str) {
    let end = s
        .char_indices()
        .find_map(|(idx, c)| if accept(c) { None } else { Some(idx) })
        .unwrap_or_else(|| s.len());
    let extracted = &s[..end];
    let remainder = &s[end..];

    (remainder, extracted)
}

pub(crate) fn extract_digits(s: &str) -> (&str, &str) {
    take_while(|c| c.is_ascii_digit(), s)
}

pub(crate) fn extract_op(s: &str) -> (&str, &str) {
    match &s[..1] {
        "+" | "-" | "*" | "/" => (&s[1..], &s[..1]),
        _ => panic!("bad operator"),
    }
}

pub(crate) fn extract_whitespace(s: &str) -> (&str, &str) {
    take_while(|c| c == ' ', s)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn extarct_one_digit() {
        assert_eq!(extract_digits("1+2"), ("+2", "1"));
    }

    #[test]
    fn extract_multiple_digits() {
        assert_eq!(extract_digits("11+2"), ("+2", "11"));
    }

    #[test]
    fn extract_empty_digits() {
        assert_eq!(extract_digits(""), ("", ""));
    }

    #[test]
    fn extract_digits_without_remainder() {
        assert_eq!(extract_digits("100"), ("", "100"));
    }

    #[test]
    fn extract_plus() {
        assert_eq!(extract_op("+3"), ("3", "+"))
    }

    #[test]
    fn extract_minus() {
        assert_eq!(extract_op("-4"), ("4", "-"))
    }

    #[test]
    fn extract_start() {
        assert_eq!(extract_op("*14"), ("14", "*"))
    }

    #[test]
    fn extract_slash() {
        assert_eq!(extract_op("/3"), ("3", "/"))
    }

    #[test]
    fn extract_spaces() {
        assert_eq!(extract_whitespace("  3"), ("3", "  "));
    }
}
