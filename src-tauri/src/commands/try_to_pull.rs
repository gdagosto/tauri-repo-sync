use tauri::Window;

use crate::{structs::RepositoryData, utils::run_async_command};

use super::check_for_updates;

#[tauri::command]
pub async fn try_to_pull(
    data: RepositoryData,
    commands: Vec<String>,
    window: Window,
) -> Result<String, String> {
    let cwd = &data.local_path;

    // Try running git pull.
    // Even though we already checked if the branch is ok, contents might have changes since the last check.
    // Therefore, we should test it again, and make sure we can pull before pulling

    // Check if current branch is the same as data.local_branch
    let check_branch_result = run_async_command("git branch --show-current", Some(&cwd)).await;

    if let None = check_branch_result {
        return Err(String::from("check branch result failed"));
    }

    // If the branches are different, try to checkout the proper branch
    if check_branch_result.unwrap() != data.local_branch {
        let command = format!("git checkout {}", &data.local_branch);
        let checkout_result = run_async_command(&command, Some(&cwd)).await;

        if let None = checkout_result {
            return Err(String::from("checkout failed"));
        };
    }

    // Check if the local branch is ancestor of remote branch
    let should_pull = check_for_updates(data.clone()).await;

    if should_pull != Ok(String::from("shouldPull")) {
        return Err(String::from(
            "local branch can no longer pull from remote branch",
        ));
    }

    // Try to pull.
    let command = format!("git pull {} {}", &data.remote_name, &data.remote_branch);
    let pull_result = run_async_command(&command, Some(&cwd)).await;

    if let None = pull_result {
        return Err(String::from("pull failed"));
    }

    // Run whatever commands you need to run after pulling
    for command in commands {
        window
            .emit("try_to_pull_command", &command)
            .expect("couldn't emit to window");
        let command_result = run_async_command(&command, Some(&cwd)).await;

        if let None = command_result {
            return Err(String::from(format!("command {} failed", command)));
        }
    }

    return Ok(String::from("sucesso"));
}
