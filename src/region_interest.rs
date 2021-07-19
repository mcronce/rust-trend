//! Represent Google Trend geo maps.
//!
//! See in which location your keyword was most popular during the specified time frame.
//! Values are calculated on a scale from 0 to 100, where 100 is the location with the most popularity as a fraction of total searches in that location, a value of 50 indicates a location which is half as popular.
//! A value of 0 indicates a location where there was not enough data for this term.

use crate::client::*;
use crate::request_handler::Query;
use serde_json::Value;

// Correpond to Multiline request => Google trend interest curve

#[derive(Debug, Clone)]
pub struct RegionInterest {
    pub client: Client,
}

impl RegionInterest {
    /// Create a `RegionInterest` Instance.
    /// 
    /// Returns a `RegionInterest` instance
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Retrieve maps data for all keywords.
    ///
    /// Retrieve data for all keywords set within the client.
    ///
    /// Returns a JSON serde Value (serde_json::Value).
    ///
    /// # Example
    /// ```rust
    /// # use rtrend::{Country, Keywords, Client, RegionInterest};
    /// let keywords = Keywords::new(vec!["hacker"]);
    /// let country = Country::new("US");
    /// let client = Client::new(keywords, country).build();
    /// 
    /// let region_interest = RegionInterest::new(client).get();
    /// 
    /// println!("{}", region_interest);
    /// ```
    ///
    /// # Panics
    /// Panic if the client have not been built.
    /// 
    /// ```rust,should_panic
    /// # use rtrend::{Country, Keywords, Client, region_interest::RegionInterest};
    /// let keywords = Keywords::new(vec!["hacker"]);
    /// let country = Country::new("US");
    /// 
    /// // Client not built
    /// let client = Client::new(keywords, country);
    /// 
    /// let region_interest = RegionInterest::new(client).get();
    /// ```
    pub fn get(&self) -> Value {
        self.send_request()[0].clone()
    }

    /// Retrieve maps data for a specific keywords.
    ///
    /// Retrieve the data for one keywords set within the client.
    ///
    /// Returns a JSON serde Value (serde_json::Value).
    ///
    /// # Example
    /// ```
    /// # use rtrend::{Country, Keywords, Client, region_interest::RegionInterest};
    /// let keywords = Keywords::new(vec!["PS4","XBOX","PC"]);
    /// let country = Country::new("ALL");
    /// let client = Client::new(keywords, country).build();
    /// 
    /// let region_interest = RegionInterest::new(client).get_for("PS4");
    /// 
    /// println!("{}", region_interest);
    /// ```
    ///
    /// # Panics
    /// Will panic if input keyword have not been set previously for the client.
    /// 
    /// ```should_panic
    /// # use rtrend::{Country, Keywords, Client, region_interest::RegionInterest};
    /// let keywords = Keywords::new(vec!["PS4","XBOX","PC"]);
    /// let country = Country::new("ALL");
    /// 
    /// let client = Client::new(keywords, country).build();
    /// 
    /// let region_interest = RegionInterest::new(client).get_for("WII");
    /// ```
    pub fn get_for(&self, keyword: &str) -> Value {
        let index = self
            .client
            .keywords
            .keywords
            .iter()
            .position(|&x| x == keyword);

        let keyword_index = match index {
            Some(k) => k,
            None => panic!("The keyword {} is not set with the client", keyword),
        };

        let response_index = keyword_index + 1;

        self.send_request()[response_index].clone()
    }
}
