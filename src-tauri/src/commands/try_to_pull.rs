
use crate::{structs::RepositoryData, utils::run_async_command};

#[tauri::command]
pub async fn try_to_pull(data: RepositoryData) -> Result<String, String> {
    let cwd = &data.local_path;
    
    // Try running git pull

    // Check if current branch is the same as data.local_branch
    let check_branch_result = run_async_command("git branch --show-current", Some(&cwd)).await;
    
    if let None = check_branch_result {
        return Err(String::from("check branch result failed"))
    }
    
    // If the branches are different, try to checkout the proper branch
    if check_branch_result.unwrap() != data.local_branch {
        let command = format!("git checkout {}", &data.local_branch);
        let checkout_result = run_async_command(&command, Some(&cwd)).await;

        if let None = checkout_result {
            return Err(String::from("checkout failed"))
        };
    }

    // Check if the local branch is ancestor of remote branch
    let command = format!("git merge-base --is-ancestor {} {}",data.remote_branch(), &data.local_branch);
    let check_local_is_ancestor = run_async_command(&command, Some(&cwd)).await;

    if let None = check_local_is_ancestor {
        return Err(String::from("check if local is ancestor failed"))
    }

    // Try to pull. Here, we could also be up to date, but that makes no difference
    let command = format!("git pull {} {}", &data.remote_name, &data.remote_branch);
    let pull_result = run_async_command(&command, Some(&cwd)).await;

    if let None = pull_result {
        return Err(String::from("pull failed"))
    }

    // Run whatever commands you need to run after pulling



    return Ok(String::from("sucesso"));


}