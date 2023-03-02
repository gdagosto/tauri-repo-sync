
use crate::structs::RepositoryData;

#[tauri::command]
pub async fn try_to_pull(data: RepositoryData) -> Result<String, String> {
    return Ok(String::from("ok"));
}