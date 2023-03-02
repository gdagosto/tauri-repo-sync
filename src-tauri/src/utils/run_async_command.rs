use crate::utils::normalize;
use std::process::Command;
use std::path::Path;

pub async fn run_async_command(actual_command: &str, cwd: Option<&str>) -> Option<String> {
    // Make sure that the path is actually a full path
    let mut cmd = Command::new("cmd");
    cmd.arg("/C");
    cmd.arg(actual_command);

    if let Some(cwd) = cwd {
        let actual_path = normalize::normalize_path(Path::new(cwd));
        cmd.current_dir(actual_path);
    }

    println!("{:?}", cmd.get_current_dir().expect("ERRO"));

    let output = cmd.output().expect("não foi possível iniciar o processo");

    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("status: {}", output.status);

    assert!(output.status.success());

    let resultado = String::from_utf8_lossy(&output.stdout);

    Some(resultado.to_string())
}