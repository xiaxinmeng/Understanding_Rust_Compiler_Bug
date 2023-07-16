rust
            macro_rules! run_loop {
                ($unchecked_additive_op:ident) => {
                    for &c in digits {
                        result = result.unchecked_mul(radix);
                        let x = (c as char).to_digit(radix).ok_or(PIE { kind: InvalidDigit })?;
                        result = T::$unchecked_additive_op(&result, x);
                    }
                }
            }

            if is_positive { run_loop!(unchecked_add) } else { run_loop!(unchecked_sub) };
