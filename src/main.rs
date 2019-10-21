mod macros;
mod globals;

use globals::*;

use std::
{
    process::
    {
        Command, Stdio,
    },
    io::
    {
        BufReader, BufRead,
    },
    thread, time,
};

// Struct that holds data
// about a key/slider event

#[allow(dead_code)]
struct MidiEvent
{
    source: String,
    event_1: String,
    event_2: String,
    channel: String,
    label_1: String,
    data_1: String,
    label_2: String,
    data_2: String,
}

// Program starts here
fn main()
{
    turn_leds_off("both");
    
    thread::spawn(move ||
    {
        loop
        {
            resource_check();
            thread::sleep(time::Duration::from_secs(5));
        }
    });
    
    start_listener();
}

// Start listening to key or slider events
fn start_listener()
{
    let mut cmd = Command::new("aseqdump")
                    .arg("-p")
                    .arg("Launchkey MK2 25")
                    .stdout(Stdio::piped())
                    .spawn()
                    .unwrap();


    {
        let stdout = cmd.stdout.as_mut().unwrap();
        let stdout_reader = BufReader::new(stdout);
        let stdout_lines = stdout_reader.lines();

        for line in stdout_lines
        {
            let chunks: Vec<String> = line.unwrap()
                                .split_whitespace()
                                .map(|x| s!(x.replace(",", "").trim()))
                                .collect();

            if chunks[0] == s!("Waiting") || chunks[0] == s!("Source")
            {
                continue;
            }

            let event = MidiEvent
            {
                source: s!(chunks[0]),
                event_1: s!(chunks[1]),
                event_2: s!(chunks[2]),
                channel: s!(chunks[3]),
                label_1: s!(chunks[4]),
                data_1: s!(chunks[5]),
                label_2: s!(chunks[6]),
                data_2: s!(chunks[7]),
            };

            process_event(event);
        }
    }
}

// Detect and react to key or slider events
fn process_event(e: MidiEvent)
{
    if e.event_1 == s!("Note")
    {
        if e.event_2 == s!("off")
        {
            return;
        }

        match &e.channel[..]
        {
            // Piano keys
            "0" =>
            {
                match &e.data_1[..]
                {
                    // First white key
                    "50" =>
                    {
                        Command::new("wmctrl")
                            .arg("-s").arg("0").status()
                            .expect("wmctrl failed to run.");
                    },
                    // Second white key
                    "52" =>
                    {
                        Command::new("wmctrl")
                            .arg("-s").arg("1").status()
                            .expect("wmctrl failed to run.");
                    },
                    // Third white key
                    "54" =>
                    {
                        Command::new("wmctrl")
                            .arg("-s").arg("2").status()
                            .expect("wmctrl failed to run.");
                    },
                    // Fourth white key
                    "55" =>
                    {
                        Command::new("wmctrl")
                            .arg("-s").arg("3").status()
                            .expect("wmctrl failed to run.");
                    },
                    // First black key
                    "51" =>
                    {
                        Command::new("xdotool")
                            .arg("key").arg("Super_L+Ctrl+Left")
                            .status().expect("xdotool failed to run.");
                    },
                    // Second black key
                    "53" =>
                    {
                        Command::new("xdotool")
                            .arg("key").arg("Super_L+Ctrl+Right")
                            .status().expect("xdotool failed to run.");
                    },                    
                    _ => {}
                }
            },
            // Drum pads
            "9" =>
            {
                turn_leds_off("both");
                g_set_cpu_level(0);
                g_set_ram_level(0);
            },
            _ => {}
        }
    }
}

// Light up or turn off a led
fn change_led(n: u8, color: &str)
{
    Command::new("amidi")
        .arg("-p").arg("hw:2,0,1").arg("-S").arg("9F")
        .arg(g_get_pad(n)).arg(g_get_color(color))
        .status().expect("amidi failed to run.");
}

