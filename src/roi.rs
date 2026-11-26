//! ROI = (value − cost) / cost. Value per month = hours saved × loaded hourly rate + revenue
//! lift × gross margin + cost avoided. Payback = one-time cost / net monthly value.
//! The conservative case applies `haircut_pct` to every value line.
use crate::util::*;
use serde_json::{json, Value};

fn case(i: &Value, haircut: f64) -> R<Value> {
    let months = f_or(i, "months", 12.0);
    if months <= 0.0 {
        return Err("months must be > 0".into());
    }
    let k = 1.0 - haircut;
    let hours = f_or(i, "hours_saved_per_month", 0.0) * k;
    let rate = f_or(i, "loaded_hourly_rate", 0.0);
    let lift = f_or(i, "revenue_lift_per_month", 0.0) * k;
    let margin = f_or(i, "gross_margin_pct", 100.0) / 100.0;
    let avoided = f_or(i, "cost_avoided_per_month", 0.0) * k;
    let monthly_value = hours * rate + lift * margin + avoided;
    let fee = f_or(i, "subscription_per_month", 0.0);
    let one = f_or(i, "one_time_cost", 0.0);
    let cost = fee * months + one;
    if cost <= 0.0 {
        return Err("total cost must be > 0".into());
    }
    let value = monthly_value * months;
    let net_monthly = monthly_value - fee;
    let payback = if net_monthly > 0.0 { Some(rd(one / net_monthly, 1)) } else { None };
    Ok(json!({"value_per_month": rd(monthly_value, 0), "value_total": rd(value, 0), "cost_total": rd(cost, 0),
        "net": rd(value - cost, 0), "roi_pct": rd((value - cost) / cost * 100.0, 1), "payback_months": payback,
        "lines": {"time_saved": rd(hours * rate, 0), "revenue_lift_margin": rd(lift * margin, 0), "cost_avoided": rd(avoided, 0)}}))
}

pub fn roi_op(i: &Value) -> R<Value> {
    let h = f_or(i, "haircut_pct", 30.0) / 100.0;
    if !(0.0..1.0).contains(&h) {
        return Err("haircut_pct must be in [0, 100)".into());
    }
    let base = case(i, 0.0)?;
    let cons = case(i, h)?;
    let headline = format!("Conservative: {}% ROI over {} months, payback {}.", cons["roi_pct"], f_or(i, "months", 12.0),
        cons["payback_months"].as_f64().map_or("not reached".to_string(), |p| format!("{p} mo")));
    Ok(json!({"base_case": base, "conservative_case": cons, "headline": headline}))
}

#[cfg(test)]
mod t {
    use super::*;
    #[test]
    fn roi_math() {
        let i = json!({"months":12,"hours_saved_per_month":10,"loaded_hourly_rate":100,"subscription_per_month":500,"one_time_cost":3000,"haircut_pct":50});
        let r = roi_op(&i).unwrap();
        assert_eq!(r["base_case"]["roi_pct"], 33.3); // 12000 vs 9000
        assert_eq!(r["base_case"]["payback_months"], 6.0);
        assert!(r["conservative_case"]["payback_months"].is_null()); // 500 value - 500 fee
        assert!(roi_op(&json!({"haircut_pct":100,"subscription_per_month":1})).is_err());
    }
}
