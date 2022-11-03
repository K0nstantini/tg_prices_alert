use std::collections::HashMap;
use std::env;
use std::str::FromStr;

use anyhow::{bail, Context, Result};
use rust_decimal::Decimal;

pub fn get_futures() -> Result<HashMap<String, Decimal>> {
    let mut futures = HashMap::new();

    let mut args = env::args();
    args.next();
    for s in args {
        match s.trim() {
            a if a.starts_with("c-") => {
                let v: Vec<_> = a.split("-").collect::<Vec<_>>().drain(1..).collect();

                let coin = v.first().context("Invalid format coin")?;
                let future = format!("{}-PERP", coin.to_uppercase());

                let price = v.last().context("Invalid format coin")?;
                let price = Decimal::from_str(price).context("Invalid format price")?;

                futures.insert(future, price);
            }
            "--release" => (),
            a => bail!("Unknown arg: {}", a)
        }
    }
    if futures.is_empty() {
        bail!("No futures in args!");
    }
    Ok(futures)
}
