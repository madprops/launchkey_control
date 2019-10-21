use crate::
{
    hashmap,
};

use std::
{
    collections::HashMap,
    sync::atomic::
    {
        AtomicUsize, Ordering,
    },
};

use lazy_static::lazy_static;

// Global variables/constants
lazy_static!
{
    static ref PADS: HashMap<u8, &'static str> = hashmap!
    {
        1 => "28",
        2 => "29",
        3 => "2A",
        4 => "2B",
        5 => "30",
        6 => "31",
        7 => "32",
        8 => "33",
        9 => "24",
        10 => "25",
        11 => "26",
        12 => "27",
        13 => "2C",
        14 => "2D",
        15 => "2E",
        16 => "2F"
    };

    static ref COLORS: HashMap<&'static str, &'static str> = hashmap!
    {
        "red" => "6A",
        "orange" => "6B",
        "yellow" => "C",
        "green" => "4B",
        "off" => "00"
    };

    static ref CPU_LEVEL: AtomicUsize = AtomicUsize::new(0);
    static ref RAM_LEVEL: AtomicUsize = AtomicUsize::new(0);
}

// Getters and setters for globals

// Returns a value from the pads hashmap
pub fn g_get_pad(n: u8) -> &'static str
{
    PADS.get(&n).unwrap()
}

// Returns a value from the colors hashmap
pub fn g_get_color(c: &str) -> &'static str
{
    COLORS.get(c).unwrap()
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