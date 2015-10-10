use cases::classcase::to_class_case;

pub fn deconstantize<'a>(non_deconstantized_string: String) -> String {
    if non_deconstantized_string.contains("::") {
        let split_string: Vec<&str> = non_deconstantized_string.split("::").collect();
        if split_string.len() > 1 {
            return format!("{}", to_class_case(split_string[split_string.len() - 2].to_string()));
        } else {
            return "".to_string();
        }
    } else {
        return "".to_string();
    }
}

#[cfg(test)]
mod deconstantize_test;
