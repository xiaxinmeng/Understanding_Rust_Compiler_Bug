
     -mark_dead_strippable_dylib
                 Specifies that the dylib being built can be dead strip by
                 any client.  That is, the dylib has no initialization side
                 effects.  So if a client links against the dylib, but never
                 uses any symbol from it, the linker can optimize away the
                 use of the dylib.
