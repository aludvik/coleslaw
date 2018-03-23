use std::collections::HashMap;

// -- C interface --
#[no_mangle]
pub extern "C" fn store_new() -> *mut Store {
    Box::into_raw(Box::new(Store::new()))
}

#[no_mangle]
pub extern "C" fn store_drop(store: *mut Store) {
    assert!(!store.is_null());
    // Restore the box from the raw pointer so it is dropped at the end
    // of this method: https://doc.rust-lang.org/std/boxed/struct.Box.html#method.from_raw
    unsafe { Box::from_raw(store) };
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

impl Drop for Store {
    fn drop(&mut self) {
        // Print to stdout so we can manually confirm the memory is freed
        // Note that the fields of Store are still dropped automatically
        // Trying to do this is a compiler error:
        // ::std::mem::drop(self.store); <- Try uncommenting this
        println!("dropping store in rust");
    }
}
