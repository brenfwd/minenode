use napi_derive::napi;

#[napi(object)]
#[derive(Debug)]
pub struct AddResult {
    pub a: i32,
    pub b: i32,
    pub result: i32,
}

#[napi]
pub fn adds(a: i32, b: i32) -> AddResult {
    let res = minenode::add(a, b);
    AddResult { a, b, result: res }
}
