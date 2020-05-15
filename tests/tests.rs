#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/parse-fn.rs");
    t.compile_fail("tests/not-fn.rs");
}
