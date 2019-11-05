use crate::
{
    s, hashmap,
    command_output,
    config::*,
};

use std::
{
    collections::HashMap,
    sync::
    {
        Mutex,
        atomic::
        {
            AtomicUsize,
            AtomicBool,
            Ordering
        },
    },
};

use lazy_static::lazy_static;

// Global variables/constants
lazy_static!
{
    static ref PADS: HashMap<usize, &'static str> = hashmap!
    {
        1 => "60",
        2 => "61",
        3 => "62",
        4 => "63",
        5 => "64",
        6 => "65",
        7 => "66",
        8 => "67",
        9 => "70",
        10 => "71",
        11 => "72",
        12 => "73",
        13 => "74",
        14 => "75",
        15 => "76",
        16 => "77"
    };

    static ref COLORS: HashMap<&'static str, &'static str> = hashmap!
    {
        "red" => "6A",
        "orange" => "6B",
        "yellow" => "C",
        "green" => "4B",
        "off" => "00"
    };

    // Store if a key is pressed or not
    static ref KEY_STATE: Mutex<HashMap<String, bool>> = Mutex::new(hashmap!
    {
        // White keys
        s!("w1") => false,
        s!("w2") => false,
        s!("w3") => false,
        s!("w4") => false,
        s!("w5") => false,
        s!("w6") => false,
        s!("w7") => false,
        s!("w8") => false,
        s!("w9") => false,
        s!("w10") => false,
        s!("w11") => false,
        s!("w12") => false,
        s!("w13") => false,
        s!("w14") => false,
        s!("w15") => false,

        // Black keys
        s!("b1") => false,
        s!("b2") => false,
        s!("b3") => false,
        s!("b4") => false,
        s!("b5") => false,
        s!("b6") => false,
        s!("b7") => false,
        s!("b8") => false,
        s!("b9") => false,
        s!("b10") => false
    });

    // Store if a key is pressed or not
    static ref LED_COLOR: Mutex<HashMap<usize, String>> = Mutex::new(hashmap!
    {
        1 => s!("off"),
        2 => s!("off"),
        3 => s!("off"),
        4 => s!("off"),
        5 => s!("off"),
        6 => s!("off"),
        7 => s!("off"),
        8 => s!("off"),
        9 => s!("off"),
        10 => s!("off"),
        11 => s!("off"),
        12 => s!("off"),
        13 => s!("off"),
        14 => s!("off"),
        15 => s!("off"),
        16 => s!("off")
    });

    static ref CONFIG: Config = make_config();
    static ref CPU_LEVEL: AtomicUsize = AtomicUsize::new(0);
    static ref RAM_LEVEL: AtomicUsize = AtomicUsize::new(0);
    static ref SCROLL_DIRECTION: AtomicUsize = AtomicUsize::new(0);
    static ref READY: AtomicBool = AtomicBool::new(false);

    // CONFIG

    static ref CONF_MIDI_PORT_1: Mutex<String> = Mutex::new(command_output("aseqdump -l | grep \"Launchkey MK2 25 MIDI 1\" | awk '{$1=$1};1' \
                    | sed 's/ .*//' | tr -d '\n'"));
    static ref CONF_MIDI_PORT_2: Mutex<String> = Mutex::new(command_output("aseqdump -l | grep \"Launchkey MK2 25 MIDI 2\" | awk '{$1=$1};1' \
                    | sed 's/ .*//' | tr -d '\n'"));
    static ref CONF_MIDI_PORT_1_B: Mutex<String> = Mutex::new(command_output("amidi --list-devices | grep \"Launchkey MK2 25 MIDI 1\" \
                    | sed -n '/^IO/s/.*\\(hw[^ ]*\\).*/\\1/p' | tr -d '\n'"));
    static ref CONF_MIDI_PORT_2_B: Mutex<String> = Mutex::new(command_output("amidi --list-devices | grep \"Launchkey MK2 25 MIDI 2\" \
                    | sed -n '/^IO/s/.*\\(hw[^ ]*\\).*/\\1/p' | tr -d '\n'"));                        
    static ref CONF_FIRST_KEY: AtomicUsize = AtomicUsize::new(48);
    static ref CONF_FIRST_PAD: AtomicUsize = AtomicUsize::new(96);
    static ref CONF_SCROLL_DELAY: AtomicUsize = AtomicUsize::new(200);
    static ref CONF_LED_DELAY: AtomicUsize = AtomicUsize::new(5000);
    static ref CONF_READY_DELAY: AtomicUsize = AtomicUsize::new(2000);
    static ref CONF_DEBUG: AtomicBool = AtomicBool::new(true);
}

