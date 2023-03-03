#[derive(serde::Deserialize, Clone)]
pub struct RepositoryData {
    pub local_path: String,
    pub local_branch: String,
    pub remote_name: String,
    pub remote_branch: String,
}

impl RepositoryData {
    pub fn remote_branch(&self) -> String {
        let owned_str = String::from(&self.remote_name);
        owned_str + "/" + &self.remote_branch
    }
}
