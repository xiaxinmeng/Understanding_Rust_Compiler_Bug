rust
extern "C" {
    static _dispatch_queue_attr_concurrent: ();
}

static DISPATCH_QUEUE_CONCURRENT: &'static () =
    unsafe { &_dispatch_queue_attr_concurrent };
