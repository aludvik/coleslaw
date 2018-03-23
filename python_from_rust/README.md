This model handles Rust+Python integration in a way that mixes all related
components into a single file. Specifically, it includes Python code in the
Rust code as a string, which is good because everything is in one place but
is bad because the Python code cannot be tested in isolation or linted.

A better option is in python_from_rust_split/
