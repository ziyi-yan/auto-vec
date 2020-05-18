#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/parse-fn.rs");
    t.compile_fail("tests/not-fn.rs");
    t.pass("tests/vec-sig.rs");
    t.compile_fail("tests/not-non-return.rs");
    t.compile_fail("tests/not-non-inputs.rs");
}

use auto_vec::auto_vec;

#[auto_vec]
pub fn foo(arg1: i64, arg2: i32) -> f64 {
    return (arg1 + arg2 as i64) as f64;
}

#[test]
#[should_panic(expected = "inputs should have the same numbers of elements")]
fn eq_inputs_len() {
    let _ = foo_vec(vec![1, 2], vec![3, 4, 5]);
}
