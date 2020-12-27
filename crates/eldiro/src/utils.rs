pub(crate) fn take_while(accept: impl Fn(char) -> bool, s: &str) -> (&str, &str) {
    let end = s
        .char_indices()
        .find_map(|(idx, c)| if accept(c) { None } else { Some(idx) })
        .unwrap_or_else(|| s.len());
    let extracted = &s[..end];
    let remainder = &s[end..];

    (remainder, extracted)
}

pub(crate) fn take_while1(
    accept: impl Fn(char) -> bool,
    s: &str,
    error_message: String,
) -> Result<(&str, &str), String> {
    let (remainder, extracted) = take_while(accept, s);
    match extracted.len() {
        0 => Err(error_message),
        _ => Ok((remainder, extracted)),
    }
}

pub(crate) fn extract_digits(s: &str) -> Result<(&str, &str), String> {
    take_while1(|c| c.is_ascii_digit(), s, "expected digits".to_owned())
}

pub(crate) fn extract_op(s: &str) -> (&str, &str) {
    match &s[..1] {
        "+" | "-" | "*" | "/" => (&s[1..], &s[..1]),
        _ => panic!("bad operator"),
    }
}

const WHITESPACE: &[char] = &[' ', '\n'];

pub(crate) fn extract_whitespace(s: &str) -> (&str, &str) {
    take_while(|c| WHITESPACE.contains(&c), s)
}

pub(crate) fn extract_whitespace1(s: &str) -> Result<(&str, &str), String> {
    take_while1(
        |c| WHITESPACE.contains(&c),
        s,
        "expected whitespace".to_owned(),
    )
}

pub(crate) fn extract_ident(s: &str) -> Result<(&str, &str), String> {
    let input_starts_with_alphabetic = s
        .chars()
        .next()
        .map(|c| c.is_ascii_alphabetic())
        .unwrap_or_else(|| false);

    if input_starts_with_alphabetic {
        Ok(take_while(|c| c.is_ascii_alphanumeric(), s))
    } else {
        Err("expected identifier".to_owned())
    }
}

pub(crate) fn tag<'a, 'b>(starting_text: &'a str, s: &'b str) -> Result<&'b str, String> {
    if s.starts_with(starting_text) {
        let len = starting_text.len();
        Ok(&s[len..])
    } else {
        Err(format!("expected {}", starting_text))
    }
}

pub(crate) fn sequence<T>(
    parser: impl Fn(&str) -> Result<(&str, T), String>,
    separator_parser: impl Fn(&str) -> (&str, &str),
    mut s: &str,
) -> Result<(&str, Vec<T>), String> {
    let mut items = vec![];

    while let Ok((new_s, item)) = parser(s) {
        s = new_s;
        items.push(item);

        let (new_s, _) = separator_parser(s);
        s = new_s;
    }

    Ok((s, items))
}

pub(crate) fn sequence1<T>(
    parser: impl Fn(&str) -> Result<(&str, T), String>,
    separator_parser: impl Fn(&str) -> (&str, &str),
    s: &str,
) -> Result<(&str, Vec<T>), String> {
    let (s, sequence) = sequence(parser, separator_parser, s)?;

    if sequence.is_empty() {
        Err("expected a sequence with more than one itme".to_owned())
    } else {
        Ok((s, sequence))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn extarct_one_digit() {
        assert_eq!(extract_digits("1+2"), Ok(("+2", "1")));
    }

    #[test]
    fn extract_multiple_digits() {
        assert_eq!(extract_digits("11+2"), Ok(("+2", "11")));
    }

    #[test]
    fn extract_empty_digits() {
        assert_eq!(extract_digits(""), Err("expected digits".to_owned()));
    }

    #[test]
    fn extract_digits_without_remainder() {
        assert_eq!(extract_digits("100"), Ok(("", "100")));
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
    fn do_not_extract_space1_when_input_does_not_starts_with_spaces() {
        assert_eq!(
            extract_whitespace1("123"),
            Err("expected whitespace".to_owned())
        )
    }

    #[test]
    fn cannot_extract_ident_beginning_with_number() {
        assert_eq!(
            extract_ident("123abc"),
            Err("expected identifier".to_owned())
        );
    }

    #[test]
    fn extract_alphanumeric_ident() {
        assert_eq!(extract_ident("foobar()"), Ok(("()", "foobar")));
    }

    #[test]
    fn tag_word() {
        assert_eq!(tag("let", "let a"), Ok(" a"));
    }
}
