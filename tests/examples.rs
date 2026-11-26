//! Every example on the demo page must run without error.
#[test]
fn every_demo_example_runs() {
    let raw = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/web/examples.json")).unwrap();
    let cfg: serde_json::Value = serde_json::from_str(&raw).unwrap();
    let ops = cfg["ops"].as_array().unwrap();
    assert!(!ops.is_empty());
    for ex in ops {
        let req = serde_json::json!({"op": ex["op"], "input": ex["input"]}).to_string();
        let out: serde_json::Value = serde_json::from_str(&gtm_10_auto_qbr::dispatch(&req)).unwrap();
        assert_eq!(out["ok"], true, "example {} failed: {}", ex["label"], out);
    }
    for o in gtm_10_auto_qbr::OPS {
        assert!(ops.iter().any(|e| e["op"] == *o), "op {o} has no demo example");
    }
}

#[test]
fn unknown_op_is_an_error() {
    let out: serde_json::Value =
        serde_json::from_str(&gtm_10_auto_qbr::dispatch(r#"{"op":"nope","input":{}}"#)).unwrap();
    assert_eq!(out["ok"], false);
}
