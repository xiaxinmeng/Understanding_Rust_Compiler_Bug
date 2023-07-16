rust
src/libarena/lib.rs
147:                self.ptr.set(self.ptr.get().offset(1));
543:            self.count.set(self.count.get() + 1);

src/test/run-pass/resource-assign-is-not-copy.rs
20:        self.i.set(self.i.get() + 1);

src/test/run-pass/vec-slice-drop.rs
20:        self.x.set(self.x.get() + 1);

src/test/run-pass/traits-conditional-model-fn.rs
33:        self.counter.set(self.counter.get() + arg);
45:        self.counter.set(self.counter.get() + arg);

src/test/run-pass/overloaded-autoderef-count.rs
39:        self.count_imm.set(self.count_imm.get() + 1);

src/test/run-pass/issue-16492.rs
32:        self.state.set(self.state.get()+1);

src/test/run-pass/option-unwrap.rs
19:        self.x.set(self.x.get() - 1);

src/test/run-pass/dynamic-drop.rs
53:        self.cur_ops.set(self.cur_ops.get() + 1);
76:        self.1.cur_ops.set(self.1.cur_ops.get()+1);

src/test/run-pass/packed-struct-drop-aligned.rs
27:        self.drop_count.set(self.drop_count.get() + 1);

src/test/run-pass/overloaded-deref-count.rs
40:        self.count_imm.set(self.count_imm.get() + 1);

src/test/run-pass/resource-destruct.rs
19:        println!("Hello!"); self.i.set(self.i.get() - 1);

src/test/run-pass/issue-979.rs
19:        self.b.set(self.b.get() + 1);

src/test/run-pass/init-res-into-things.rs
26:        self.i.set(self.i.get() + 1)

src/test/pretty/block-disambig.rs
60:    match true { true => { } _ => { } } regs.set(regs.get() + 1);

src/liballoc/tests/slice.rs
1407:        self.version.set(self.version.get() + 1);
1408:        other.version.set(other.version.get() + 1);

src/librustc_typeck/check/_match.rs
610:            self.diverges.set(self.diverges.get() | Diverges::Always);

src/librustc_typeck/check/mod.rs
3601:            self.diverges.set(self.diverges.get() | Diverges::Always);
3610:        self.diverges.set(self.diverges.get() | old_diverges);
3611:        self.has_errors.set(self.has_errors.get() | old_has_errors);
4362:        self.diverges.set(self.diverges.get() | old_diverges);
4363:        self.has_errors.set(self.has_errors.get() | old_has_errors);

src/librustc/util/common.rs
237:    accu.set(duration + accu.get());
384:        self.set(self.get() + 1);

src/librustc/session/mod.rs
899:                self.print_fuel.set(self.print_fuel.get() + 1);
