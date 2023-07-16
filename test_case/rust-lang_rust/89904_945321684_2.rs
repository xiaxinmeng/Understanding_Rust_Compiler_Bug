rust
        let (flag_ps, flag_rest) = self.named_ps(|p| p.is_flag());
        let (name_value_ps, name_value_rest) = self.named_ps(|p| p.is_name_value());
        let (name_args_ps, name_args_rest) = self.named_ps(|p| p.is_name_args());

        let mut arms_named = Vec::new();
        for (index, p) in flag_ps.iter().enumerate() {
            arms_named.push(p.build_arm_parse(index, ArgKind::Flag));
        }
