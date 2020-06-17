use crate::{globals::*, spawn_command};

// Execute a function associated with a key
// Either when a key is pressed or released
pub fn key_function(s: &str, mode: &str) {
    // w=white b=black
    // w1 means first white key
    // b4 means fourth black key

    match mode {
        // When key is pressed
        "on" => {
            match s {
                // White keys
                "w1" => spawn_command("wmctrl -s 0"),
                "w2" => spawn_command("wmctrl -s 1"),
                "w3" => spawn_command("wmctrl -s 2"),
                "w4" => spawn_command("wmctrl -s 3"),
                "w5" => {}
                "w6" => {}
                "w7" => {}
                "w8" => {}
                "w9" => {}
                "w10" => {}
                "w11" => {}
                "w12" => {}
                "w13" => {}
                "w14" => spawn_command("/home/yo/scripts/lockscreen.sh"),
                "w15" => spawn_command("/home/yo/scripts/lockscreen.sh lower"),

                // Black keys
                "b1" => spawn_command("/home/yo/scripts/aktion.sh maximize"),
                "b2" => spawn_command("/home/yo/scripts/aktion.sh minimize"),
                "b3" => spawn_command("/home/yo/scripts/aktion.sh tile_left"),
                "b4" => spawn_command("/home/yo/scripts/aktion.sh tile_right"),
                "b5" => spawn_command("/home/yo/scripts/aktion.sh next_screen"),
                "b6" => spawn_command("/home/yo/lab/cursor.sh up"),
                "b7" => spawn_command("/home/yo/lab/cursor.sh down"),
                "b8" => spawn_command("/home/yo/lab/cursor.sh left"),
                "b9" => {}
                "b10" => spawn_command("/home/yo/lab/cursor.sh right"),
                _ => {}
            }
        }
        // When key is released
        "off" => {
            match s {
                // White keys
                "w1" => {}
                "w2" => {}
                "w3" => {}
                "w4" => {}
                "w5" => {}
                "w6" => {}
                "w7" => {}
                "w8" => {}
                "w9" => {}
                "w10" => {}
                "w11" => {}
                "w12" => {}
                "w13" => {}
                "w14" => {}
                "w15" => {}

                // Black keys
                "b1" => {}
                "b2" => {}
                "b3" => {}
                "b4" => {}
                "b5" => {}
                "b6" => {}
                "b7" => {}
                "b8" => {}
                "b9" => {}
                "b10" => {}
                _ => {}
            }
        }
        _ => {}
    }
}

// Execute a function on pad event
pub fn pad_function(n: usize) {
    // 1 2 3 .. 8
    // 9 10 11 .. 16
    match n {
        // First row
        1 => spawn_command("ksysguard"),
        2 => spawn_command("kcalc"),
        3 => {}
        4 => {}
        5 => {}
        6 => {}
        7 => {}
        8 => {}

        // Second row
        9 => spawn_command("xdotool key XF86AudioPlay"),
        10 => spawn_command("dolphin"),
        11 => {}
        12 => {}
        13 => {}
        14 => {}
        15 => {}
        16 => {}
        _ => {}
    }
}

// Arrow button to the right of the apds
pub fn pad_top_arrow_function() {
    // spawn_command("~/scripts/audioswitch speakers");
}

// Arrow button to the right of the apds
pub fn pad_bottom_arrow_function() {
    // spawn_command("~/scripts/audioswitch headphones");
}

// Pitch bend
pub fn pitch_function(data: &str) {
    let n = data.parse::<isize>().unwrap();
    let direction = if n > 0 {
        1
    } else if n < 0 {
        2
    } else {
        0
    };
    g_set_scroll_direction(direction);
}

// First top row slider
pub fn first_knob_function(data: &str) {
    // Change volume
    let v: isize = ((data.parse::<f64>().unwrap() / 127.0) * 100.0) as isize;
    let cmd = format!("amixer -q -D pulse set Master {}%", v);
    println!("{}", cmd);
    spawn_command(&cmd);
}

// Stop button
pub fn stop_button_function() {
    spawn_command("systemctl suspend");
}

// Track left button
pub fn track_left_button_function() {
    spawn_command("xdotool key XF86AudioPrev");
}

// Track right button
pub fn track_right_button_function() {
    spawn_command("xdotool key XF86AudioNext");
}
