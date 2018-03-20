extern crate cpython;

use cpython::{Python, PyDict, PyObject, PyResult};
use std::collections::HashMap;

// -- Python interface -- //
pub fn import_store(py: &mut Python) -> PyResult<PyDict> {
    // Variable created will be stored in locals
    let locals = PyDict::new(*py);

    let pycode = r#"
from cffi import FFI
ffi = FFI()
ffi.cdef("""
    void * store_new();
    void store_put(void * store, int key, int val);
    int * store_get(void * store, int key);
""")
lib = ffi.dlopen("../rust/target/debug/libpython_from_rust.dylib")
"#;

    py.run(pycode, None, Some(&locals))?;
    Ok(locals)
}

// -- C interface --
#[no_mangle]
pub extern "C" fn store_new() -> *mut Store {
    Box::into_raw(Box::new(Store::new()))
}

#[no_mangle]
pub extern "C" fn store_put(store: *mut Store, key: i32, val: i32) {
    assert!(!store.is_null());
    unsafe {
        (*store).put(key, val)
    }
}

#[no_mangle]
pub extern "C" fn store_get(store: *mut Store, key: i32) -> *const i32 {
    assert!(!store.is_null());
    unsafe {
        match (*store).get(key) {
            Some(i) => i as *const i32,
            None => 0 as *const i32,
        }
    }
}

// -- Rust interface --
#[repr(C)]
pub struct Store {
    store: HashMap<i32, i32>,
}

impl Store {
    pub fn new() -> Self {
        Store{ store: HashMap::new() }
    }

    pub fn put(&mut self, key: i32, val: i32) {
        self.store.insert(key, val);
    }

    pub fn get(&self, key: i32) -> Option<&i32> {
        self.store.get(&key)
    }
}