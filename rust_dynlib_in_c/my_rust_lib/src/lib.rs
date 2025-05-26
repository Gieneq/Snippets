#[unsafe(no_mangle)]
pub extern "C" fn do_add(a: u32, b: u32) -> u32 {
    a + b
}

#[test]
fn test_do_add() {
    assert_eq!(do_add(1,2), 3);
    assert_ne!(do_add(5, 2), 3);
}