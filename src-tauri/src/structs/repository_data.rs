
#[derive(serde::Deserialize)]
pub struct RepositoryData {
    pub local_path: String,
    pub local_branch: String,
    pub remote_name: String,
    pub remote_branch: String,
}
