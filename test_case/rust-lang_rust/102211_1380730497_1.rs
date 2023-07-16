
│ │ ├─┐rustc_borrowck::type_check::canonical::normalize_with_category value=([[async block@lib.rs:6:5: 12:6]]; c_variadic: false)->[async block@lib.rs:6:5: 12:6], location=bb1[3], category=Boring
│ │ │ ├─0ms DEBUG rustc_borrowck::type_check::canonical output=([[async block@lib.rs:6:5: 12:6]]; c_variadic: false)->[async block@lib.rs:6:5: 12:6], constraints=None
│ │ ├─┘
│ │ ├─┐rustc_borrowck::type_check::canonical::normalize_with_category value=impl std::future::Future<Output = ()>, location=bb1[3], category=Boring
│ │ │ ├─0ms DEBUG rustc_borrowck::type_check::canonical output=impl std::future::Future<Output = ()>, constraints=None
│ │ ├─┘
│ │ ├─┐rustc_borrowck::type_check::relate_tys::relate_types a=impl std::future::Future<Output = ()>, v=-, b=[async block@lib.rs:6:5: 12:6], locations=Single(bb1[3]), category=Return(Normal)
│ │ │ ├─0ms DEBUG rustc_borrowck::type_check::canonical output=(), constraints=None
│ │ ├─┘
