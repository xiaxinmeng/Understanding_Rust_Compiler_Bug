plain
   Compiling chalk-ir v0.55.0
   Compiling tracing v0.1.28
   Compiling rustc_index v0.0.0 (/checkout/compiler/rustc_index)
   Compiling rustc_data_structures v0.0.0 (/checkout/compiler/rustc_data_structures)
error: using `retain` can result in unstable query results
   --> compiler/rustc_data_structures/src/obligation_forest/mod.rs:698:27
    |
698 |         self.active_cache.retain(|_predicate, index| {
    |
    |
    = note: `-D rustc::potential-query-instability` implied by `-D warnings`
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `into_iter` can result in unstable query results
   |
64 |         let mut vector = self.base.into_iter().collect::<Vec<_>>();
   |                                    ^^^^^^^^^
   |
   |
   = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `into_iter` can result in unstable query results
   |
53 |         let mut vector = self.base.into_iter().collect::<Vec<_>>();
   |                                    ^^^^^^^^^
   |
   |
   = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `iter` can result in unstable query results
    |
    |
513 |         let mut keys: Vec<_> = self.iter().map(|k| k.to_stable_hash_key(hcx)).collect();
    |
    |
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `iter` can result in unstable query results
    |
    |
555 |     let mut entries: Vec<_> = map.iter().map(|(k, v)| (to_stable_hash_key(k, hcx), v)).collect();
    |
    |
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `drain` can result in unstable query results
   --> compiler/rustc_data_structures/src/sso/map.rs:170:59
    |
170 |             SsoHashMap::Map(map) => EitherIter::Right(map.drain()),
    |
    |
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `drain` can result in unstable query results
   --> compiler/rustc_data_structures/src/sso/map.rs:208:47
    |
208 |                 *self = SsoHashMap::Array(map.drain().collect());
    |
    |
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `retain` can result in unstable query results
   --> compiler/rustc_data_structures/src/sso/map.rs:222:41
    |
222 |             SsoHashMap::Map(map) => map.retain(f),
    |
    |
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `into_iter` can result in unstable query results
   --> compiler/rustc_data_structures/src/sso/map.rs:411:59
    |
411 |             SsoHashMap::Map(map) => EitherIter::Right(map.into_iter()),
    |
    |
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `iter` can result in unstable query results
   --> compiler/rustc_data_structures/src/sso/map.rs:443:59
    |
443 |             SsoHashMap::Map(map) => EitherIter::Right(map.iter()),
    |
    |
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale

error: using `iter_mut` can result in unstable query results
   --> compiler/rustc_data_structures/src/sso/map.rs:461:59
    |
461 |             SsoHashMap::Map(map) => EitherIter::Right(map.iter_mut()),
    |
    |
    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale
error: could not compile `rustc_data_structures` due to 11 previous errors
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:05:50
