#[test]
fn test_empty() {
    // Sometimes under certain conditions, we may not have any constructor functions.
    // But the `invoke_ctors` function should still work, and the `__init_array_start` and
    // `__init_array_end` symbols should be valid.
    constructor_array::invoke_ctors();
    println!("It should exit successfully when we don't specify any constructor functions.");
}
