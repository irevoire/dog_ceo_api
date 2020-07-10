use anyhow::Result;

impl crate::Request<crate::Base> {
    /// get a list of all breeds
    pub fn list_breeds(&self) -> Result<String> {
        let resp: serde_json::Value = ureq::get(&self.list_breeds_url()).call().into_json()?;
        crate::json_as_result(resp)
    }

    /// list all breeds and sub-breeds
    pub fn list_all_breeds(&self) -> Result<String> {
        let resp: serde_json::Value = ureq::get(&self.list_all_breeds_url()).call().into_json()?;
        crate::json_as_result(resp)
    }
}
