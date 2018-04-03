from cffi import FFI
ffi = FFI()
ffi.cdef("""
    void * store_new();
    void store_drop(void * store);
    void store_put(void * store, int key, int val);
    int * store_get(void * store, int key);
""")
lib = ffi.dlopen("../target/debug/libpython_from_rust.dylib")

class Store:
    def __init__(self):
        self._store = lib.store_new()

    def put(self, key, value):
        lib.store_put(self._store, ffi.cast("int", key), ffi.cast("int", value))

    def get(self, key):
        c_value = lib.store_get(self._store, ffi.cast("int", key))
        return c_value[0]

    def __del__(self):
        print("store garbage collected by python")
        lib.store_drop(self._store)

class Counter:
    def __init__(self):
        self._count = 0

    def inc(self):
        self._count += 1

    def get(self):
        return self._count

    def __del__(self):
        print("counter garbage collected by python")
