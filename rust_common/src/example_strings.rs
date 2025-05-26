use std::borrow::Cow;

pub fn reverse_string(input: &str) -> String {
    input.chars()
        .rev()
        .collect()
}

pub fn is_palimdrom(input: &str) -> bool {
    if input.is_empty() {
        return false;
    }

    let elements_count = (input.len() + 1) / 2;
    input.chars()
        .take(elements_count)
        .zip(input.chars()
            .rev()
            .take(elements_count)
        )
        .all(|(front, back)| front == back)
}

pub fn reverse_cow<'a>(input: &'a str) -> Cow<'a, str> {
    if is_palimdrom(input) {
        // return borrowed
        Cow::Borrowed(input)
    } else {
        let reversed = reverse_string(input);
        //return owned
        Cow::Owned(reversed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reversing_string_simple() {
        let input = "12345";
        let result = reverse_string(input);
        assert_eq!("54321", result);
    }

    #[test]
    fn test_palimdroness() {
        assert!(is_palimdrom("1234321"));
        assert!(is_palimdrom("1"));
        assert!(is_palimdrom("121"));
        assert!(!is_palimdrom("1233"));
    }
    
    #[test]
    fn test_reversing_string_cowed() {
        let result = reverse_cow("12321");
        assert!(matches!(result, Cow::Borrowed("12321")));
        
        let result = reverse_cow("12345");
        assert_eq!(result, Cow::<String>::Owned( "54321".to_string()));
    }
}