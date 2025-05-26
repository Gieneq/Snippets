fn repeater<T: Clone>(n: usize, element: T) -> impl Iterator<Item = T> {
    std::iter::repeat_n(element, n)
}

#[test]
fn test_repeat_n() {
    let mut iterator = repeater(3, "abc");
    assert!(iterator.next().is_some());
    assert!(iterator.next().is_some());
    assert!(iterator.next().is_some());
    assert!(iterator.next().is_none());
}