// Turn some or all leds off
fn turn_leds_off(mode: &str)
{
    if mode == "top" || mode == "both"
    {
        change_led_range(1, 8, "off");
    }

    if mode == "bottom" || mode == "both"
    {
        change_led_range(9, 16, "off");
    } 
}

// Change leds from a linear range
fn change_led_range(n1: u8, n2: u8, color: &str)
{
    for x in n1..=n2
    {
        change_led(x, color);
    }
}

// Get CPU and RAM info 
// and reflect it with the leds
fn resource_check()
{
    // Calculate and reflect CPU usage

    let cpu = psutil::cpu::cpu_percent(1.0).expect("Can't measure cpu usage.");
    
    if cpu > 90.0
    {
        if g_get_cpu_level() != 8
        {
            turn_leds_off("top");
            change_led_range(1, 8, "red");
            g_set_cpu_level(8);
        }
    }

    else if cpu > 80.0
    {
        if g_get_cpu_level() != 7
        {
            turn_leds_off("top");
            change_led_range(1, 7, "red");
            g_set_cpu_level(7);
        }
    }

    else if cpu > 70.0
    {
        if g_get_cpu_level() != 6
        {
            turn_leds_off("top");
            change_led_range(1, 6, "red");
            g_set_cpu_level(6);
        }
    }

    else if cpu > 60.0
    {
        if g_get_cpu_level() != 5
        {
            turn_leds_off("top");
            change_led_range(1, 5, "yellow");
            g_set_cpu_level(5);
        }
    }

    else if cpu > 50.0
    {
        if g_get_cpu_level() != 4
        {
            turn_leds_off("top");
            change_led_range(1, 4, "yellow");
            g_set_cpu_level(4);
        }
    }

    else if cpu > 40.0
    {
        if g_get_cpu_level() != 3
        {
            turn_leds_off("top");
            change_led_range(1, 3, "yellow");
            g_set_cpu_level(3);
        }
    }

    else if cpu > 30.0
    {
        if g_get_cpu_level() != 2
        {
            turn_leds_off("top");
            change_led_range(1, 2, "green");
            g_set_cpu_level(2);
        }
    }

    else
    {
        if g_get_cpu_level() != 1
        {
            turn_leds_off("top");
            change_led(1, "green");
            g_set_cpu_level(1);
        }
    }

    // Calculate and reflect RAM usage

    let ram = psutil::memory::virtual_memory().expect("Can't measure ram usage.").percent;

    if ram > 90.0
    {
        if g_get_ram_level() != 8
        {
            turn_leds_off("bottom");
            change_led_range(9, 16, "red");
            g_set_ram_level(8);
        }
    }

    else if ram > 80.0
    {
        if g_get_ram_level() != 7
        {
            turn_leds_off("bottom");
            change_led_range(9, 15, "red");
            g_set_ram_level(7);
        }
    }

    else if ram > 70.0
    {
        if g_get_ram_level() != 6
        {
            turn_leds_off("bottom");
            change_led_range(9, 14, "red");
            g_set_ram_level(6);
        }
    }

    else if ram > 60.0
    {
        if g_get_ram_level() != 5
        {
            turn_leds_off("bottom");
            change_led_range(9, 13, "yellow");
            g_set_ram_level(5);
        }
    }

    else if ram > 50.0
    {
        if g_get_ram_level() != 4
        {
            turn_leds_off("bottom");
            change_led_range(9, 12, "yellow");
            g_set_ram_level(4);
        }
    }

    else if ram > 40.0
    {
        if g_get_ram_level() != 3
        {
            turn_leds_off("bottom");
            change_led_range(9, 11, "yellow");
            g_set_ram_level(3);
        }
    }

    else if ram > 30.0
    {
        if g_get_ram_level() != 2
        {
            turn_leds_off("bottom");
            change_led_range(9, 10, "green");
            g_set_ram_level(2);
        }
    }

    else
    {
        if g_get_ram_level() != 1
        {
            turn_leds_off("bottom");
            change_led(9, "green");
            g_set_ram_level(1);
        }
    }
}