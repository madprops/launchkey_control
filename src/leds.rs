use crate::
{
    midi_signal,
    config::*,
    globals::*,
};

use std::
{
    thread, time,
};

// Light up or turn off a led
pub fn change_led(n: usize, color: &str, force: bool)
{
    if !force && color == g_get_led_color(n) {return}
    midi_signal(&format!("9F {} {}", g_get_pad(n), g_get_color(color)));
    g_set_led_color(n, color);
}

// Turn some or all leds off
pub fn turn_leds_off(mode: &str)
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
pub fn change_led_range(n1: usize, n2: usize, color: &str)
{
    for x in n1..=n2
    {
        change_led(x, color, false);
    }
}

// Get the proper led level
pub fn led_level(p: f32) -> usize
{
    if p >= 90.0 {8}
    else if p >= 80.0 {7}
    else if p >= 70.0 {6}
    else if p >= 60.0 {5}
    else if p >= 50.0 {4}
    else if p >= 40.0 {3}
    else if p >= 30.0 {2}
    else {1}
}

// Get the proper led color
pub fn led_color<'a>(n: usize) -> &'a str
{
    if n >= 6 {"red"}
    else if n >= 3 {"orange"}
    else {"green"}
}

// Get CPU and RAM info 
// and reflect it with the leds
pub fn update_leds()
{
    // Calculate and reflect CPU usage

    let cpu = psutil::cpu::cpu_percent(1.0).expect("Can't measure cpu usage.") as f32;
    let level = led_level(cpu);

    if g_get_cpu_level() != level
    {
        change_led_range(1, level, led_color(level));

        if level < 8
        {
            change_led_range(1 + level, 8, "off");
        }

        g_set_cpu_level(level);
    }

    // Calculate and reflect RAM usage

    let ram = psutil::memory::virtual_memory().expect("Can't measure ram usage.").percent;
    let level = led_level(ram);

    if g_get_ram_level() != level
    {
        change_led_range(9, 8 + level, led_color(level));

        if level < 8
        {
            change_led_range(9 + level, 16, "off");
        }

        g_set_ram_level(level);
    }
}

// Starts a thread to check leds
pub fn start_led_check()
{
    thread::spawn(move ||
    {
        loop
        {
            update_leds();
            thread::sleep(time::Duration::from_secs(LED_DELAY));
        }
    });
}