 Rust
    pub fn new<T: CallbackContainer>(callback: &T, handling: ExceptionHandling) -> CallSetup {
        let global = unsafe { global_root_from_object(callback.callback()) };
        let cx = global.r().get_cx();

        let exception_compartment = unsafe {
            GetGlobalForObjectCrossCompartment(callback.callback())
        };
        CallSetup {
            exception_compartment: RootedObject::new_with_addr(cx,
                                                               exception_compartment,
                                                               unsafe { return_address() }),
            cx: cx,
            old_compartment: unsafe { JS_EnterCompartment(cx, callback.callback()) },
            handling: handling,
        }
    }
