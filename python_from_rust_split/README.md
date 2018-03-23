This model uses the C ABI to communicate from Rust to Python and spawn a Python
interpreter using the Rust cpython library. This model is good because there
is no serialization cost in communicating between processes like with the ZMQ
model, but the downside is memory management is difficult (who is responsible
for freeing the memory?). It also requires writing the same interface four
times for Rust library code.

1. The main Rust library
2. A C ABI wrapper around the Rust library
3. C header declarations
4. A Python wrapper around the C ABI

With this model, it is possible to instantiate python objects outside of
central python program and pass them into the main python program using
function arguments, as demonstrated in this example.
