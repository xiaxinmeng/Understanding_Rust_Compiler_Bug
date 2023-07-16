
    fn eq(&self, other: &Foobar) -> bool {
        match *other {
            Foobar { value: ref __self_1_0, value2: ref __self_1_1 } =>
            match *self {
                Foobar { value: ref __self_0_0, value2: ref __self_0_1 } =>
                true && (*__self_0_0) == (*__self_1_0) &&
                    (*__self_0_1) == (*__self_1_1),
            },
        }
    }
