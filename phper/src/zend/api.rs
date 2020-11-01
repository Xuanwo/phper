use crate::sys::{zend_function_entry, zend_ini_entry_def};
use std::cell::UnsafeCell;
use std::os::raw::{c_char, c_int, c_void};
use crate::zend::ini::Mh;
use std::ptr::null_mut;

#[repr(C)]
pub struct ModuleGlobals<T: 'static> {
    inner: UnsafeCell<T>,
}

impl<T: 'static> ModuleGlobals<T> {
    pub const fn new(inner: T) -> Self {
        Self { inner: UnsafeCell::new(inner) }
    }

    pub const fn get(&self) -> *mut T {
        self.inner.get()
    }

    pub const fn create_ini_entry_def(&'static self, name: &str, default_value: &str, on_modify: Option<Mh>, modifiable: u32) -> zend_ini_entry_def {
        zend_ini_entry_def {
            name: name.as_ptr().cast(),
            on_modify,
            mh_arg1: 0 as *mut _,
            mh_arg2: self.get().cast(),
            mh_arg3: null_mut(),
            value: default_value.as_ptr().cast(),
            displayer: None,
            modifiable: modifiable as c_int,
            name_length: name.len() as u32,
            value_length: default_value.len() as u32,
        }
    }
}

unsafe impl<T: 'static> Sync for ModuleGlobals<T> {}

pub struct FunctionEntries<const N: usize> {
    inner: UnsafeCell<[zend_function_entry; N]>,
}

impl<const N: usize> FunctionEntries<N> {
    pub const fn new(inner: [zend_function_entry; N]) -> Self {
        Self { inner: UnsafeCell::new(inner) }
    }

    pub const fn get(&self) -> *const zend_function_entry {
        self.inner.get().cast()
    }
}

unsafe impl<const N: usize> Sync for FunctionEntries<N> {}
