use crate::
{
    s, run_command,
    config::*,
    globals::*,
    listeners::*,
};

use std::
{
    thread, time,
};

// Execute a function associated with a key
// Either when a key is pressed or released
pub fn key_function(s: &str, mode: &str)
{
    // w=white b=black
    // w1 means first white key
    // b4 means fourth black key

    match mode
    {
        // When key is pressed
        "on" =>
        {
            match s
            {
                // White keys
                "w1" => run_command("wmctrl -s 0"),
                "w2" => run_command("wmctrl -s 1"),
                "w3" => run_command("wmctrl -s 2"),
                "w4" => run_command("wmctrl -s 3"),
                "w5" => {},
                "w6" => {},
                "w7" => {},
                "w8" => {},
                "w9" => {},
                "w10" => {},
                "w11" => {},
                "w12" => {},
                "w13" => {},
                "w14" => {},
                "w15" => run_command("xdotool key Super_L+l"),

                // Black keys
                "b1" => run_command("xdotool key Super_L+Ctrl+Left"),
                "b2" => run_command("xdotool key Super_L+Ctrl+Right"),
                "b3" => run_command("xdotool keydown Super_R"),
                "b4" => run_command("xdotool keydown Shift"),
                "b5" => run_command("xdotool keydown Return"),
                "b6" => run_command("xdotool key Super_L+Ctrl+Shift+Left"),
                "b7" => run_command("xdotool key Super_L+Ctrl+Shift+Right"),
                "b8" => run_command("xdotool keydown Ctrl"),
                "b9" => run_command("xdotool keydown minus"),
                "b10" => run_command("xdotool keydown plus"),
                _ => {}
            }
        },
        // When key is released
        "off" =>
        {
            match s
            {
                // White keys
                "w1" => {},
                "w2" => {},
                "w3" => {},
                "w4" => {},
                "w5" => {},
                "w6" => {},
                "w7" => {},
                "w8" => {},
                "w9" => {},
                "w10" => {},
                "w11" => {},
                "w12" => {},
                "w13" => {},
                "w14" => {},
                "w15" => {},

                // Black keys
                "b1" => {},
                "b2" => {},
                "b3" => run_command("xdotool keyup Super_R"),
                "b4" => run_command("xdotool keyup Shift"),
                "b5" => run_command("xdotool keyup Return"),
                "b6" => {},
                "b7" => {},
                "b8" => run_command("xdotool keyup Ctrl"),
                "b9" => run_command("xdotool keyup minus"),
                "b10" => run_command("xdotool keyup plus"),
                _ => {}
            }
        },
        _ => {}
    }
}

// Gets the key positon
// i.e note 48 -> w1
pub fn get_key_position(note: String) -> String
{
    let fst = FIRST_KEY;
    let n = note.parse::<usize>().unwrap();

    // White keys
    if n == fst {s!("w1")}
    else if n == fst + 2 {s!("w2")}
    else if n == fst + 4 {s!("w3")}
    else if n == fst + 5 {s!("w4")}
    else if n == fst + 7 {s!("w5")}
    else if n == fst + 9 {s!("w6")}
    else if n == fst + 11 {s!("w7")}
    else if n == fst + 12 {s!("w8")}
    else if n == fst + 14 {s!("w9")}
    else if n == fst + 16 {s!("w10")}
    else if n == fst + 17 {s!("w11")}
    else if n == fst + 19 {s!("w12")}
    else if n == fst + 21 {s!("w13")}
    else if n == fst + 23 {s!("w14")}
    else if n == fst + 24 {s!("w15")}

    // Black keys
    else if n == fst + 1 {s!("b1")}
    else if n == fst + 3 {s!("b2")}
    else if n == fst + 6 {s!("b3")}
    else if n == fst + 8 {s!("b4")}
    else if n == fst + 10 {s!("b5")}
    else if n == fst + 13 {s!("b6")}
    else if n == fst + 15 {s!("b7")}
    else if n == fst + 18 {s!("b8")}
    else if n == fst + 20 {s!("b9")}
    else if n == fst + 22 {s!("b10")}

    else {s!("")}
}

