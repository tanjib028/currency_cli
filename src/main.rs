mod cli;
mod api;
mod config;

use anyhow::{Context, Result};
use clap::Parser;
use cli::{Cli, Commands, OutputFormat};
use api::RatesApi;

fn print_output(format: OutputFormat, value: &serde_json::Value) {
    match format {
        OutputFormat::Plain => {
            if let Some(s) = value.get("text").and_then(|t| t.as_str()) {
                println!("{}", s);
            } else {
                println!("{}", serde_json::to_string_pretty(value).unwrap());
            }
        }
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(value).unwrap());
        }
        OutputFormat::Csv => {
            if let Some(rows) = value.get("rows").and_then(|t| t.as_array()) {
                for row in rows {
                    if let Some(cols) = row.as_array() {
                        let line = cols
                            .iter()
                            .map(|c| c.as_str().map(str::to_owned).unwrap_or_else(|| c.to_string()))
                            .collect::<Vec<_>>()
                            .join(",");
                        println!("{}", line);
                    }
                }
            } else {
                println!("text");
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let output = cli.output;
    let api = RatesApi::new().context("Failed to create API client")?;

    match cli.command {
        Commands::Convert { amount, from, to, date } => {
            let response = if let Some(d) = date {
                let nd = chrono::NaiveDate::parse_from_str(&d, "%Y-%m-%d")?;
                api.historical_convert(amount, &from, &to, nd).await?
            } else {
                api.convert(amount, &from, &to).await?
            };

            let converted = response.result.unwrap_or(0.0);
            let text = format!(
                "{amount:.4} {} = {:.4} {}",
                from.to_uppercase(),
                converted,
                to.to_uppercase()
            );
            let json = serde_json::json!({
                "text": text,
                "amount": amount,
                "from": from.to_uppercase(),
                "to": to.to_uppercase(),
                "date": response.date.unwrap_or_default(),
                "result": converted
            });
            print_output(output, &json);
        }
        Commands::Multi { amount, from, tos, date } => {
            let mut rows = Vec::new();
            for to in tos {
                let response = if let Some(d) = &date {
                    let nd = chrono::NaiveDate::parse_from_str(d, "%Y-%m-%d")?;
                    api.historical_convert(amount, &from, &to, nd).await?
                } else {
                    api.convert(amount, &from, &to).await?
                };
                if let Some(v) = response.result {
                    rows.push(serde_json::json!([
                        from.to_uppercase(),
                        to.to_uppercase(),
                        amount,
                        v,
                        response.date.clone().unwrap_or_default()
                    ]));
                } else {
                    eprintln!("Skipping {to}: no result");
                }
            }
            let text = format!(
                "Converted {amount} {} into {} currencies",
                from.to_uppercase(),
                rows.len()
            );
            let json = serde_json::json!({ "text": text, "rows": rows });
            print_output(output, &json);
        }
        Commands::List { .. } => {
            // The RatesApi struct does not expose a `list_symbols` method in this crate,
            // so return a placeholder message instead of calling into the API.
            let text = "Listing symbols is not implemented for this API client.";
            let json = serde_json::json!({ "text": text });
            print_output(output, &json);
        }
        _ => {
            let text = "Update command is not supported with the /convert endpoint.";
            let json = serde_json::json!({ "text": text });
            print_output(output, &json);
        }
    }

    Ok(())
}