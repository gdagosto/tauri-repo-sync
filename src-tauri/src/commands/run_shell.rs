use crate::utils::run_async_command;

#[tauri::command]
pub async fn run_shell(command: &str, cwd: &str) -> Result<String, String> {
    let actual_cwd = if cwd == "" { None } else { Some(cwd) };

    let result = run_async_command(command, actual_cwd).await;

    if let Some(message) = result {
        return Ok(String::from(message));
    } else {
        return Err("No result".into());
    }
}
