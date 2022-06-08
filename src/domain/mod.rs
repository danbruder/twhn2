use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use std::str::FromStr;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug, PartialEq, Clone)]
pub enum ListCategory {
    Top,
    New,
    Best,
    Ask,
    Show,
    Job,
}

#[derive(PartialEq, Debug, Clone)]
pub struct ItemRank {
    pub id: u32,
    pub rank: u32,
    pub category: ListCategory,
    pub ts: DateTime<Utc>,
}

impl FromStr for ListCategory {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self> {
        match input.to_lowercase().as_str() {
            "top" => Ok(Self::Top),
            "new" => Ok(Self::New),
            "best" => Ok(Self::Best),
            "ask" => Ok(Self::Ask),
            "show" => Ok(Self::Show),
            "job" => Ok(Self::Job),
            _ => Err(anyhow!("Invalid ListCategory")),
        }
    }
}
