fn default_fn() -> i32 {
    return 42
}

#[test]
fn test_default_fn() {
    assert_eq!(default_fn(), 42);
}

