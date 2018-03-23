extern crate cpython;

use cpython::{Python, PyTuple};

fn main() {
    let gil = Python::acquire_gil();
    let py = &mut gil.python();

    let lib = py.import("lib")
        .map_err(|err| err.print(*py)).unwrap();
    let store = lib.call(*py, "Store", PyTuple::new(*py, &[]), None)
        .map_err(|err| err.print(*py)).unwrap();

    let main = py.import("main")
        .map_err(|err| err.print(*py)).unwrap();
    main.call(*py, "main", (store,), None)
        .map_err(|err| err.print(*py)).unwrap();
}
