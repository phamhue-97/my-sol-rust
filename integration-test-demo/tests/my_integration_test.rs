mod common;

#[test]
fn test_add() {
    assert_eq!(common::CASES.len(), 2);
    for (left, right, expected) in common::CASES {
        assert_eq!(integration_test_demo::add(left, right), expected);
    }
}
#[ignore]
#[test]
pub fn f() {
    println!("HELLOOOO?")
    // cargo test -- --show-output
    // cargo test -- --ignored
    // cargo test -- --include-ignored
}