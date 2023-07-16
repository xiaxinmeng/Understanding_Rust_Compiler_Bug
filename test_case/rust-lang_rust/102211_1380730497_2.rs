
│ │ ├─┐rustc_borrowck::type_check::canonical::normalize_with_category value=([[async block@lib.rs:6:5: 12:6]]; c_variadic: false)->[async block@lib.rs:6:5: 12:6], location=bb1[3], category=Boring
│ │ │ ├─0ms DEBUG rustc_borrowck::type_check::canonical output=([[async block@lib.rs:6:5: 12:6]]; c_variadic: false)->[async block@lib.rs:6:5: 12:6], constraints=None
│ │ ├─┘
│ │ ├─┐rustc_borrowck::type_check::canonical::normalize_with_category value=impl std::future::Future<Output = ()> + std::marker::Send, location=bb1[3], category=Boring
│ │ │ ├─0ms DEBUG rustc_borrowck::type_check::canonical output=impl std::future::Future<Output = ()> + std::marker::Send, constraints=None
│ │ ├─┘
│ │ ├─┐rustc_borrowck::type_check::relate_tys::relate_types a=impl std::future::Future<Output = ()> + std::marker::Send, v=-, b=[async block@lib.rs:6:5: 12:6], locations=Single(bb1[3]), category=Return(Normal)
│ │ │ ├─0ms DEBUG rustc_borrowck::type_check::canonical output=(), constraints=Some(QueryRegionConstraints { outlives: [(Binder(OutlivesPredicate('_#2r, RePlaceholder(Placeholder { universe: U1, name: BrAnon(0, None) })), []), BoringNoLocation), (Binder(OutlivesPredicate('_#3r, RePlaceholder(Placeholder { universe: U1, name: BrAnon(0, None) })), []), BoringNoLocation), (Binder(OutlivesPredicate('_#4r, RePlaceholder(Placeholder { universe: U1, name: BrAnon(0, None) })), []), BoringNoLocation), (Binder(OutlivesPredicate('_#2r, RePlaceholder(Placeholder { universe: U1, name: BrAnon(1, Some(lib.rs:8:14: 8:17 (#0))) })), []), BoringNoLocation), (Binder(OutlivesPredicate(RePlaceholder(Placeholder { universe: U1, name: BrAnon(0, None) }), '_#2r), []), BoringNoLocation), (Binder(OutlivesPredicate(RePlaceholder(Placeholder { universe: U1, name: BrAnon(1, Some(lib.rs:8:14: 8:17 (#0))) }), '_#2r), []), BoringNoLocation), (Binder(OutlivesPredicate(RePlaceholder(Placeholder { universe: U1, name: BrAnon(0, None) }), '_#3r), []), BoringNoLocation), (Binder(OutlivesPredicate(RePlaceholder(Placeholder { universe: U1, name: BrAnon(0, None) }), '_#4r), []), BoringNoLocation)], member_constraints: [] })
│ │ │ ├─┐rustc_borrowck::type_check::push_region_constraints locations=Single(bb1[3]), category=Return(Normal)
│ │ │ │ ├─0ms DEBUG rustc_borrowck::type_check constraints generated: QueryRegionConstraints {
│ │ │ │ │     outlives: [
│ │ │ │ │         (
│ │ │ │ │             Binder(
│ │ │ │ │                 OutlivesPredicate(
│ │ │ │ │                     '_#2r,
│ │ │ │ │                     RePlaceholder(Placeholder { universe: U1, name: BrAnon(0, None) }),
│ │ │ │ │                 ),
│ │ │ │ │                 [],
│ │ │ │ │             ),
│ │ │ │ │             BoringNoLocation,
│ │ │ │ │         ),
│ │ │ │ │         (
│ │ │ │ │             Binder(
│ │ │ │ │                 OutlivesPredicate(
│ │ │ │ │                     '_#3r,
│ │ │ │ │                     RePlaceholder(Placeholder { universe: U1, name: BrAnon(0, None) }),
│ │ │ │ │                 ),
│ │ │ │ │                 [],
│ │ │ │ │             ),
│ │ │ │ │             BoringNoLocation,
│ │ │ │ │         ),
│ │ │ │ │         (
│ │ │ │ │             Binder(
│ │ │ │ │                 OutlivesPredicate(
│ │ │ │ │                     '_#4r,
│ │ │ │ │                     RePlaceholder(Placeholder { universe: U1, name: BrAnon(0, None) }),
│ │ │ │ │                 ),
│ │ │ │ │                 [],
│ │ │ │ │             ),
│ │ │ │ │             BoringNoLocation,
│ │ │ │ │         ),
│ │ │ │ │         (
│ │ │ │ │             Binder(
│ │ │ │ │                 OutlivesPredicate(
│ │ │ │ │                     '_#2r,
│ │ │ │ │                     RePlaceholder(Placeholder { universe: U1, name: BrAnon(1, Some(lib.rs:8:14: 8:17 (#0))) }),
│ │ │ │ │                 ),
│ │ │ │ │                 [],
│ │ │ │ │             ),
│ │ │ │ │             BoringNoLocation,
│ │ │ │ │         ),
│ │ │ │ │         (
│ │ │ │ │             Binder(
│ │ │ │ │                 OutlivesPredicate(
│ │ │ │ │                     RePlaceholder(Placeholder { universe: U1, name: BrAnon(0, None) }),
│ │ │ │ │                     '_#2r,
│ │ │ │ │                 ),
│ │ │ │ │                 [],
│ │ │ │ │             ),
│ │ │ │ │             BoringNoLocation,
│ │ │ │ │         ),
│ │ │ │ │         (
│ │ │ │ │             Binder(
│ │ │ │ │                 OutlivesPredicate(
│ │ │ │ │                     RePlaceholder(Placeholder { universe: U1, name: BrAnon(1, Some(lib.rs:8:14: 8:17 (#0))) }),
│ │ │ │ │                     '_#2r,
│ │ │ │ │                 ),
│ │ │ │ │                 [],
│ │ │ │ │             ),
│ │ │ │ │             BoringNoLocation,
│ │ │ │ │         ),
│ │ │ │ │         (
│ │ │ │ │             Binder(
│ │ │ │ │                 OutlivesPredicate(
│ │ │ │ │                     RePlaceholder(Placeholder { universe: U1, name: BrAnon(0, None) }),
│ │ │ │ │                     '_#3r,
│ │ │ │ │                 ),
│ │ │ │ │                 [],
│ │ │ │ │             ),
│ │ │ │ │             BoringNoLocation,
│ │ │ │ │         ),
│ │ │ │ │         (
│ │ │ │ │             Binder(
│ │ │ │ │                 OutlivesPredicate(
│ │ │ │ │                     RePlaceholder(Placeholder { universe: U1, name: BrAnon(0, None) }),
│ │ │ │ │                     '_#4r,
│ │ │ │ │                 ),
│ │ │ │ │                 [],
│ │ │ │ │             ),
│ │ │ │ │             BoringNoLocation,
│ │ │ │ │         ),
│ │ │ │ │     ],
│ │ │ │ │     member_constraints: [],
│ │ │ │ │ }
│ │ │ │ ├─┐rustc_borrowck::type_check::constraint_conversion::convert_all query_constraints=QueryRegionConstraints { outlives: [(Binder(OutlivesPredicate('_#2r, RePlaceholder(Placeholder { universe: U1, name: BrAnon(0, None) })), []), BoringNoLocation), (Binder(OutlivesPredicate('_#3r, RePlaceholder(Placeholder { universe: U1, name: BrAnon(0, None) })), []), BoringNoLocation), (Binder(OutlivesPredicate('_#4r, RePlaceholder(Placeholder { universe: U1, name: BrAnon(0, None) })), []), BoringNoLocation), (Binder(OutlivesPredicate('_#2r, RePlaceholder(Placeholder { universe: U1, name: BrAnon(1, Some(lib.rs:8:14: 8:17 (#0))) })), []), BoringNoLocation), (Binder(OutlivesPredicate(RePlaceholder(Placeholder { universe: U1, name: BrAnon(0, None) }), '_#2r), []), BoringNoLocation), (Binder(OutlivesPredicate(RePlaceholder(Placeholder { universe: U1, name: BrAnon(1, Some(lib.rs:8:14: 8:17 (#0))) }), '_#2r), []), BoringNoLocation), (Binder(OutlivesPredicate(RePlaceholder(Placeholder { universe: U1, name: BrAnon(0, None) }), '_#3r), []), BoringNoLocation), (Binder(OutlivesPredicate(RePlaceholder(Placeholder { universe: U1, name: BrAnon(0, None) }), '_#4r), []), BoringNoLocation)], member_constraints: [] }

...

├─┐rustc_borrowck::region_infer::best_blame_constraint from_region='_#5r, from_region_origin=Placeholder(Placeholder { universe: U1, name: BrAnon(0, None) })
