
error: internal compiler error: encountered ambiguity selecting `Binder(<response::stream::reader::ReaderStream<impl futures::Stream<Item = response::stream::raw_sse::RawLinedEvent>> as std::convert::From<impl futures::Stream<Item = response::stream::raw_sse::RawLinedEvent>>>, [])` during codegen, presuming due to overflow or prior type error
  |
  = note: delayed at compiler\rustc_trait_selection\src\traits\codegen.rs:54:32

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler\rustc_errors\src\lib.rs:1176:13
