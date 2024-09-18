pub fn is_letter(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '_')
}

pub fn is_digit(c: char) -> bool {
    matches!(c, '0'..='9')
}

pub fn is_str_char(c: char) -> bool {
    c != '\0' && c != '"'
}
