pub fn deordinalize<'a>(non_ordinalized_string: String) -> String {
    if non_ordinalized_string.contains(".")  {
        return non_ordinalized_string;
    } else {
        return non_ordinalized_string
            .trim_right_matches("st")
            .trim_right_matches("nd")
            .trim_right_matches("rd")
            .trim_right_matches("th")
            .to_string();
    }
}

#[cfg(test)]
mod deordinalize_test;