// Detect and react to key or slider events
pub fn process_midi_event(e: MidiEvent)
{
    match &e.event_1[..]
    {
        "Note" =>
        {
            match &e.channel[..]
            {
                // Keys
                "0" => 
                {
                    let pos = get_key_position(e.data_1);

                    match &e.event_2[..]
                    {
                        // Press
                        "on" =>
                        {
                            g_set_key_state(&pos, true);
                            key_function(&pos, "on");
                        },
                        // Release
                        "off" =>
                        {
                            g_set_key_state(&pos, false);
                            key_function(&pos, "off");
                        },
                        _ => {}
                    }
                }
                // Drum pads
                "15" =>  
                {
                    let n = e.data_1.parse::<usize>().unwrap();
                    let pos = get_pad_position(n);

                    match &e.event_2[..]
                    {
                        // Press
                        "on" => pad_function(pos),
                        // Release
                        "off" => {},
                        _ => {}
                    }
                },
                _ => {}
            }
        },
        // Pitch bend
        "Pitch" =>
        {
            let n = e.data_1.parse::<isize>().unwrap();
            let direction = if n > 0 {1} else if n < 0 {2} else {0};
            g_set_scroll_direction(direction);
        },
        // Other controls
        "Control" =>
        {
            match &e.data_1[..]
            {
                // Curved right slider
                "1" =>
                {
                    // Change volume
                    let v:f64 = e.data_2.parse::<f64>().unwrap() / 127.0;
            
                    let cmd = format!("for sink in `pacmd list-sinks | grep 'index:' | cut -b12-`\n\
                    do\n\
                        pactl set-sink-volume $sink {:.*}\n\
                    done", 2, v);

                    run_command(&cmd);
                },
                // Linear slider
                "7" => {},
                // Stop button
                "114" => 
                {
                    // Press
                    if e.data_2 == "127"
                    {
                        run_command("systemctl suspend");
                    }
                },
                // Track left
                "102" => 
                {
                    // Press
                    if e.data_2 == "127"
                    {
                        run_command("xdotool key XF86AudioPrev");
                    }
                },
                // Track right
                "103" =>
                {
                    // Press
                    if e.data_2 == "127"
                    {
                        run_command("xdotool key XF86AudioNext");
                    }
                },
                _ => {}
            }
        },
        _ => {}
    }
}

// Thread that starts a check
// to see if the system should scroll
pub fn start_scroll_check()
{
    thread::spawn(move || 
    {   
        loop
        {
            let direction = g_get_scroll_direction();
            if direction == 1 {run_command("xdotool click 4")}
            else if direction == 2 {run_command("xdotool click 5")}

            thread::sleep(time::Duration::from_millis(SCROLL_DELAY));
        }
    });
}

// Get the position of a drum pad
// Order goes from top left -> top right
// then bottom left -> bottom right
fn get_pad_position(n: usize) -> usize
{
    let fst = FIRST_PAD;

    if n == fst {9}
    else if n == fst + 1 {10}
    else if n == fst + 2 {11}
    else if n == fst + 3 {12}
    else if n == fst + 4 {1}
    else if n == fst + 5 {2}
    else if n == fst + 6 {3}
    else if n == fst + 7 {4}
    else if n == fst + 8 {13}
    else if n == fst + 9 {14}
    else if n == fst + 10 {15}
    else if n == fst + 11 {16}
    else if n == fst + 12 {5}
    else if n == fst + 13 {6}
    else if n == fst + 14 {7}
    else if n == fst + 15 {8}
    else {0}
}

// Execute a function on pad event
fn pad_function(n: usize)
{
    // 1 2 3 .. 8
    // 9 10 11 .. 16
    match n
    {
        // First row
        1 => {},
        2 => {},
        3 => {},
        4 => {},
        5 => {},
        6 => {},
        7 => {},
        8 => {},

        // Second row
        9 => run_command("xdotool key XF86AudioPlay"),
        10 => {},
        11 => {},
        12 => {},
        13 => {},
        14 => {},
        15 => {},
        16 => {},
        _ => {}
    }
}