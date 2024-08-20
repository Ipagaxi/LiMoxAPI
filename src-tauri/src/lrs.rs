use tokio::process::Command;

pub fn start_lrs() {
    const PROGRAMM_PATH : &str = "/home/ipagaxi/Repos/Tauri/LRS/bin/run_sqlite.sh";

  let mut command = Command::new(PROGRAMM_PATH);
  let result = command.spawn(); // Await the future

  match result {
    Ok(_child) => {
      println!("Server started asynchronously in a separate shell.");
      // Optionally handle the child process (e.g., get its exit status)
    },
    Err(err) => {
      eprintln!("Error starting server: {}", err);
    }
  }
}