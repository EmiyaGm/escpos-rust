pub struct Utils {}

impl Utils {
    pub fn separate_numbers_and_non_numbers(input: &str) -> Vec<String> {
        let mark = "!";
        let s = format!("{}{}", mark, input);
        let mut result = vec![];
    
        let mut temp_str = String::new();
        let mut prev_char_digit = s.chars().next().map_or(false, |c| c.is_digit(10));
        for c in s.chars().skip(1) {
            let is_digit = c.is_digit(10);
            if prev_char_digit == is_digit {
                temp_str.push(c);
            } else {
                result.push(temp_str.clone());
                temp_str.clear();
                temp_str.push(c);
                prev_char_digit = is_digit;
            }
        }
        result.push(temp_str);
        result
    }
}