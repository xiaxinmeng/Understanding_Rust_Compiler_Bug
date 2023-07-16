
            // Get all values map at once otherwise it would be costly.
            // (8 values * 220 targets ~= 1760 times, at the time of writing this comment).
            let [
                values_target_os,
                values_target_family,
                values_target_arch,
                values_target_endian,
                values_target_env,
                values_target_abi,
                values_target_vendor,
                values_target_pointer_width,
            ] = self
                .values_valid
                .get_many_mut(VALUES)
                .expect("unable to get all the check-cfg values buckets");