// Getters and setters for globals

pub fn g_get_pad(n: usize) -> &'static str
{
    PADS.get(&n).unwrap()
}

pub fn g_get_color(c: &str) -> &'static str
{
    COLORS.get(c).unwrap()
}

#[allow(dead_code)]
pub fn g_get_key_state(s: &str) -> bool
{
    *KEY_STATE.lock().unwrap().get(s).unwrap()
}

pub fn g_set_key_state(s: &str, b: bool)
{
    KEY_STATE.lock().unwrap().insert(s!(s), b);
}

pub fn g_get_led_color(n: usize) -> String
{
    s!(LED_COLOR.lock().unwrap().get(&n).unwrap())
}

pub fn g_set_led_color(n: usize, s: &str)
{
    LED_COLOR.lock().unwrap().insert(n, s!(s));
}

pub fn g_get_cpu_level() -> usize
{
    CPU_LEVEL.load(Ordering::SeqCst)
}

pub fn g_set_cpu_level(n: usize)
{
    CPU_LEVEL.store(n, Ordering::SeqCst)
}

pub fn g_get_ram_level() -> usize
{
    RAM_LEVEL.load(Ordering::SeqCst)
}

pub fn g_set_ram_level(n: usize)
{
    RAM_LEVEL.store(n, Ordering::SeqCst)
}

pub fn g_get_scroll_direction() -> usize
{
    SCROLL_DIRECTION.load(Ordering::SeqCst)
}

pub fn g_set_scroll_direction(n: usize)
{
    SCROLL_DIRECTION.store(n, Ordering::SeqCst)
}

pub fn g_get_ready() -> bool
{
    READY.load(Ordering::SeqCst)
}

pub fn g_set_ready(b: bool)
{
    READY.store(b, Ordering::SeqCst)
}

// Config Getters

pub fn g_get_midi_port_1() -> String
{
    s!(CONF_MIDI_PORT_1.lock().unwrap())
}

#[allow(dead_code)]
pub fn g_get_midi_port_1_b() -> String
{
    s!(CONF_MIDI_PORT_1_B.lock().unwrap())
}

pub fn g_get_midi_port_2() -> String
{
    s!(CONF_MIDI_PORT_2.lock().unwrap())
}


pub fn g_get_midi_port_2_b() -> String
{
    s!(CONF_MIDI_PORT_2_B.lock().unwrap())
}

pub fn g_get_first_key() -> usize
{
    CONF_FIRST_KEY.load(Ordering::SeqCst)
}

pub fn g_get_first_pad() -> usize
{
    CONF_FIRST_PAD.load(Ordering::SeqCst)
}

pub fn g_get_scroll_delay() -> usize
{
    CONF_SCROLL_DELAY.load(Ordering::SeqCst)
}

pub fn g_get_led_delay() -> usize
{
    CONF_LED_DELAY.load(Ordering::SeqCst)
}

pub fn g_get_ready_delay() -> usize
{
    CONF_READY_DELAY.load(Ordering::SeqCst)
}

pub fn g_get_debug() -> bool
{
    CONF_DEBUG.load(Ordering::SeqCst)
}