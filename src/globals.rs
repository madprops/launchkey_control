use crate::
{
    s, hashmap,
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
}

// Getters and setters for globals

// Returns a value from the pads hashmap
pub fn g_get_pad(n: usize) -> &'static str
{
    PADS.get(&n).unwrap()
}

// Returns a value from the colors hashmap
pub fn g_get_color(c: &str) -> &'static str
{
    COLORS.get(c).unwrap()
}

// Returns a value from the key state hashmap
#[allow(dead_code)]
pub fn g_get_key_state(s: &str) -> bool
{
    *KEY_STATE.lock().unwrap().get(s).unwrap()
}

// Set a value in the key state hashmap
pub fn g_set_key_state(s: &str, b: bool)
{
    KEY_STATE.lock().unwrap().insert(s!(s), b);
}

// Returns a value from the led color hashmap
pub fn g_get_led_color(n: usize) -> String
{
    s!(LED_COLOR.lock().unwrap().get(&n).unwrap())
}

// Set a value in the led color hashmap
pub fn g_set_led_color(n: usize, s: &str)
{
    LED_COLOR.lock().unwrap().insert(n, s!(s));
}

// Returns the cpu level global value
pub fn g_get_cpu_level() -> usize
{
    CPU_LEVEL.load(Ordering::SeqCst)
}

// Sets the cpu level global value
pub fn g_set_cpu_level(n: usize)
{
    CPU_LEVEL.store(n, Ordering::SeqCst)
}

// Returns the ram level global value
pub fn g_get_ram_level() -> usize
{
    RAM_LEVEL.load(Ordering::SeqCst)
}

// Sets the ram level global value
pub fn g_set_ram_level(n: usize)
{
    RAM_LEVEL.store(n, Ordering::SeqCst)
}

// Returns the scroll direction value
pub fn g_get_scroll_direction() -> usize
{
    SCROLL_DIRECTION.load(Ordering::SeqCst)
}

// Sets the scroll direction global value
pub fn g_set_scroll_direction(n: usize)
{
    SCROLL_DIRECTION.store(n, Ordering::SeqCst)
}

// Returns the ready value
pub fn g_get_ready() -> bool
{
    READY.load(Ordering::SeqCst)
}

// Sets the ready global value
pub fn g_set_ready(b: bool)
{
    READY.store(b, Ordering::SeqCst)
}

// Get the config struct
pub fn conf() -> &'static Config
{
    &CONFIG
}
