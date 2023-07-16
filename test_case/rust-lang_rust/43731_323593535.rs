rust
            CheckedBinaryOp(bin_op, ref left, ref right) => {
                // Due to the extra boolean in the result, we can never reuse the `dest.layout`.
                let left = self.read_immediate(self.eval_operand(left, None)?)?;
                let layout = if binop_right_homogeneous(bin_op) { Some(left.layout) } else { None };
                let right = self.read_immediate(self.eval_operand(right, layout)?)?;
                self.binop_with_overflow(                                       
                    bin_op,
                    left,
                    right,
                    dest,
                )?;
                eprintln!("!{:?}!{:?}!{:?}!{:?}!", left,bin_op,right,dest);
            }
