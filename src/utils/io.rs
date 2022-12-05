use std::{fs::read_to_string, str::FromStr};

use crate::utils::date::Date;
use anyhow::Result;

pub fn read_input<T>(date: Date) -> Result<Vec<T>>
where
    T: FromStr,
{
    let path = format!("input/{}.txt", date);

    Ok(read_to_string(path)?
        .lines()
        .filter_map(|line| line.parse::<T>().ok())
        .collect())
}
