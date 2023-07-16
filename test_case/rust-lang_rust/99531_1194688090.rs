rust
      for<'a> extern "C" fn(Foo<'a>) -> Bar<'a>
      // desugars to:
      FnPtr<
          {FnModifiers {
              unsafety: Unsafety::Safe,
              c_variadic: false,
              abi: Abi::C,
          }},
          for<'a> k#fn_sig(Foo<'a>) -> Bar<'a>>,
      >
      