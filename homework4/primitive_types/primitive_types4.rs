// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `zustlings hint primitive_types4` for hints!!


fn main() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1..4];

    assert_eq!(&[2, 3, 4], nice_slice);

    println!("{:#?}", nice_slice);
}