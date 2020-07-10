use anyhow::Result;
use reqwest::{blocking, Client};

impl crate::Request<crate::Base> {
    /// get a list of all breeds
    pub async fn async_list_breeds(&self) -> Result<String> {
        let resp: serde_json::Value = Client::new()
            .get(&self.list_breeds_url())
            .send()
            .await?
            .json()
            .await?;
        crate::json_as_result(resp)
    }

    /// get a list of all breeds
    pub fn list_breeds(&self) -> Result<String> {
        let resp: serde_json::Value = blocking::Client::new()
            .get(&self.list_breeds_url())
            .send()?
            .json()?
        crate::json_as_result(resp)
    }
}
