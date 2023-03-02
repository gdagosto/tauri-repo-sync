

#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::process::{Command};
use std::io::{self, Write};
use std::fs;



async fn run_async_command(actual_command: &str, cwd: Option<&str>) -> Option<String> {
  // Make sure that the path is actually a full path
  let cmd = Command::new("cmd").arg("/C").arg(actual_command);

  let final_command = if let Some(cwd) = cwd {
    let actual_path = fs::canonicalize(cwd).expect("Não foi possível processar o path");
    cmd.current_dir(actual_path)
  } else {
    cmd
  };

  let output = final_command.output().expect("não foi possível iniciar o processo");

  println!("status: {}", output.status);
  io::stdout().write_all(&output.stdout).unwrap();
  io::stderr().write_all(&output.stderr).unwrap();
  
  assert!(output.status.success());

  let resultado = String::from_utf8_lossy(&output.stdout);

  Some(resultado.to_string())
  
}



#[tauri::command]
async fn run_shell() -> Result<String, String>  {
  let result = run_async_command("echo hello").await;

  if let Some(message) = result {
    return Ok(String::from(message))
  } else {
    return Err("No result".into())
  }

  // let child = Command::new("cmd")
  //     .args(["/C", "echo hello"])
  //     .stdin(Stdio::piped())
  //     .stdout(Stdio::piped())
  //     .spawn().expect("ERRO DESCONHECIDO");

  
  // let output = child.wait_with_output();

  // println!("output = {:?}", output);


  // match output {
  //   Ok(_) => return String::from("SUCESSO"),
  //   Err(_) => return String::from("ERRO"),
  // }



}


fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![run_shell])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}