use auto_vec::auto_vec;

#[auto_vec]
pub fn foo(arg1: i64, arg2: i32) -> f64 {
    return (arg1 + arg2 as i64) as f64;
}

fn main() {
    let _: Vec<f64> = foo_vec(vec![1, 2], vec![3, 4]);
}
