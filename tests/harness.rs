#[test]
fn trybuild_basic_structs() {
    let t = trybuild::TestCases::new();
    t.pass("tests/basic_struct_tests/basic_struct_1.rs");
    t.pass("tests/basic_struct_tests/basic_struct_2.rs");
}

#[test]
fn trybuild_negative_examples() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/negative_examples/fail_union.rs");
}
