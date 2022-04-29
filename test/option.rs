#![feature(register_tool)]
#![register_tool(lr)]

// We need this right now to not deal with the enum `Some`
#[lr::assume]
fn some<T>(x: T) -> Option<T> {
    Option::Some(x)
}

#[lr::sig(fn(i32) -> i32{v:10 < v})]
pub fn test2(n: i32) -> i32 {
    if (0 < n) {
        let opt = some(n);
        opt.unwrap()
    } else {
        0
    }
}
