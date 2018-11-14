//! testing

use super::Buffer;

static BASE_STR: &str = "ABC";

pub fn check_compliance<B: Buffer>() {
    test_append::<B>();
    test_delete::<B>();
    test_insert::<B>();
    test_replace::<B>();
}

fn test_append<B: Buffer>() {
    let mut b = B::new(BASE_STR);
    b.append("DEF");
    assert_eq!(b.get_contents(), "ABCDEF");
}
 
fn test_delete<B: Buffer>() {
    let mut b = B::new(BASE_STR);
    b.delete(..2);
    assert_eq!(b.get_contents(), "C");
}

fn test_insert<B: Buffer>() {
    let mut b = B::new(BASE_STR);
    b.insert(2, "RA");
    assert_eq!(b.get_contents(), "ABRAC");
}

fn test_replace<B: Buffer>() {
    let mut b = B::new(BASE_STR);
    b.replace(1..2, "DHO");
    assert_eq!(b.get_contents(), "ADHOC");
}
