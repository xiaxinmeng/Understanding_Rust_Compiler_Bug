rust
            if non_exhaustive {
                self.non_exhaustive_reachable_pattern(pat, &accessible_unmentioned_fields, adt_ty)
            } else if !etc {
