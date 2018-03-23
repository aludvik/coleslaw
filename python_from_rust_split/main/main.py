def main(store):
    store.put(1, 2)
    i = store.get(1)
    print(i)
    assert(i == 2)
