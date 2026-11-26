//! Stage 10 — The Automated QBR (Quarterly Business Review).
//!
//! * `roi` — ROI model: hours saved × loaded rate + revenue lift + cost avoided − cost,
//!   ROI %, payback months, with a conservative case.
//! * `qbr` — quarter-over-quarter deltas against goals, wins and risks picked by rule,
//!   and a CSM slide outline with a bar-chart SVG per metric.

mod api;
pub mod qbr;
pub mod roi;
pub mod util;

pub use api::dispatch;
use serde_json::Value;

pub const OPS: &[&str] = &["qbr_deck", "roi_model"];

pub fn handle(op: &str, i: &Value) -> Result<Value, String> {
    match op {
        "qbr_deck" => qbr::deck_op(i),
        "roi_model" => roi::roi_op(i),
        _ => Err(format!("unknown op `{op}`; known: {}", OPS.join(", "))),
    }
}
