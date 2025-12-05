#![allow(dead_code)]
use std::collections::HashMap;
use anyhow::{anyhow, Result};

pub fn convert(amount: f64, from: &str, to: &str, base: &str, rates: &HashMap<String, f64>) -> Result<f64> {
    let from_u = from.to_uppercase();
    let to_u = to.to_uppercase();
    let base_u = base.to_uppercase();

    if base_u == from_u {
        let rate_to = rates.get(&to_u).ok_or_else(|| anyhow!("Unknown target currency: {}", to))?;
        return Ok(amount * rate_to);
    }

    if base_u == to_u {
        let rate_from = rates.get(&from_u).ok_or_else(|| anyhow!("Unknown source currency: {}", from))?;
        return Ok(amount / rate_from);
    }

    let rate_from = rates.get(&from_u).ok_or_else(|| anyhow!("Unknown source currency: {}", from))?;
    let rate_to = rates.get(&to_u).ok_or_else(|| anyhow!("Unknown target currency: {}", to))?;
    let in_base = amount / rate_from;
    Ok(in_base * rate_to)
}