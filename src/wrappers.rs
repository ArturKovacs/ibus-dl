
use std::{marker::PhantomData, mem::MaybeUninit};

use glib::gobject_ffi::{g_object_ref, g_object_unref, GWeakRef, g_weak_ref_init, g_weak_ref_get};

use crate::ffi;

pub struct WeakRef<T> {
    native: Box<GWeakRef>,
    _phantom: PhantomData<T>,
}

impl<T> WeakRef<T> {
    /// Returns None if the object doesn't have any more strong references to it.
    /// In other words if the object has been destroyed.
    pub fn to_strong(&self) -> Option<StrongRef<T>> {
        unsafe {
            // It's safe to cast the immutable reference to a mutable pointer
            // because this refcounting is atomic
            let ptr = g_weak_ref_get((&*self.native) as *const _ as *mut _);
            if ptr.is_null() {
                None
            } else {
                Some(StrongRef {
                    native: ptr as *mut _
                })
            }
        }
    }
}

/// Strong reference to an IBus ojbect
pub struct StrongRef<T> {
    native: *mut T,
}

impl<T> StrongRef<T> {
    pub fn to_weak(&self) -> WeakRef<T> {
        let weak = unsafe {
            let uninit = MaybeUninit::<GWeakRef>::zeroed().assume_init();
            let mut weak = Box::new(uninit);
            g_weak_ref_init((&mut *weak) as *mut _, self.native as *mut _);
            weak
        };
        WeakRef {
            native: weak,
            _phantom: Default::default()
        }
    }
}

impl<T> Clone for StrongRef<T> {
    fn clone(&self) -> Self {
        unsafe {
            g_object_ref(self.native as *mut _);
        }
        StrongRef {
            native: self.native
        }
    }
}

impl<T> Drop for StrongRef<T> {
    fn drop(&mut self) {
        unsafe {
            g_object_unref(self.native as *mut _);
        }
    }
}

pub type Bus = StrongRef<ffi::IBusBus>;
