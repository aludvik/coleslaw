extern crate cpython;

use std::collections::HashMap;

use cpython::{PyObject, Python, PyTuple, ObjectProtocol};

// -- C interface --
#[no_mangle]
pub extern "C" fn store_new() -> *mut Store {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let lib = py.import("lib")
        .map_err(|err| err.print(py)).unwrap();
    let counter = lib.call(py, "Counter", PyTuple::new(py, &[]), None)
        .map_err(|err| err.print(py)).unwrap();
    Box::into_raw(Box::new(Store::new(Box::new(PyCounter{ pyobj: counter }))))
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

struct PyCounter {
    pyobj: PyObject
}

impl Counter for PyCounter {
    fn inc(&mut self) {
        let gil = Python::acquire_gil();
        let py = gil.python();
        self.pyobj.call_method(py, "inc", PyTuple::new(py, &[]), None).unwrap();
    }
    fn get(&self) -> i32 {
        let gil = Python::acquire_gil();
        let py = gil.python();
        self.pyobj.call_method(py, "get", PyTuple::new(py, &[]), None).unwrap().extract(py).unwrap()
    }
}

pub trait Counter {
    fn inc(&mut self);
    fn get(&self) -> i32;
}

// -- Rust interface --
pub struct Store {
    store: HashMap<i32, i32>,
    counter: Box<Counter>,
}

impl Store {
    pub fn new(counter: Box<Counter>) -> Self {
        Store{
            store: HashMap::new(),
            counter: counter,
        }
    }

    pub fn set_counter(&mut self, counter: Box<Counter>) {
        self.counter = counter;
    }

    pub fn put(&mut self, key: i32, val: i32) {
        self.counter.inc();
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
        println!("put called {} times", self.counter.get());
    }
}
