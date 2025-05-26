use std::ops::Deref;

struct A {
    value: u32,
}

impl Deref for A {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

fn unpack_refs<T, U>(value: &T) -> U
where 
    T: Deref<Target = U>,
    U: Copy
{
    *value.deref()
}

fn sum_refs<A, B, U>(left: &A, right: &B) -> U
where 
    A: Deref<Target = U>,
    B: Deref<Target = U>,
    U: std::ops::Add<Output = U> + Copy
{
    *left.deref() + *right.deref()
}

#[test]
fn test_sth() {
    let a_value = 3;
    let a = A {value: a_value};
    let a_value_unpacked = unpack_refs(&a);
    assert_eq!(a_value, a_value_unpacked);
    
    let b_value = 3;
    let b = Box::new(b_value);
    let b_value_unpacked = unpack_refs(&b);
    assert_eq!(b_value, b_value_unpacked);

    let sum_value = sum_refs(&a, &b);
    assert_eq!(sum_value, a_value + b_value);
}