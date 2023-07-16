
  = note: ld.lld: error: undefined symbol: anon.91c2f56426c5022640e519f02889d07d.8.llvm.746630307250149404
          >>> referenced by C:\msys64\home\we\rust\library\core\src/option.rs:0
          >>>               librustc_resolve-fac19ceb16326411.rlib(rustc_resolve-fac19ceb16326411.1brnvk2mjzqx5b9n.rcgu.o):(rustc_resolve::macros::registered_idents)
          >>> referenced by C:\msys64\home\we\rust\library\alloc\src\vec/mod.rs:425
          >>>               librustc_resolve-fac19ceb16326411.rlib(rustc_resolve-fac19ceb16326411.2u8t3u4aqh8idzl4.rcgu.o):(<rustc_resolve::late::lifetimes::LifetimeContext>::with::<<rustc_resolve::late::lifetimes::LifetimeContext>::visit_fn_like_elision::{closure#9}>)
          >>> referenced by C:\msys64\home\we\rust\library\alloc\src\vec/mod.rs:425
          >>>               librustc_resolve-fac19ceb16326411.rlib(rustc_resolve-fac19ceb16326411.2u8t3u4aqh8idzl4.rcgu.o):(<rustc_resolve::late::lifetimes::LifetimeContext>::with::<<rustc_resolve::late::lifetimes::LifetimeContext>::visit_segment_args::{closure#6}>)
          >>> referenced 30 more times

          ld.lld: error: undefined symbol: anon.91c2f56426c5022640e519f02889d07d.9.llvm.746630307250149404
          >>> referenced by C:\msys64\home\we\rust\library\alloc\src\vec/mod.rs:425
          >>>               librustc_resolve-fac19ceb16326411.rlib(rustc_resolve-fac19ceb16326411.2u8t3u4aqh8idzl4.rcgu.o):(<rustc_resolve::late::lifetimes::LifetimeContext>::with::<<rustc_resolve::late::lifetimes::LifetimeContext>::visit_fn_like_elision::{closure#9}>)
          >>> referenced by C:\msys64\home\we\rust\library\alloc\src\vec/mod.rs:425
          >>>               librustc_resolve-fac19ceb16326411.rlib(rustc_resolve-fac19ceb16326411.2u8t3u4aqh8idzl4.rcgu.o):(<rustc_resolve::late::lifetimes::LifetimeContext>::with::<<rustc_resolve::late::lifetimes::LifetimeContext>::visit_segment_args::{closure#6}>)
          >>> referenced by C:\msys64\home\we\rust\library\alloc\src\vec/mod.rs:425
          >>>               librustc_resolve-fac19ceb16326411.rlib(rustc_resolve-fac19ceb16326411.2u8t3u4aqh8idzl4.rcgu.o):(<rustc_resolve::late::lifetimes::LifetimeContext>::with::<<rustc_resolve::late::lifetimes::LifetimeContext>::visit_segment_args::{closure#7}>)
          >>> referenced 30 more times

          ld.lld: error: undefined symbol: core::ptr::drop_in_place::<rustc_span::FileName> (.llvm.1438190246702246187)
          >>> referenced by C:\msys64\home\we\rust\library\core\src\ptr/mod.rs:188
          >>>               librustc_resolve-fac19ceb16326411.rlib(rustc_resolve-fac19ceb16326411.3bc39m1okvl6wof.rcgu.o):(<&mut <rustc_resolve::Resolver>::check_unused::{closure#0} as core::ops::function::FnMut<(&rustc_span::span_encoding::Span,)>>::call_mut)
          >>> referenced by C:\msys64\home\we\rust\library\core\src\ptr/mod.rs:0
          >>>               librustc_resolve-fac19ceb16326411.rlib(rustc_resolve-fac19ceb16326411.3bc39m1okvl6wof.rcgu.o):(<&mut <rustc_resolve::Resolver>::check_unused::{closure#0} as core::ops::function::FnMut<(&rustc_span::span_encoding::Span,)>>::call_mut)
          >>> referenced by C:\msys64\home\we\rust\library\core\src\ptr/mod.rs:188
          >>>               librustc_resolve-fac19ceb16326411.rlib(rustc_resolve-fac19ceb16326411.4391tludo7n7dv8i.rcgu.o):(core::ptr::drop_in_place::<rustc_span::SpanSnippetError>)
          >>> referenced 9 more times

          ld.lld: error: undefined symbol: core::ptr::drop_in_place::<(rustc_span::FileName, rustc_span::BytePos)> (.llvm.1438190246702246187)
          >>> referenced by C:\msys64\home\we\rust\library\core\src\ptr/mod.rs:188
          >>>               librustc_resolve-fac19ceb16326411.rlib(rustc_resolve-fac19ceb16326411.3bc39m1okvl6wof.rcgu.o):(<&mut <rustc_resolve::Resolver>::check_unused::{closure#0} as core::ops::function::FnMut<(&rustc_span::span_encoding::Span,)>>::call_mut)
          >>> referenced by C:\msys64\home\we\rust\library\core\src\ptr/mod.rs:188
          >>>               librustc_resolve-fac19ceb16326411.rlib(rustc_resolve-fac19ceb16326411.4391tludo7n7dv8i.rcgu.o):(core::ptr::drop_in_place::<rustc_span::SpanSnippetError>)
          >>> referenced by C:\msys64\home\we\rust\library\core\src\ptr/mod.rs:188
          >>>               librustc_resolve-fac19ceb16326411.rlib(rustc_resolve-fac19ceb16326411.5327cotcw21h0vj4.rcgu.o):(core::ptr::drop_in_place::<core::result::Result<alloc::string::String, rustc_span::SpanSnippetError>>)
          >>> referenced 2 more times
          collect2.exe: error: ld returned 1 exit status
