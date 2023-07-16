rust
    fn eq(&self, other: &SbTag) -> bool {
        {
            let __self_vi = ::core::intrinsics::discriminant_value(&*self);
            let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*other) {
                    (&SbTag::Tagged(ref __self_0), &SbTag::Tagged(ref __arg_1_0)) => {
                        (*__self_0) == (*__arg_1_0)
                    }
                    _ => true,
                }
            } else {
                false
            }
        }
    }
