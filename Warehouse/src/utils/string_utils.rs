
pub fn convert_to_lower_case(input: &str) -> String {
    input.to_ascii_lowercase()
}

pub fn convert_to_snake_case(input: &str) -> String {
    let mut result = String::new();
    let mut prev_char_was_upper = false;

    for (i, c) in input.chars().enumerate() {
        if c.is_whitespace() {
            result.push('_');
            prev_char_was_upper = false;
        } else if c.is_uppercase() {
            if i != 0 && !prev_char_was_upper {
                result.push('_');
            }
            result.push(c.to_ascii_lowercase());
            prev_char_was_upper = true;
        } else {
            result.push(c);
            prev_char_was_upper = false;
        }
    }

    result
}

pub fn clean_string_for_db(input: &str) -> String {
    convert_to_snake_case(input)
}

pub fn format_string_for_display(input: &str) -> String {
    let mut result = String::new();
    let mut prev_char_was_upper = false;

    for (i, c) in input.chars().enumerate() {
        if c == '_' {
            result.push(' ');
            prev_char_was_upper = false;
        } else if c.is_uppercase() {
            if i != 0 && !prev_char_was_upper {
                result.push(' ');
            }
            result.push(c);
            prev_char_was_upper = true;
        } else {
            result.push(c);
            prev_char_was_upper = false;
        }
    }

    result
}

pub fn format_string_for_display_with_capitalization(input: &str) -> String {
    let formatted = format_string_for_display(input);
    let mut result = String::new();
    let mut capitalize_next = true;

    for c in formatted.chars() {
        if c == ' ' {
            result.push(c);
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }

    result
}