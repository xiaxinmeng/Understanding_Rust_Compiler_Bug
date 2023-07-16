
[00:07:50] error[E0277]: the trait bound `core::option::Option<mir::ClosureRegionRequirements>: serialize::Decodable` is not satisfied
[00:07:50]    --> /checkout/src/librustc/ty/maps/config.rs:634:48
[00:07:50]     |
[00:07:50] 634 |                 tcx.on_disk_query_result_cache.try_load_query_result(tcx, id)
[00:07:50]     |                                                ^^^^^^^^^^^^^^^^^^^^^ the trait `serialize::Decodable` is not implemented for `core::option::Option<mir::ClosureRegionRequirements>`
[00:07:50] ...
[00:07:50] 642 | impl_disk_cacheable_query!(mir_borrowck, |def_id| def_id.is_local());
[00:07:50]     | --------------------------------------------------------------------- in this macro invocation
[00:07:50]     |
[00:07:50]     = help: the following implementations were found:
[00:07:50]               <core::option::Option<T> as serialize::Decodable>
[00:07:50] 
[00:07:50] error[E0277]: the trait bound `core::option::Option<mir::ClosureRegionRequirements>: serialize::Encodable` is not satisfied
[00:07:50]    --> /checkout/src/librustc/ty/maps/on_disk_cache.rs:213:13
[00:07:50]     |
[00:07:50] 213 |             encode_query_results::<mir_borrowck, _>(tcx, enc, qri)?;
[00:07:50]     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `serialize::Encodable` is not implemented for `core::option::Option<mir::ClosureRegionRequirements>`
[00:07:50]     |
[00:07:50]     = help: the following implementations were found:
[00:07:50]               <core::option::Option<T> as serialize::Encodable>
[00:07:50]     = note: required by `ty::maps::on_disk_cache::encode_query_results`
[00:07:50] 
[00:07:52] error: aborting due to 2 previous errors
[00:07:52] 
[00:07:52] error: Could not compile `rustc`.
[00:07:52] 
