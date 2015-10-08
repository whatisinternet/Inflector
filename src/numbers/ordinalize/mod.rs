pub fn ordinalize<'a>(non_ordinalized_string: String) -> String {
    let chars: Vec<char>= non_ordinalized_string.clone().chars().collect();
    let last_number: char= chars[chars.len() - 1];
    if !last_number.is_numeric() {
        return non_ordinalized_string;
    }
    if chars.len() > 1 {
        let second_last_number: char= chars[chars.len() - 2];
        if second_last_number == '1'{
            return format!("{}{}", non_ordinalized_string, "th");
        }
    }
    match last_number {
        '1' => format!("{}{}", non_ordinalized_string, "st"),
        '2' => format!("{}{}", non_ordinalized_string, "nd"),
        '3' => format!("{}{}", non_ordinalized_string, "rd"),
        _ => format!("{}{}", non_ordinalized_string, "th"),
    }
}

#[cfg(test)]
mod ordinalize_test;
