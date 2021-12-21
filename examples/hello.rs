use std::{ffi::CStr, panic};

use ibus_dl::IBUS;

fn main() {
    unsafe {
        let bus = (IBUS.ibus_bus_new)();
        if bus.is_null() {
            panic!("bus was null");
        }
        let engine = (IBUS.ibus_bus_get_global_engine)(bus);
        if engine.is_null() {
            panic!("engine was null");
        }
        let name = (IBUS.ibus_engine_desc_get_name)(engine);
        if name.is_null() {
            panic!("name was null");
        }
        let name_str = CStr::from_ptr(name).to_str().unwrap();
        println!("name is: {}", name_str);
    }
}
