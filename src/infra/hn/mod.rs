use rayon::prelude::*;
use std::time::Duration;

use crate::domain::ListCategory;
use reqwest::{self, Client};

pub mod types;

#[cfg(not(test))]
fn get_url() -> &'static str {
    "https://hacker-news.firebaseio.com/v0"
}

#[cfg(test)]
fn get_url() -> String {
    mockito::server_url()
}

/// The API client.
#[derive(Clone)]
pub struct HnClient {
    client: Client,
}

impl ToString for ListCategory {
    fn to_string(&self) -> String {
        match self {
            Self::Top => "top".into(),
            Self::New => "new".into(),
            Self::Best => "best".into(),
            Self::Ask => "ask".into(),
            Self::Show => "show".into(),
            Self::Job => "job".into(),
        }
    }
}

impl HnClient {
    /// Create a new `` instance.
    pub fn init() -> reqwest::Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()?;
        Ok(Self { client })
    }

    /// Return the item with the specified id.
    ///
    /// May return `None` if item id is invalid.
    pub fn get_item(&self, id: u32) -> reqwest::Result<Option<types::Item>> {
        self.client
            .get(&format!("{}/item/{}.json", get_url(), id))
            .send()?
            .json()
    }

    /// Par get items
    pub fn get_items(&self, ids: Vec<u32>) -> anyhow::Result<Vec<types::Item>> {
        Ok(ids
            .into_par_iter()
            .filter_map(|id| self.get_item(id).ok().flatten())
            .collect::<Vec<types::Item>>())
    }

    /// Return the user with the specified username.
    ///
    /// May return `None` if username is invalid.
    pub fn get_user(&self, username: &str) -> reqwest::Result<Option<types::User>> {
        self.client
            .get(&format!("{}/user/{}.json", get_url(), username))
            .send()?
            .json()
    }

    /// Return the id of the newest item.
    ///
    /// To get the 10 latest items, you can decrement the id 10 times.
    pub fn get_max_item_id(&self) -> reqwest::Result<u32> {
        self.client
            .get(&format!("{}/maxitem.json", get_url()))
            .send()?
            .json()
    }

    /// Return a list of top story item ids.
    pub fn get_stories_list(&self, category: ListCategory) -> reqwest::Result<Vec<u32>> {
        let category = category.to_string();
        self.client
            .get(&format!("{}/{}stories.json", get_url(), category))
            .send()?
            .json()
    }

    /// Return a list of items and users that have been updated recently.
    pub fn get_updates(&self) -> reqwest::Result<types::Updates> {
        self.client
            .get(&format!("{}/updates.json", get_url()))
            .send()?
            .json()
    }
}
