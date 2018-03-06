from cffi import FFI
ffi = FFI()
ffi.cdef("""
    int timestwo(int x);
""")
# TODO: Replace with host OS check
try:
    C = ffi.dlopen("../rust/target/debug/librust_from_python_ffi.dylib")
except OSError:
    C = ffi.dlopen("../rust/target/debug/librust_from_python_ffi.so")

print(C.timestwo(9))
