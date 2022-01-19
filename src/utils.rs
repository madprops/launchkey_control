use crate::{globals::*, p};

use std::{
  process::Command
};

// 127 value to percentage (100)
pub fn get_percentage(data: &str) -> isize {
  ((data.parse::<f64>().unwrap() / 127.0) * 100.0) as isize
}

// Sets the controller to extended or basic mode
pub fn switch_mode(mode: &str) {
  match mode {
      "basic" => midi_signal("9F 0C 00"),
      "extended" => midi_signal("9F 0C 7F"),
      _ => {}
  }
}

// Sends a midi signal
pub fn midi_signal(hex: &str) {
  debug(&format!("Signal: {}", hex));
  run_command(&format!("amidi -p {} -S {}", g_get_midi_port_2_b(), hex));
}

// Function used for debugging information
pub fn debug(s: &str) {
  if g_get_debug() {
      p!(s);
  }
}

// Runs a command
pub fn run_command(cmd: &str) {
  Command::new("sh")
      .arg("-c")
      .arg(cmd)
      .status()
      .expect("Can't run command.");
}

// Spawn command
pub fn spawn_command(cmd: &str) {
  Command::new("bash")
      .arg("-c")
      .arg(cmd)
      .spawn()
      .expect("Can't spawn command.");
}

// Spawn command and get output
pub fn command_output(cmd: &str) -> String {
  let o = Command::new("bash")
      .arg("-c")
      .arg(cmd)
      .output()
      .expect("Can't get command output.");

  String::from_utf8_lossy(&o.stdout).to_string()
}