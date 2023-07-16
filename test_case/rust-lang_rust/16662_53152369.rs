 rust
            #[inline]
            #[allow(dead_code)]
            static __STATIC_FMTSTR: [::std::fmt::rt::Piece<'static>, ..4u] =
                [::std::fmt::rt::String("foo"),
                 ::std::fmt::rt::Argument(::std::fmt::rt::Argument{ ... }),
                 ::std::fmt::rt::Argument(::std::fmt::rt::Argument{ ... }),
                 ::std::fmt::rt::String("baz")];
            let __args_vec =
                &[::std::fmt::argument(::std::fmt::secret_show, __arg0),
                  ::std::fmt::argument(::std::fmt::secret_show, __arg1)];
            let __args =
                unsafe {
                    ::std::fmt::Arguments::new(__STATIC_FMTSTR, __args_vec)
                };
            ::std::io::stdio::println_args(&__args)
