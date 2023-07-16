c
typedef void (*callback)(int);
// global cb
callback cb;

// parameter cb
void register(callback cb) {
    cb = cb; // XXX: the reason, rename parameter cb to _cb.
}

void call() {
    cb(9527);
}
// ...
