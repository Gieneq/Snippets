fn count_characters<T: AsRef<str>>(input: T) -> usize {
    input.as_ref().chars().count()
}

#[test]
fn test_counting() {
    let a = "1234";
    assert_eq!(count_characters(a), 4);
    
    let b = String::from("1234");
    assert_eq!(count_characters(b), 4);
    
    let c = &String::from("1234");
    assert_eq!(count_characters(c), 4);
}