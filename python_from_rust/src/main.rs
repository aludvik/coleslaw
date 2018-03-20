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

fn use_store(py: &mut Python, store_class: PyObject) -> PyResult<()> {
    let locals = PyDict::new(*py);
    locals.set_item(*py, "Store", store_class)?;
    py.run(r#"
store = Store()
store.put(1, 2)
print(store.get(1))
assert(store.get(1) == 2)
"#, None, Some(&locals))
}
