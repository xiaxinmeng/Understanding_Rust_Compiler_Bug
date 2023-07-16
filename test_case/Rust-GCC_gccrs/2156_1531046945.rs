rust
421-            #[stable(feature = "rust1", since = "1.0.0")]
422-            #[rustc_const_stable(feature = "const_int_conversions", since = "1.32.0")]
423-            #[inline]
424:            pub const fn to_le(self) -> Self {
425-                #[cfg(target_endian = "little")]
426-                {
427-                    self
428-                }
429-                #[cfg(not(target_endian = "little"))]
430-                {
431-                    self.swap_bytes()
432-                }
433-            }
