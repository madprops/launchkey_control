#![allow(clippy::cognitive_complexity)]

mod macros;
mod config;
mod globals;
mod listeners;
mod events;
mod functions;
mod leds;

use crate::
{
    globals::*,
    listeners::*,
    events::*,
    leds::*,
};

use std::
{
    time, thread,
    sync::
    {
        Arc,
        atomic::
        {
            AtomicBool, Ordering,
        },
    },
    process::Command,
};

// Program starts here
fn main()
{
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    while running.load(Ordering::SeqCst) 
    {
        switch_mode("extended");
        turn_leds_off("both");
        start_led_check();
        start_scroll_check();
        start_ready_countdown();
        start_ic_listener();
        start_main_listener();
    }

    cleanup();
}

// On termination
fn cleanup()
{
    turn_leds_off("both");
    switch_mode("basic");
}

// Runs a command
fn run_command(cmd: &str)
{
    Command::new("sh").arg("-c").arg(cmd)
        .status().expect("Can't run command.");
}

fn spawn_command(cmd: &str)
{
    Command::new("sh").arg("-c").arg(cmd)
        .spawn().expect("Can't spawn command.");
}

fn command_output(cmd: &str) -> String
{
    let o = Command::new("sh").arg("-c").arg(cmd)
        .output().expect("Can't get command output.");
    
    String::from_utf8_lossy(&o.stdout).to_string()
}

// Sends a midi signal
fn midi_signal(hex: &str)
{
    run_command(&format!("amidi -p {} -S {}", conf().midi_port_2_b, hex));
}

// Sets the controller to extended or basic mode
fn switch_mode(mode: &str)
{
    match mode
    {
        "basic" => midi_signal("9F 0C 00"),
        "extended" => midi_signal("9F 0C 7F"),
        _ => {}
    }
}

// Function used for debugging information
fn debug(s: &str)
{
    if conf().debug
    {
        p!(s);
    }
}

// Sets the program ready after some time
// This is necesarry because using buttons
// while the program is not running
// will add them to a queue that is
// executed right as the program starts
fn start_ready_countdown()
{
    thread::spawn(move || 
    {
        thread::sleep(time::Duration::from_millis(conf().ready_delay));
        g_set_ready(true);
        p!("Ready")
    });
}