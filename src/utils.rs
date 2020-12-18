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

pub(crate) fn extract_ident(s: &str) -> (&str, &str) {
    let input_starts_with_alphabetic = s
        .chars()
        .next()
        .map(|c| c.is_ascii_alphabetic())
        .unwrap_or_else(|| false);
    
    if input_starts_with_alphabetic {
        take_while(|c| c.is_ascii_alphanumeric(), s)
    } else {
        (s, "")
    }
}

pub(crate) fn tag<'a, 'b>(starting_text: &'a str, s: &'b str) -> &'b str {
    if s.starts_with(starting_text) {
        let len = starting_text.len();
        &s[len..]
    } else {
        panic!("expected {}", starting_text);
    }
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

    #[test]
    fn cannot_extract_ident_beginning_with_number() {
        assert_eq!(extract_ident("123abc"), ("123abc", ""));
    }

    #[test]
    fn tag_word() {
        assert_eq!(tag("let", "let a"), " a");
    }
}
