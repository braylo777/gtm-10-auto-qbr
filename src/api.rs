//! Shared JSON entry point used by the CLI, the tests and the WASM demo page.
use serde_json::{json, Value};

/// Run one operation. Input: `{"op": "<name>", "input": {...}}`.
/// Output: `{"ok": true, "result": ...}` or `{"ok": false, "error": "..."}`.
pub fn dispatch(raw: &str) -> String {
    let parsed: Result<Value, _> = serde_json::from_str(raw);
    let out = match parsed {
        Err(e) => json!({"ok": false, "error": format!("bad JSON: {e}")}),
        Ok(v) => {
            let op = v.get("op").and_then(Value::as_str).unwrap_or("");
            let input = v.get("input").cloned().unwrap_or(Value::Null);
            match crate::handle(op, &input) {
                Ok(r) => json!({"ok": true, "result": r}),
                Err(e) => json!({"ok": false, "error": e}),
            }
        }
    };
    out.to_string()
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    #[no_mangle]
    pub extern "C" fn alloc(len: usize) -> *mut u8 {
        let mut v: Vec<u8> = Vec::with_capacity(len);
        let p = v.as_mut_ptr();
        std::mem::forget(v);
        p
    }
    /// # Safety
    /// `p` must come from `alloc` (cap `cap`) or `run` (len `cap`).
    #[no_mangle]
    pub unsafe extern "C" fn dealloc(p: *mut u8, cap: usize) {
        drop(Vec::from_raw_parts(p, 0, cap));
    }
    /// # Safety
    /// `p..p+len` must be a valid buffer written by the host.
    #[no_mangle]
    pub unsafe extern "C" fn run(p: *const u8, len: usize) -> u64 {
        let s = std::str::from_utf8(std::slice::from_raw_parts(p, len)).unwrap_or("");
        let out = super::dispatch(s).into_bytes().into_boxed_slice();
        let l = out.len();
        let ptr = Box::into_raw(out) as *mut u8;
        ((ptr as u64) << 32) | l as u64
    }
}
