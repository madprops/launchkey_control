use crate::
{
    s, debug,
    sleep,
    spawn_command,
    globals::*,
    listeners::*,
    functions::*,
};

use std::
{
    thread,
};

// Detect and react to key or slider events
pub fn process_midi_event(e: MidiEvent)
{
    if !g_get_ready() {return}

    match &e.event_1[..]
    {
        "Note" =>
        {
            match &e.channel[..]
            {
                // Keys
                "0" => 
                {
                    let pos = get_key_position(&e.data_1);

                    match &e.event_2[..]
                    {
                        // Press
                        "on" =>
                        {
                            debug(&format!("Key {} on", &e.data_1));
                            g_set_key_state(&pos, true);
                            key_function(&pos, "on");
                        },
                        // Release
                        "off" =>
                        {
                            debug(&format!("Key {} off", &e.data_1));
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
                        "on" => 
                        {
                            debug(&format!("Pad {} on", &e.data_1));

                            match &e.data_1[..]
                            {
                                // Top arrow button
                                "104" => pad_top_arrow_function(),
                                // Bottom arrow button
                                "120" => pad_bottom_arrow_function(),
                                // Normal pads
                                _ => pad_function(pos)
                            }
                        }
                        // Release
                        "off" =>
                        {
                            debug(&format!("Pad {} off", &e.data_1));
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        },
        // Pitch bend
        "Pitch" =>
        {
            debug(&format!("Pitch {}", &e.data_1));
            pitch_function(&e.data_1);
        },
        // Other controls
        "Control" =>
        {
            debug(&format!("Control {} {}", &e.data_1, &e.data_2));

            match &e.data_1[..]
            {
                // Curved right slider
                "1" => mod_slider_function(&e.data_2),
                // Linear slider
                "7" => {},
                // Stop button
                "114" => 
                {
                    // Press
                    if e.data_2 == "127"
                    {
                        stop_button_function();
                    }
                },
                // Track left
                "102" => 
                {
                    // Press
                    if e.data_2 == "127"
                    {
                        track_left_button_function();
                    }
                },
                // Track right
                "103" =>
                {
                    // Press
                    if e.data_2 == "127"
                    {
                        track_right_button_function();
                    }
                },
                _ => {}
            }
        },
        _ => {}
    }
}

// Gets the key positon
// i.e note 48 -> w1
pub fn get_key_position(note: &str) -> String
{
    let fst = g_get_first_key();
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

// Thread that starts a check
// to see if the system should scroll
pub fn start_scroll_check()
{
    thread::spawn(move || 
    {   
        loop
        {
            let direction = g_get_scroll_direction();
            if direction == 1 {spawn_command("xdotool click 4")}
            else if direction == 2 {spawn_command("xdotool click 5")}

            sleep(g_get_scroll_delay());
        }
    });
}

// Get the position of a drum pad
// Order goes from top left -> top right
// then bottom left -> bottom right
fn get_pad_position(n: usize) -> usize
{
    let fst = g_get_first_pad();

    if n == fst {1}
    else if n == fst + 1 {2}
    else if n == fst + 2 {3}
    else if n == fst + 3 {4}
    else if n == fst + 4 {5}
    else if n == fst + 5 {6}
    else if n == fst + 6 {7}
    else if n == fst + 7 {8}
    else if n == fst + 16 {9}
    else if n == fst + 17 {10}
    else if n == fst + 18 {11}
    else if n == fst + 19 {12}
    else if n == fst + 20 {13}
    else if n == fst + 21 {14}
    else if n == fst + 22 {15}
    else if n == fst + 23 {16}
    else {0}
}