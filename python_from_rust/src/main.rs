extern crate cpython;

mod lib;
use lib::import_store;
use cpython::{Python, PyDict, PyObject, PyResult};

fn main() {
    let gil = Python::acquire_gil();
    let mut py = &mut gil.python();

    let store_class = import_store(py).map_err(|err| err.print(*py)).unwrap();
    use_store(py, store_class).map_err(|err| err.print(*py)).unwrap();
}

fn use_store(py: &mut Python, store_class: PyDict) -> PyResult<()> {
    py.run(r#"
class Store:
    def __init__(self, ffi, lib):
        self._lib = lib
        self._ffi = ffi
        self._store = self._lib.store_new()

    def put(self, key, value):
        self._lib.store_put(self._store, self._ffi.cast("int", key), self._ffi.cast("int", value))

    def get(self, key):
        c_value = self._lib.store_get(self._store, self._ffi.cast("int", key))
        return c_value[0]

store = Store(ffi, lib)
store.put(1, 2)
print(store.get(1))
"#, None, Some(&store_class))
}
