rust
<<<<<<< HEAD
                // Always yeet out for errors on debug (unless
                // `RUSTC_TRANSLATION_NO_DEBUG_ASSERT` is set in the environment - this allows
                // local runs of the test suites, of builds with debug assertions, to test the
                // behaviour in a normal build).
                Some(Err(primary))
                    if cfg!(debug_assertions)
                        && env::var("RUSTC_TRANSLATION_NO_DEBUG_ASSERT").is_err() =>
                {
                    do yeet primary
                }
=======
                // If `translate_with_bundle` returns `Err` with the primary bundle, this is likely
                // just that the primary bundle doesn't contain the message being translated, so
                // proceed to the fallback bundle.
                Some(Err(
                    primary @ TranslateError::One {
                        kind: TranslateErrorKind::MessageMissing, ..
                    },
                )) => translate_with_bundle(self.fallback_fluent_bundle())
                    .map_err(|fallback| primary.and(fallback))?,

                // However, when errors are produced from translation, then that means the translation
                // is broken (e.g. `{$foo}` exists in a translation but `foo` isn't provided).
                //
                // In debug builds, assert so that compiler devs can spot the broken translation and
                // fix it..
                Some(Err(primary)) if cfg!(debug_assertions) => do yeet primary,
>>>>>>> 6b812c48ce6 (Restore behavior when primary bundle is missing)
