
            // No need to overwrite an already evaluated constant
            Rvalue::Use(Operand::Constant(box Constant {
                literal: Literal::Value {
                    value: &ty::Const {
                        val: ConstVal::Value(_),
                        ..
                    },
                },
                ..
            })) => None,
