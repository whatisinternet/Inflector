use cases::classcase::to_class_case;

pub fn demodulize<'a>(non_demodulize_string: String) -> String {
    if non_demodulize_string.contains("::") {
        let split_string: Vec<&str> = non_demodulize_string.split("::").collect();
        return format!("{}", to_class_case(split_string[split_string.len() - 1].to_string()));
    } else {
        return non_demodulize_string;
    }
}

#[cfg(test)]
mod demodulize_test;
