
         ::core::fmt::Formatter::write_str(f, {
             const __NAMES: [&str; 3] = ["A", "B", "C"];
             __NAMES[::core::intrinsics::discriminant_value(self) as usize]
         })
