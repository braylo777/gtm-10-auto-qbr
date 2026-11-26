//! QBR deck. Each metric has last / this quarter, a goal and a direction (`higher` or
//! `lower` is better). Wins: met goal or improved ≥ `win_pct`. Risks: missed goal and got
//! worse. The outline is the slide list a CSM would present, in a fixed order.
use crate::util::*;
use serde_json::{json, Value};

pub fn bar_svg(label: &str, prev: f64, cur: f64, goal: Option<f64>) -> String {
    let max = [prev, cur, goal.unwrap_or(0.0)].iter().fold(0.0f64, |a, b| a.max(*b)).max(1e-9);
    let w = |x: f64| rd(x / max * 200.0, 1);
    let g = goal.map(|g| format!(r##"<line x1="{x}" x2="{x}" y1="4" y2="56" stroke="#b39b70" stroke-dasharray="3 3"/>"##, x = 60.0 + w(g))).unwrap_or_default();
    let esc = label.replace('&', "&amp;").replace('<', "&lt;").replace('"', "&quot;");
    format!(r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 300 60" role="img" aria-label="{esc}"><text x="0" y="20" font-size="10">Last Q</text><rect x="60" y="10" height="14" width="{}" fill="#8a94a6"/><text x="0" y="44" font-size="10">This Q</text><rect x="60" y="34" height="14" width="{}" fill="#2f6f9f"/>{g}</svg>"##, w(prev), w(cur))
}

pub fn deck_op(i: &Value) -> R<Value> {
    let cust = s(i, "customer")?;
    let q = s_or(i, "quarter", "this quarter");
    let win_pct = f_or(i, "win_pct", 10.0);
    let mut rows = vec![];
    let (mut wins, mut risks) = (vec![], vec![]);
    for m in arr(i, "metrics")? {
        let name = s(m, "metric")?;
        let (p, c) = (f(m, "last_q")?, f(m, "this_q")?);
        let goal = m.get("goal").and_then(Value::as_f64);
        let higher = s_or(m, "better", "higher") == "higher";
        let chg = if p != 0.0 { (c - p) / p.abs() * 100.0 } else { 0.0 };
        let improved = if higher { chg } else { -chg };
        let met = goal.map(|g| if higher { c >= g } else { c <= g });
        let unit = s_or(m, "unit", "");
        let line = format!("{name}: {p}{unit} → {c}{unit} ({:+.1}%){}", chg, goal.map(|g| format!(", goal {g}{unit}")).unwrap_or_default());
        if met == Some(true) || improved >= win_pct {
            wins.push(line.clone());
        } else if met == Some(false) && improved < 0.0 {
            risks.push(line.clone());
        }
        rows.push(json!({"metric": name, "last_q": p, "this_q": c, "change_pct": rd(chg, 1), "goal": goal, "goal_met": met,
            "status": if met == Some(true) { "on_track" } else if improved >= 0.0 { "improving" } else { "at_risk" }, "chart_svg": bar_svg(name, p, c, goal)}));
    }
    let next: Vec<Value> = i.get("next_quarter_goals").and_then(Value::as_array).cloned().unwrap_or_default();
    let mut slides = vec![
        json!({"n": 1, "title": format!("{cust} × Us — {q} Business Review"), "bullets": ["Agenda: results, value delivered, risks, next-quarter plan"]}),
        json!({"n": 2, "title": "Results vs goals", "bullets": rows.iter().map(|r| format!("{}: {} ({})", r["metric"].as_str().unwrap(), r["this_q"], r["status"].as_str().unwrap())).collect::<Vec<_>>()}),
        json!({"n": 3, "title": "Wins", "bullets": if wins.is_empty() { vec!["(none this quarter)".to_string()] } else { wins.clone() }}),
    ];
    if let Some(roi) = i.get("roi_headline").and_then(Value::as_str) {
        slides.push(json!({"n": 4, "title": "Value delivered", "bullets": [roi]}));
    }
    slides.push(json!({"n": slides.len() + 1, "title": "Risks and asks", "bullets": if risks.is_empty() { vec!["No metric missed goal and declined".to_string()] } else { risks.clone() }}));
    slides.push(json!({"n": slides.len() + 1, "title": "Next quarter", "bullets": next}));
    Ok(json!({"customer": cust, "quarter": q, "metrics": rows, "wins": wins, "risks": risks, "slides": slides}))
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn wins_risks_direction() {
        let i = json!({"customer":"C","metrics":[{"metric":"a","last_q":100,"this_q":120,"goal":150},
            {"metric":"tickets","last_q":10,"this_q":14,"goal":8,"better":"lower"},{"metric":"ttfr","last_q":10,"this_q":7,"goal":8,"better":"lower"}]});
        let r = deck_op(&i).unwrap();
        assert_eq!(r["wins"].as_array().unwrap().len(), 2);
        assert_eq!(r["risks"].as_array().unwrap().len(), 1);
        assert_eq!(r["metrics"][1]["status"], "at_risk");
        assert!(bar_svg("<x>", 1.0, 2.0, Some(3.0)).contains("&lt;x>"));
    }
}
