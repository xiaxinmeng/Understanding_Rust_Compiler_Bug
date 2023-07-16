rust
                        if res != old_binding.res() {
                            resolution.binding = Some(this.ambiguity(
                                AmbiguityKind::GlobVsGlob,
                                old_binding,
                                binding,
                            ));
                        } else if !old_binding.vis.is_at_least(binding.vis, &*this) {
