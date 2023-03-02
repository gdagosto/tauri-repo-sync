use crate::utils::run_async_command;
use crate::structs::RepositoryData;

#[tauri::command]
pub async fn check_for_updates(data: RepositoryData) -> Result<String, String> {
    let cwd = data.local_path;
    let local_branch = data.local_branch;
    let remote_branch = data.remote_name + "/" + &data.remote_branch;

    // Get local branch commit id
    let command = format!("git rev-parse {}", local_branch);
    let local_commit_id = run_async_command(&command, Some(&cwd));

    // Get remote branch commit id
    let command = format!("git rev-parse {}", remote_branch);
    let remote_commit_id = run_async_command(&command, Some(&cwd));

    // Get the common ancestor of both branches
    let command = format!("git merge-base {} {}", local_branch, remote_branch);
    let common_commit_id = run_async_command(&command, Some(&cwd));

    // Await until the commands finish
    let local_commit_id = local_commit_id.await;
    let remote_commit_id = remote_commit_id.await;
    let common_commit_id = common_commit_id.await;

    // If both branches are in the same commit, nothing to do
    if local_commit_id == remote_commit_id {
        return Ok(String::from("noUpdates"));
    }
    // If local branch is behind remote branch, let user know
    else if local_commit_id == common_commit_id {
        return Ok(String::from("shouldPull"));
    }
    // If local branch is ahead of remote branch, let user know
    else if remote_commit_id == common_commit_id {
        return Ok(String::from("shouldPush"));
    }
    // If local branch is ahead of remote branch, let user know
    else {
        return Ok(String::from("shouldMerge"));
    }
}
