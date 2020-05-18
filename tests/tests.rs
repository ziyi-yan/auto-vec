#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/parse-fn.rs");
    t.compile_fail("tests/not-fn.rs");
    t.pass("tests/vec-sig.rs");
    t.compile_fail("tests/not-non-return.rs");
}
