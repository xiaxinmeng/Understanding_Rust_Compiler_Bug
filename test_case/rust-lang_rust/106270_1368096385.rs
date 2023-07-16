
$ jq -c 'sort_by(.total_estimate, .name) | .[]' rustc_query_impl.mono_items-thread_local_attr.json | diff - <(jq -c 'sort_by(.total_estimate, .name) | .[]' rustc_query_impl.mono_items-thread_local_macro.json)
1893a1894
> {"name":"rustc_middle::ty::tls::get_tlv::{closure#0}","instantiation_count":1,"size_estimate":5,"total_estimate":5}
2997a2999
> {"name":"std::thread::__FastLocalKeyInner::<T>::register_dtor","instantiation_count":1,"size_estimate":10,"total_estimate":10}
9137a9140
> {"name":"rustc_middle::ty::tls::TLV::__getit","instantiation_count":3,"size_estimate":75,"total_estimate":225}
9175d9177
< {"name":"std::mem::needs_drop","instantiation_count":125,"size_estimate":2,"total_estimate":250}
9182a9185
> {"name":"std::mem::needs_drop","instantiation_count":128,"size_estimate":2,"total_estimate":256}
9267d9269
< {"name":"std::thread::LocalKey::<T>::with","instantiation_count":19,"size_estimate":17,"total_estimate":323}
9384c9386
< {"name":"std::mem::drop","instantiation_count":257,"size_estimate":2,"total_estimate":514}
---
> {"name":"std::mem::drop","instantiation_count":256,"size_estimate":2,"total_estimate":512}
9386d9387
< {"name":"<std::result::Result<T, F> as std::ops::FromResidual<std::result::Result<std::convert::Infallible, E>>>::from_residual","instantiation_count":40,"size_estimate":13,"total_estimate":520}
9395a9397
> {"name":"<std::result::Result<T, F> as std::ops::FromResidual<std::result::Result<std::convert::Infallible, E>>>::from_residual","instantiation_count":41,"size_estimate":13,"total_estimate":533}
9828d9829
< {"name":"std::thread::LocalKey::<T>::try_with","instantiation_count":19,"size_estimate":58,"total_estimate":1102}
9880a9882
> {"name":"std::cell::Cell::<T>::get","instantiation_count":192,"size_estimate":8,"total_estimate":1536}
9883d9884
< {"name":"std::cell::Cell::<T>::get","instantiation_count":194,"size_estimate":8,"total_estimate":1552}
9989,9990c9990,9991
< {"name":"std::mem::MaybeUninit::<T>::uninit","instantiation_count":1454,"size_estimate":2,"total_estimate":2908}
< {"name":"std::mem::ManuallyDrop::<T>::into_inner","instantiation_count":1456,"size_estimate":2,"total_estimate":2912}
---
> {"name":"std::mem::MaybeUninit::<T>::uninit","instantiation_count":1453,"size_estimate":2,"total_estimate":2906}
> {"name":"std::mem::ManuallyDrop::<T>::into_inner","instantiation_count":1455,"size_estimate":2,"total_estimate":2910}
9998a10000
> {"name":"std::cell::Cell::<T>::set","instantiation_count":192,"size_estimate":16,"total_estimate":3072}
10001d10002
< {"name":"std::cell::Cell::<T>::set","instantiation_count":193,"size_estimate":16,"total_estimate":3088}
10059a10061
> {"name":"std::cell::Cell::<T>::replace","instantiation_count":192,"size_estimate":25,"total_estimate":4800}
10062d10063
< {"name":"std::cell::Cell::<T>::replace","instantiation_count":193,"size_estimate":25,"total_estimate":4825}
10137c10138
< {"name":"std::cell::UnsafeCell::<T>::get","instantiation_count":1257,"size_estimate":8,"total_estimate":10056}
---
> {"name":"std::cell::UnsafeCell::<T>::get","instantiation_count":1255,"size_estimate":8,"total_estimate":10040}
10151c10152
< {"name":"std::mem::MaybeUninit::<T>::assume_init","instantiation_count":1456,"size_estimate":8,"total_estimate":11648}
---
> {"name":"std::mem::MaybeUninit::<T>::assume_init","instantiation_count":1455,"size_estimate":8,"total_estimate":11640}
10157c10158
< {"name":"std::mem::MaybeUninit::<T>::as_mut_ptr","instantiation_count":1544,"size_estimate":8,"total_estimate":12352}
---
> {"name":"std::mem::MaybeUninit::<T>::as_mut_ptr","instantiation_count":1543,"size_estimate":8,"total_estimate":12344}
10172a10174,10175
> {"name":"rustc_middle::ty::tls::set_tlv::{closure#0}::{closure#0}","instantiation_count":1597,"size_estimate":9,"total_estimate":14373}
> {"name":"rustc_middle::ty::tls::set_tlv::{closure#1}","instantiation_count":1597,"size_estimate":9,"total_estimate":14373}
10181,10182c10184
< {"name":"rustc_middle::ty::tls::set_tlv::{closure#0}","instantiation_count":1597,"size_estimate":11,"total_estimate":17567}
< {"name":"std::ptr::read::runtime","instantiation_count":1426,"size_estimate":13,"total_estimate":18538}
---
> {"name":"std::ptr::read::runtime","instantiation_count":1425,"size_estimate":13,"total_estimate":18525}
10184c10186
< {"name":"std::result::Result::<T, E>::expect","instantiation_count":968,"size_estimate":20,"total_estimate":19360}
---
> {"name":"std::result::Result::<T, E>::expect","instantiation_count":969,"size_estimate":20,"total_estimate":19380}
10185a10188
> {"name":"rustc_middle::ty::tls::set_tlv::{closure#0}","instantiation_count":1597,"size_estimate":13,"total_estimate":20761}
10189c10192
< {"name":"std::mem::replace","instantiation_count":1140,"size_estimate":20,"total_estimate":22800}
---
> {"name":"std::mem::replace","instantiation_count":1139,"size_estimate":20,"total_estimate":22780}
10199c10202
< {"name":"std::ptr::write::runtime","instantiation_count":1819,"size_estimate":16,"total_estimate":29104}
---
> {"name":"std::ptr::write::runtime","instantiation_count":1818,"size_estimate":16,"total_estimate":29088}
10212,10213c10215,10216
< {"name":"std::ptr::write","instantiation_count":1790,"size_estimate":21,"total_estimate":37590}
< {"name":"std::ptr::read","instantiation_count":1426,"size_estimate":28,"total_estimate":39928}
---
> {"name":"std::ptr::write","instantiation_count":1789,"size_estimate":21,"total_estimate":37569}
> {"name":"std::ptr::read","instantiation_count":1425,"size_estimate":28,"total_estimate":39900}
10227a10231
> {"name":"std::thread::LocalKey::<T>::with","instantiation_count":3214,"size_estimate":17,"total_estimate":54638}
10229c10233
< {"name":"rustc_middle::ty::tls::set_tlv","instantiation_count":1597,"size_estimate":37,"total_estimate":59089}
---
> {"name":"rustc_middle::ty::tls::set_tlv","instantiation_count":1597,"size_estimate":39,"total_estimate":62283}
10240a10245
> {"name":"std::thread::LocalKey::<T>::try_with","instantiation_count":3214,"size_estimate":58,"total_estimate":186412}
