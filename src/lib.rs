
//!
//! This crate provides an unsafe interface to the ibus library installed
//! to the system where it is executed. The ibus shared object is loaded
//! at runtime; there shouldn't be any need for any complie time action.
//! 
//! Note that all objects that have a `get_type` method, e.g.
//! `ibus_bus_get_type`, are "subclasses" of the GObject from glib.
//! This for example means that if you want to free such an object,
//! you can for example use the `g_object_unref` function from the
//! `gobject-sys` crate or better yet: use the `wrapper!` macro with
//! `Shared` kind from the `glib` crate.
//! 


#![allow(non_camel_case_types)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use once_cell::sync::Lazy;

#[cfg(feature = "wrappers")]
pub mod wrappers;

pub mod ffi;

pub static IBUS: Lazy<ffi::IBus> = Lazy::new(|| unsafe {
    ffi::IBus::open("libibus-1.0.so").expect(
        "Failed to load ibus. You might need to install `libibus`"
    )
});
