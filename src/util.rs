//! Small JSON helpers shared by the modules. Missing or wrong-typed fields are errors.
use serde_json::Value;

pub type R<T> = Result<T, String>;

pub fn arr<'a>(v: &'a Value, k: &str) -> R<&'a Vec<Value>> {
    v.get(k).and_then(Value::as_array).ok_or_else(|| format!("`{k}` must be an array"))
}
pub fn obj<'a>(v: &'a Value, k: &str) -> R<&'a serde_json::Map<String, Value>> {
    v.get(k).and_then(Value::as_object).ok_or_else(|| format!("`{k}` must be an object"))
}
pub fn s<'a>(v: &'a Value, k: &str) -> R<&'a str> {
    v.get(k).and_then(Value::as_str).ok_or_else(|| format!("`{k}` must be a string"))
}
pub fn s_or<'a>(v: &'a Value, k: &str, d: &'a str) -> &'a str {
    v.get(k).and_then(Value::as_str).unwrap_or(d)
}
pub fn f(v: &Value, k: &str) -> R<f64> {
    let x = v.get(k).and_then(Value::as_f64).ok_or_else(|| format!("`{k}` must be a number"))?;
    if x.is_finite() { Ok(x) } else { Err(format!("`{k}` must be finite")) }
}
pub fn f_or(v: &Value, k: &str, d: f64) -> f64 {
    v.get(k).and_then(Value::as_f64).filter(|x| x.is_finite()).unwrap_or(d)
}
/// Round to `d` decimals for stable, readable output.
pub fn rd(x: f64, d: i32) -> f64 {
    let m = 10f64.powi(d);
    (x * m).round() / m
}
