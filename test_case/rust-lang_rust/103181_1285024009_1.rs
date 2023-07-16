
error[E0425]: cannot find value `ident_error` in this scope
 --> src/main.rs:4:5
  |
4 |     ident_error;
  |     ^^^^^^^^^^^ not found in this scope


error: internal compiler error: compiler/rustc_trait_selection/src/traits/query/normalize.rs:256:21:

unexpected ambiguity: 

Canonical {
     max_universe: U0, variables: [], value: ParamEnvAnd {
         param_env: ParamEnv {
             caller_bounds: [Binder(TraitPredicate(<B as std::marker::Send>, polarity:Positive), []), Binder(TraitPredicate(<A as std::marker::Send>, polarity:Positive), []), Binder(TraitPredicate(<B as std::marker::Sized>, polarity:Positive), []), Binder(TraitPredicate(<A as std::marker::Sized>, polarity:Positive), [])], reveal: UserFacing, constness: NotConst 
        }, value: ProjectionTy {
             substs: [hyper::Server<hyper::server::conn::AddrIncoming, hyper::service::make::MakeServiceFn<[closure@src/main.rs:14:51: 14:54]>>], item_def_id: DefId(2:14053 ~ core[b141]::future::into_future::IntoFuture::IntoFuture) 
        } 
    } 
}

Canonical {
     max_universe: U39, variables: [CanonicalVarInfo {
         kind: Region(U39) 
    }, CanonicalVarInfo {
         kind: Region(U39) 
    }, CanonicalVarInfo {
         kind: Region(U16) 
    }, CanonicalVarInfo {
         kind: Region(U39) 
    }, CanonicalVarInfo {
         kind: Region(U39) 
    }, CanonicalVarInfo {
         kind: Region(U39) 
    }, CanonicalVarInfo {
         kind: Region(U39) 
    }, CanonicalVarInfo {
         kind: Region(U1) 
    }, CanonicalVarInfo {
         kind: PlaceholderRegion(Placeholder {
             universe: U1, name: BrNamed(DefId(21:1013 ~ hyper[0b84]::service::make::{
                impl#2
            }::'a), 'a) 
        }) 
    }, CanonicalVarInfo {
         kind: Region(U2) 
    }, CanonicalVarInfo {
         kind: PlaceholderRegion(Placeholder {
             universe: U2, name: BrNamed(DefId(21:1013 ~ hyper[0b84]::service::make::{
                impl#2
            }::'a), 'a) 
        }) 
    }, CanonicalVarInfo {
         kind: Region(U3) 
    }, CanonicalVarInfo {
         kind: PlaceholderRegion(Placeholder {
             universe: U3, name: BrNamed(DefId(21:1013 ~ hyper[0b84]::service::make::{
                impl#2
            }::'a), 'a) 
        }) 
    }, CanonicalVarInfo {
         kind: Region(U4) 
    }, CanonicalVarInfo {
         kind: PlaceholderRegion(Placeholder {
             universe: U4, name: BrNamed(DefId(21:1013 ~ hyper[0b84]::service::make::{
                impl#2
            }::'a), 'a) 
        }) 
    }, CanonicalVarInfo {
         kind: Region(U5) 
    }, CanonicalVarInfo {
         kind: PlaceholderRegion(Placeholder {
             universe: U5, name: BrNamed(DefId(21:1013 ~ hyper[0b84]::service::make::{
                impl#2
            }::'a), 'a) 
        }) 
    }, CanonicalVarInfo {
         kind: Region(U6) 
    }, CanonicalVarInfo {
         kind: PlaceholderRegion(Placeholder {
             universe: U6, name: BrNamed(DefId(21:1013 ~ hyper[0b84]::service::make::{
                impl#2
            }::'a), 'a) 
        }) 
    }, CanonicalVarInfo {
         kind: Region(U7) 
    }, CanonicalVarInfo {
         kind: PlaceholderRegion(Placeholder {
             universe: U7, name: BrNamed(DefId(21:1013 ~ hyper[0b84]::service::make::{
                impl#2
            }::'a), 'a) 
        }) 
    }, CanonicalVarInfo {
         kind: Region(U8) 
    }, CanonicalVarInfo {
         kind: PlaceholderRegion(Placeholder {
             universe: U8, name: BrNamed(DefId(21:1013 ~ hyper[0b84]::service::make::{
                impl#2
            }::'a), 'a) 
        }) 
    }, CanonicalVarInfo {
         kind: Region(U9) 
    }, CanonicalVarInfo {
         kind: PlaceholderRegion(Placeholder {
             universe: U9, name: BrNamed(DefId(21:1013 ~ hyper[0b84]::service::make::{
                impl#2
            }::'a), 'a) 
        }) 
    }, CanonicalVarInfo {
         kind: Region(U10) 
    }, CanonicalVarInfo {
         kind: PlaceholderRegion(Placeholder {
             universe: U10, name: BrNamed(DefId(21:1013 ~ hyper[0b84]::service::make::{
                impl#2
            }::'a), 'a) 
        }) 
    }, CanonicalVarInfo {
         kind: Region(U11) 
    }, CanonicalVarInfo {
         kind: PlaceholderRegion(Placeholder {
             universe: U11, name: BrNamed(DefId(21:1013 ~ hyper[0b84]::service::make::{
                impl#2
            }::'a), 'a) 
        }) 
    }, CanonicalVarInfo {
         kind: Region(U12) 
    }, CanonicalVarInfo {
         kind: PlaceholderRegion(Placeholder {
             universe: U12, name: BrNamed(DefId(21:1013 ~ hyper[0b84]::service::make::{
                impl#2
            }::'a), 'a) 
        }) 
    }, CanonicalVarInfo {
         kind: Region(U13) 
    }, CanonicalVarInfo {
         kind: PlaceholderRegion(Placeholder {
             universe: U13, name: BrNamed(DefId(21:1013 ~ hyper[0b84]::service::make::{
                impl#2
            }::'a), 'a) 
        }) 
    }, CanonicalVarInfo {
         kind: Region(U14) 
    }, CanonicalVarInfo {
         kind: PlaceholderRegion(Placeholder {
             universe: U14, name: BrNamed(DefId(21:1013 ~ hyper[0b84]::service::make::{
                impl#2
            }::'a), 'a) 
        }) 
    }, CanonicalVarInfo {
         kind: Region(U15) 
    }, CanonicalVarInfo {
         kind: PlaceholderRegion(Placeholder {
             universe: U15, name: BrNamed(DefId(21:1013 ~ hyper[0b84]::service::make::{
                impl#2
            }::'a), 'a) 
        }) 
    }, CanonicalVarInfo {
         kind: Region(U16) 
    }, CanonicalVarInfo {
         kind: PlaceholderRegion(Placeholder {
             universe: U16, name: BrNamed(DefId(21:1013 ~ hyper[0b84]::service::make::{
                impl#2
            }::'a), 'a) 
        }) 
    }, CanonicalVarInfo {
         kind: Region(U17) 
    }, CanonicalVarInfo {
         kind: PlaceholderRegion(Placeholder {
             universe: U17, name: BrNamed(DefId(21:1013 ~ hyper[0b84]::service::make::{
                impl#2
            }::'a), 'a) 
        }) 
    }, CanonicalVarInfo {
         kind: Region(U18) 
    }, CanonicalVarInfo {
         kind: PlaceholderRegion(Placeholder {
             universe: U18, name: BrNamed(DefId(21:1013 ~ hyper[0b84]::service::make::{
                impl#2
            }::'a), 'a) 
        }) 
    }, CanonicalVarInfo {
         kind: Region(U19) 
    }, CanonicalVarInfo {
         kind: PlaceholderRegion(Placeholder {
             universe: U19, name: BrNamed(DefId(21:1013 ~ hyper[0b84]::service::make::{
                impl#2
            }::'a), 'a) 
        }) 
    }, CanonicalVarInfo {
         kind: Region(U20) 
    }, CanonicalVarInfo {
         kind: PlaceholderRegion(Placeholder {
             universe: U20, name: BrNamed(DefId(21:10686 ~ hyper[0b84]::service::make::{
                impl#4
            }::'_), '_) 
        }) 
    }, CanonicalVarInfo {
         kind: Region(U20) 
    }, CanonicalVarInfo {
         kind: Region(U21) 
    }, CanonicalVarInfo {
         kind: PlaceholderRegion(Placeholder {
             universe: U21, name: BrNamed(DefId(21:10686 ~ hyper[0b84]::service::make::{
                impl#4
            }::'_), '_) 
        }) 
    }, CanonicalVarInfo {
         kind: Region(U22) 
    }, CanonicalVarInfo {
         kind: PlaceholderRegion(Placeholder {
             universe: U22, name: BrNamed(DefId(21:10686 ~ hyper[0b84]::service::make::{
                impl#4
            }::'_), '_) 
        }) 
    }, CanonicalVarInfo {
         kind: Region(U22) 
    }, CanonicalVarInfo {
         kind: Region(U23) 
    }, CanonicalVarInfo {
         kind: PlaceholderRegion(Placeholder {
             universe: U23, name: BrNamed(DefId(21:10686 ~ hyper[0b84]::service::make::{
                impl#4
            }::'_), '_) 
        }) 
    }, CanonicalVarInfo {
         kind: Region(U23) 
    }, CanonicalVarInfo {
         kind: Region(U24) 
    }, CanonicalVarInfo {
         kind: PlaceholderRegion(Placeholder {
             universe: U24, name: BrNamed(DefId(21:10686 ~ hyper[0b84]::service::make::{
                impl#4
            }::'_), '_) 
        }) 
    }, CanonicalVarInfo {
         kind: Region(U24) 
    }, CanonicalVarInfo {
         kind: Region(U25) 
    }, CanonicalVarInfo {
         kind: PlaceholderRegion(Placeholder {
             universe: U25, name: BrNamed(DefId(21:10686 ~ hyper[0b84]::service::make::{
                impl#4
            }::'_), '_) 
        }) 
    }, CanonicalVarInfo {
         kind: Region(U25) 
    }, CanonicalVarInfo {
         kind: Region(U26) 
    }, CanonicalVarInfo {
         kind: PlaceholderRegion(Placeholder {
             universe: U26, name: BrNamed(DefId(21:10686 ~ hyper[0b84]::service::make::{
                impl#4
            }::'_), '_) 
        }) 
    }, CanonicalVarInfo {
         kind: Region(U26) 
    }, CanonicalVarInfo {
         kind: Region(U27) 
    }, CanonicalVarInfo {
         kind: PlaceholderRegion(Placeholder {
             universe: U27, name: BrNamed(DefId(21:10686 ~ hyper[0b84]::service::make::{
                impl#4
            }::'_), '_) 
        }) 
    }, CanonicalVarInfo {
         kind: Region(U27) 
    }, CanonicalVarInfo {
         kind: Region(U28) 
    }, CanonicalVarInfo {
         kind: PlaceholderRegion(Placeholder {
             universe: U28, name: BrNamed(DefId(21:10686 ~ hyper[0b84]::service::make::{
                impl#4
            }::'_), '_) 
        }) 
    }, CanonicalVarInfo {
         kind: Region(U28) 
    }, CanonicalVarInfo {
         kind: Region(U29) 
    }, CanonicalVarInfo {
         kind: PlaceholderRegion(Placeholder {
             universe: U29, name: BrNamed(DefId(21:10686 ~ hyper[0b84]::service::make::{
                impl#4
            }::'_), '_) 
        }) 
    }, CanonicalVarInfo {
         kind: Region(U29) 
    }, CanonicalVarInfo {
         kind: Region(U30) 
    }, CanonicalVarInfo {
         kind: PlaceholderRegion(Placeholder {
             universe: U30, name: BrNamed(DefId(21:10686 ~ hyper[0b84]::service::make::{
                impl#4
            }::'_), '_) 
        }) 
    }, CanonicalVarInfo {
         kind: Region(U30) 
    }, CanonicalVarInfo {
         kind: Region(U31) 
    }, CanonicalVarInfo {
         kind: PlaceholderRegion(Placeholder {
             universe: U31, name: BrNamed(DefId(21:10686 ~ hyper[0b84]::service::make::{
                impl#4
            }::'_), '_) 
        }) 
    }, CanonicalVarInfo {
         kind: Region(U31) 
    }, CanonicalVarInfo {
         kind: Region(U32) 
    }, CanonicalVarInfo {
         kind: PlaceholderRegion(Placeholder {
             universe: U32, name: BrNamed(DefId(21:10686 ~ hyper[0b84]::service::make::{
                impl#4
            }::'_), '_) 
        }) 
    }, CanonicalVarInfo {
         kind: Region(U32) 
    }, CanonicalVarInfo {
         kind: Region(U33) 
    }, CanonicalVarInfo {
         kind: PlaceholderRegion(Placeholder {
             universe: U33, name: BrNamed(DefId(21:10686 ~ hyper[0b84]::service::make::{
                impl#4
            }::'_), '_) 
        }) 
    }, CanonicalVarInfo {
         kind: Region(U33) 
    }, CanonicalVarInfo {
         kind: Region(U34) 
    }, CanonicalVarInfo {
         kind: PlaceholderRegion(Placeholder {
             universe: U34, name: BrNamed(DefId(21:10686 ~ hyper[0b84]::service::make::{
                impl#4
            }::'_), '_) 
        }) 
    }, CanonicalVarInfo {
         kind: Region(U34) 
    }, CanonicalVarInfo {
         kind: Region(U35) 
    }, CanonicalVarInfo {
         kind: PlaceholderRegion(Placeholder {
             universe: U35, name: BrNamed(DefId(21:10686 ~ hyper[0b84]::service::make::{
                impl#4
            }::'_), '_) 
        }) 
    }, CanonicalVarInfo {
         kind: Region(U35) 
    }, CanonicalVarInfo {
         kind: Region(U36) 
    }, CanonicalVarInfo {
         kind: PlaceholderRegion(Placeholder {
             universe: U36, name: BrNamed(DefId(21:10686 ~ hyper[0b84]::service::make::{
                impl#4
            }::'_), '_) 
        }) 
    }, CanonicalVarInfo {
         kind: Region(U36) 
    }, CanonicalVarInfo {
         kind: Region(U37) 
    }, CanonicalVarInfo {
         kind: PlaceholderRegion(Placeholder {
             universe: U37, name: BrNamed(DefId(21:10686 ~ hyper[0b84]::service::make::{
                impl#4
            }::'_), '_) 
        }) 
    }, CanonicalVarInfo {
         kind: Region(U37) 
    }, CanonicalVarInfo {
         kind: Region(U38) 
    }, CanonicalVarInfo {
         kind: PlaceholderRegion(Placeholder {
             universe: U38, name: BrNamed(DefId(21:10686 ~ hyper[0b84]::service::make::{
                impl#4
            }::'_), '_) 
        }) 
    }, CanonicalVarInfo {
         kind: Region(U38) 
    }, CanonicalVarInfo {
         kind: Region(U39) 
    }, CanonicalVarInfo {
         kind: PlaceholderRegion(Placeholder {
             universe: U39, name: BrNamed(DefId(21:10686 ~ hyper[0b84]::service::make::{
                impl#4
            }::'_), '_) 
        }) 
    }, CanonicalVarInfo {
         kind: Region(U39) 
    }], value: QueryResponse {
         var_values: CanonicalVarValues {
             var_values: [] 
        }, region_constraints: QueryRegionConstraints {
             outlives: [(Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 0, kind: BrAnon(0) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 0, kind: BrAnon(0) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 1, kind: BrAnon(1) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 1, kind: BrAnon(1) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 0, kind: BrAnon(0) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 0, kind: BrAnon(0) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 1, kind: BrAnon(1) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 1, kind: BrAnon(1) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 2, kind: BrAnon(2) 
            }), ReStatic), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 3, kind: BrAnon(3) 
            }), ReStatic), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 4, kind: BrAnon(4) 
            }), ReStatic), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 5, kind: BrAnon(5) 
            }), ReStatic), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 6, kind: BrAnon(6) 
            }), ReStatic), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 7, kind: BrAnon(7) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 8, kind: BrAnon(8) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 9, kind: BrAnon(9) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 10, kind: BrAnon(10) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 11, kind: BrAnon(11) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 12, kind: BrAnon(12) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 13, kind: BrAnon(13) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 14, kind: BrAnon(14) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 15, kind: BrAnon(15) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 16, kind: BrAnon(16) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 17, kind: BrAnon(17) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 18, kind: BrAnon(18) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 19, kind: BrAnon(19) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 20, kind: BrAnon(20) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 21, kind: BrAnon(21) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 22, kind: BrAnon(22) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 23, kind: BrAnon(23) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 24, kind: BrAnon(24) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 25, kind: BrAnon(25) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 26, kind: BrAnon(26) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 27, kind: BrAnon(27) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 28, kind: BrAnon(28) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 29, kind: BrAnon(29) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 30, kind: BrAnon(30) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 31, kind: BrAnon(31) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 32, kind: BrAnon(32) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 33, kind: BrAnon(33) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 34, kind: BrAnon(34) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 35, kind: BrAnon(35) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 36, kind: BrAnon(36) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 37, kind: BrAnon(37) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 38, kind: BrAnon(38) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 39, kind: BrAnon(39) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 40, kind: BrAnon(40) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 41, kind: BrAnon(41) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 42, kind: BrAnon(42) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 43, kind: BrAnon(43) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 44, kind: BrAnon(44) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 45, kind: BrAnon(45) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 46, kind: BrAnon(46) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 47, kind: BrAnon(47) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 46, kind: BrAnon(46) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 48, kind: BrAnon(48) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 49, kind: BrAnon(49) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 50, kind: BrAnon(50) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 51, kind: BrAnon(51) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 52, kind: BrAnon(52) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 51, kind: BrAnon(51) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 53, kind: BrAnon(53) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 54, kind: BrAnon(54) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 55, kind: BrAnon(55) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 54, kind: BrAnon(54) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 56, kind: BrAnon(56) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 57, kind: BrAnon(57) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 58, kind: BrAnon(58) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 57, kind: BrAnon(57) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 59, kind: BrAnon(59) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 60, kind: BrAnon(60) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 61, kind: BrAnon(61) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 60, kind: BrAnon(60) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 62, kind: BrAnon(62) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 63, kind: BrAnon(63) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 64, kind: BrAnon(64) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 63, kind: BrAnon(63) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 65, kind: BrAnon(65) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 66, kind: BrAnon(66) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 67, kind: BrAnon(67) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 66, kind: BrAnon(66) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 68, kind: BrAnon(68) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 69, kind: BrAnon(69) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 70, kind: BrAnon(70) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 69, kind: BrAnon(69) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 71, kind: BrAnon(71) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 72, kind: BrAnon(72) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 73, kind: BrAnon(73) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 72, kind: BrAnon(72) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 74, kind: BrAnon(74) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 75, kind: BrAnon(75) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 76, kind: BrAnon(76) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 75, kind: BrAnon(75) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 77, kind: BrAnon(77) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 78, kind: BrAnon(78) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 79, kind: BrAnon(79) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 78, kind: BrAnon(78) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 80, kind: BrAnon(80) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 81, kind: BrAnon(81) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 82, kind: BrAnon(82) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 81, kind: BrAnon(81) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 83, kind: BrAnon(83) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 84, kind: BrAnon(84) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 85, kind: BrAnon(85) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 84, kind: BrAnon(84) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 86, kind: BrAnon(86) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 87, kind: BrAnon(87) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 88, kind: BrAnon(88) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 87, kind: BrAnon(87) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 89, kind: BrAnon(89) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 90, kind: BrAnon(90) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 91, kind: BrAnon(91) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 90, kind: BrAnon(90) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 92, kind: BrAnon(92) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 93, kind: BrAnon(93) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 94, kind: BrAnon(94) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 93, kind: BrAnon(93) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 95, kind: BrAnon(95) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 96, kind: BrAnon(96) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 97, kind: BrAnon(97) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 96, kind: BrAnon(96) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 98, kind: BrAnon(98) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 99, kind: BrAnon(99) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 100, kind: BrAnon(100) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 99, kind: BrAnon(99) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 101, kind: BrAnon(101) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 102, kind: BrAnon(102) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 103, kind: BrAnon(103) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 102, kind: BrAnon(102) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 8, kind: BrAnon(8) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 7, kind: BrAnon(7) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 10, kind: BrAnon(10) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 9, kind: BrAnon(9) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 12, kind: BrAnon(12) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 11, kind: BrAnon(11) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 14, kind: BrAnon(14) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 13, kind: BrAnon(13) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 16, kind: BrAnon(16) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 15, kind: BrAnon(15) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 18, kind: BrAnon(18) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 17, kind: BrAnon(17) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 20, kind: BrAnon(20) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 19, kind: BrAnon(19) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 22, kind: BrAnon(22) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 21, kind: BrAnon(21) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 24, kind: BrAnon(24) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 23, kind: BrAnon(23) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 26, kind: BrAnon(26) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 25, kind: BrAnon(25) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 28, kind: BrAnon(28) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 27, kind: BrAnon(27) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 30, kind: BrAnon(30) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 29, kind: BrAnon(29) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 32, kind: BrAnon(32) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 31, kind: BrAnon(31) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 34, kind: BrAnon(34) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 33, kind: BrAnon(33) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 36, kind: BrAnon(36) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 35, kind: BrAnon(35) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 38, kind: BrAnon(38) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 37, kind: BrAnon(37) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 40, kind: BrAnon(40) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 39, kind: BrAnon(39) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 42, kind: BrAnon(42) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 41, kind: BrAnon(41) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 44, kind: BrAnon(44) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 43, kind: BrAnon(43) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 46, kind: BrAnon(46) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 45, kind: BrAnon(45) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 46, kind: BrAnon(46) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 47, kind: BrAnon(47) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 49, kind: BrAnon(49) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 48, kind: BrAnon(48) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 51, kind: BrAnon(51) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 50, kind: BrAnon(50) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 51, kind: BrAnon(51) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 52, kind: BrAnon(52) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 54, kind: BrAnon(54) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 53, kind: BrAnon(53) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 54, kind: BrAnon(54) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 55, kind: BrAnon(55) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 57, kind: BrAnon(57) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 56, kind: BrAnon(56) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 57, kind: BrAnon(57) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 58, kind: BrAnon(58) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 60, kind: BrAnon(60) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 59, kind: BrAnon(59) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 60, kind: BrAnon(60) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 61, kind: BrAnon(61) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 63, kind: BrAnon(63) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 62, kind: BrAnon(62) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 63, kind: BrAnon(63) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 64, kind: BrAnon(64) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 66, kind: BrAnon(66) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 65, kind: BrAnon(65) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 66, kind: BrAnon(66) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 67, kind: BrAnon(67) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 69, kind: BrAnon(69) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 68, kind: BrAnon(68) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 69, kind: BrAnon(69) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 70, kind: BrAnon(70) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 72, kind: BrAnon(72) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 71, kind: BrAnon(71) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 72, kind: BrAnon(72) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 73, kind: BrAnon(73) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 75, kind: BrAnon(75) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 74, kind: BrAnon(74) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 75, kind: BrAnon(75) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 76, kind: BrAnon(76) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 78, kind: BrAnon(78) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 77, kind: BrAnon(77) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 78, kind: BrAnon(78) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 79, kind: BrAnon(79) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 81, kind: BrAnon(81) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 80, kind: BrAnon(80) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 81, kind: BrAnon(81) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 82, kind: BrAnon(82) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 84, kind: BrAnon(84) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 83, kind: BrAnon(83) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 84, kind: BrAnon(84) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 85, kind: BrAnon(85) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 87, kind: BrAnon(87) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 86, kind: BrAnon(86) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 87, kind: BrAnon(87) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 88, kind: BrAnon(88) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 90, kind: BrAnon(90) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 89, kind: BrAnon(89) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 90, kind: BrAnon(90) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 91, kind: BrAnon(91) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 93, kind: BrAnon(93) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 92, kind: BrAnon(92) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 93, kind: BrAnon(93) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 94, kind: BrAnon(94) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 96, kind: BrAnon(96) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 95, kind: BrAnon(95) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 96, kind: BrAnon(96) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 97, kind: BrAnon(97) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 99, kind: BrAnon(99) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 98, kind: BrAnon(98) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 99, kind: BrAnon(99) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 100, kind: BrAnon(100) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 102, kind: BrAnon(102) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 101, kind: BrAnon(101) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 102, kind: BrAnon(102) 
            }), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 103, kind: BrAnon(103) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(hyper::server::conn::AddrStream, ReStatic), []), BoringNoLocation), (Binder(OutlivesPredicate(hyper::Body, ReStatic), []), BoringNoLocation), (Binder(OutlivesPredicate(hyper::proto::h2::server::H2Stream<impl futures::Future<Output = std::result::Result<hyper::Response<hyper::Body>, std::convert::Infallible>>, hyper::Body>, ReStatic), []), BoringNoLocation), (Binder(OutlivesPredicate(hyper::server::server::new_svc::NewSvcTask<hyper::server::conn::AddrStream, impl futures::Future<Output = std::result::Result<hyper::service::util::ServiceFn<[closure@src/main.rs:16:60: 16:63], hyper::Body>, std::convert::Infallible>>, hyper::service::util::ServiceFn<[closure@src/main.rs:16:60: 16:63], hyper::Body>, hyper::common::exec::Exec, hyper::server::server::NoopWatcher>, ReStatic), []), BoringNoLocation), (Binder(OutlivesPredicate(std::io::Error, ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 2, kind: BrAnon(2) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(hyper::body::Bytes, ReStatic), []), BoringNoLocation), (Binder(OutlivesPredicate(hyper::body::Bytes, ReStatic), []), BoringNoLocation), (Binder(OutlivesPredicate(hyper::server::conn::AddrStream, ReStatic), []), BoringNoLocation), (Binder(OutlivesPredicate(hyper::Body, ReStatic), []), BoringNoLocation), (Binder(OutlivesPredicate(hyper::Body, ReStatic), []), BoringNoLocation), (Binder(OutlivesPredicate(hyper::Body, ReStatic), []), BoringNoLocation), (Binder(OutlivesPredicate(std::convert::Infallible, ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 3, kind: BrAnon(3) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(hyper::proto::h2::server::H2Stream<impl futures::Future<Output = std::result::Result<hyper::Response<hyper::Body>, std::convert::Infallible>>, hyper::Body>, ReStatic), []), BoringNoLocation), (Binder(OutlivesPredicate(hyper::proto::h2::server::H2Stream<impl futures::Future<Output = std::result::Result<hyper::Response<hyper::Body>, std::convert::Infallible>>, hyper::Body>, ReStatic), []), BoringNoLocation), (Binder(OutlivesPredicate(hyper::proto::h2::server::H2Stream<impl futures::Future<Output = std::result::Result<hyper::Response<hyper::Body>, std::convert::Infallible>>, hyper::Body>, ReStatic), []), BoringNoLocation), (Binder(OutlivesPredicate((), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 0, kind: BrAnon(0) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate(hyper::body::Bytes, ReStatic), []), BoringNoLocation), (Binder(OutlivesPredicate(hyper::Error, ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 4, kind: BrAnon(4) 
            })), []), BoringNoLocation), (Binder(OutlivesPredicate((), ReLateBound(DebruijnIndex(1), BoundRegion {
                 var: 1, kind: BrAnon(1) 
            })), []), BoringNoLocation)], member_constraints: [] 
        }, certainty: Ambiguous, opaque_types: [], value: NormalizationResult {
             normalized_ty: hyper::Server<hyper::server::conn::AddrIncoming, hyper::service::make::MakeServiceFn<[closure@src/main.rs:14:51: 14:54]>> 
        } 
    } 
}

thread 'rustc' panicked at 'Box<dyn Any>', /rustc/4b8f4319954ff2642690b9e5cbe4af352d095bf6/compiler/rustc_errors/src/lib.rs:1516:9
stack backtrace:
   0:     0x7fe954d256e0 - std::backtrace_rs::backtrace::libunwind::trace::h2119464896b45b57
                               at /rustc/4b8f4319954ff2642690b9e5cbe4af352d095bf6/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   1:     0x7fe954d256e0 - std::backtrace_rs::backtrace::trace_unsynchronized::h99b1d9b13310118f
                               at /rustc/4b8f4319954ff2642690b9e5cbe4af352d095bf6/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fe954d256e0 - std::sys_common::backtrace::_print_fmt::hbe6d08ff752ef116
                               at /rustc/4b8f4319954ff2642690b9e5cbe4af352d095bf6/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7fe954d256e0 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h6dd4a5ce328f2e15
                               at /rustc/4b8f4319954ff2642690b9e5cbe4af352d095bf6/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7fe954d815fe - core::fmt::write::h4593006cca976c9b
                               at /rustc/4b8f4319954ff2642690b9e5cbe4af352d095bf6/library/core/src/fmt/mod.rs:1209:17
   5:     0x7fe954d15855 - std::io::Write::write_fmt::h75d064e88170c1b4
                               at /rustc/4b8f4319954ff2642690b9e5cbe4af352d095bf6/library/std/src/io/mod.rs:1682:15
   6:     0x7fe954d254a5 - std::sys_common::backtrace::_print::hfe138f63dad5fb4c
                               at /rustc/4b8f4319954ff2642690b9e5cbe4af352d095bf6/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7fe954d254a5 - std::sys_common::backtrace::print::he836ff7168a1f810
                               at /rustc/4b8f4319954ff2642690b9e5cbe4af352d095bf6/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7fe954d282af - std::panicking::default_hook::{{closure}}::h4a0ff709ff17404d
                               at /rustc/4b8f4319954ff2642690b9e5cbe4af352d095bf6/library/std/src/panicking.rs:267:22
   9:     0x7fe954d27fea - std::panicking::default_hook::h45465b3e5f51d8d9
                               at /rustc/4b8f4319954ff2642690b9e5cbe4af352d095bf6/library/std/src/panicking.rs:286:9
  10:     0x7fe95765a811 - <rustc_driver[2feadf6a045f69f8]::DEFAULT_HOOK::{closure#0}::{closure#0} as core[b141a5f448cbfdbc]::ops::function::FnOnce<(&core[b141a5f448cbfdbc]::panic::panic_info::PanicInfo,)>>::call_once::{shim:vtable#0}
  11:     0x7fe954d28ad9 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::h2f9ca2b8279bca39
                               at /rustc/4b8f4319954ff2642690b9e5cbe4af352d095bf6/library/alloc/src/boxed.rs:2001:9
  12:     0x7fe954d28ad9 - std::panicking::rust_panic_with_hook::he67ffd9c867b0e23
                               at /rustc/4b8f4319954ff2642690b9e5cbe4af352d095bf6/library/std/src/panicking.rs:692:13
  13:     0x7fe958600831 - std[5adef4932411bbc2]::panicking::begin_panic::<rustc_errors[3c325fb77ff98e3b]::ExplicitBug>::{closure#0}
  14:     0x7fe9585ff146 - std[5adef4932411bbc2]::sys_common::backtrace::__rust_end_short_backtrace::<std[5adef4932411bbc2]::panicking::begin_panic<rustc_errors[3c325fb77ff98e3b]::ExplicitBug>::{closure#0}, !>
  15:     0x7fe958668d36 - std[5adef4932411bbc2]::panicking::begin_panic::<rustc_errors[3c325fb77ff98e3b]::ExplicitBug>
  16:     0x7fe9585fe0b6 - std[5adef4932411bbc2]::panic::panic_any::<rustc_errors[3c325fb77ff98e3b]::ExplicitBug>
  17:     0x7fe9585fdfcd - <rustc_errors[3c325fb77ff98e3b]::HandlerInner>::bug::<&alloc[7894da9ceb74614d]::string::String>
  18:     0x7fe9585fda40 - <rustc_errors[3c325fb77ff98e3b]::Handler>::bug::<&alloc[7894da9ceb74614d]::string::String>
  19:     0x7fe9586b25cd - rustc_middle[d605453e4543da8c]::ty::context::tls::with_context_opt::<rustc_middle[d605453e4543da8c]::ty::context::tls::with_opt<rustc_middle[d605453e4543da8c]::util::bug::opt_span_bug_fmt<rustc_span[f76be64242de1050]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  20:     0x7fe9586b2966 - rustc_middle[d605453e4543da8c]::util::bug::opt_span_bug_fmt::<rustc_span[f76be64242de1050]::span_encoding::Span>
  21:     0x7fe955f000b3 - rustc_middle[d605453e4543da8c]::util::bug::bug_fmt
  22:     0x7fe9569482af - <rustc_trait_selection[63ea626d0d9dd5f]::traits::query::normalize::QueryNormalizer as rustc_middle[d605453e4543da8c]::ty::fold::FallibleTypeFolder>::try_fold_ty
  23:     0x7fe956cd6c0e - <&rustc_middle[d605453e4543da8c]::ty::list::List<rustc_middle[d605453e4543da8c]::ty::Ty> as rustc_middle[d605453e4543da8c]::ty::fold::TypeFoldable>::try_fold_with::<rustc_trait_selection[63ea626d0d9dd5f]::traits::query::normalize::QueryNormalizer>
  24:     0x7fe956cd6845 - <rustc_infer[d3358ef915d99f18]::infer::at::At as rustc_trait_selection[63ea626d0d9dd5f]::traits::query::normalize::AtExt>::normalize::<rustc_middle[d605453e4543da8c]::ty::sty::FnSig>
  25:     0x7fe956cd5671 - rustc_traits[897ea7b5c414bd07]::type_op::type_op_normalize::<rustc_middle[d605453e4543da8c]::ty::sty::FnSig>
  26:     0x7fe956cd4b17 - <rustc_infer[d3358ef915d99f18]::infer::InferCtxtBuilder as rustc_trait_selection[63ea626d0d9dd5f]::infer::InferCtxtBuilderExt>::enter_canonical_trait_query::<rustc_middle[d605453e4543da8c]::ty::ParamEnvAnd<rustc_middle[d605453e4543da8c]::traits::query::type_op::Normalize<rustc_middle[d605453e4543da8c]::ty::sty::FnSig>>, rustc_middle[d605453e4543da8c]::ty::sty::FnSig, rustc_traits[897ea7b5c414bd07]::type_op::type_op_normalize<rustc_middle[d605453e4543da8c]::ty::sty::FnSig>>
  27:     0x7fe956cd47c6 - rustc_traits[897ea7b5c414bd07]::type_op::type_op_normalize_fn_sig
  28:     0x7fe956f3d307 - rustc_query_system[e96ca7e2da71ed3f]::query::plumbing::try_execute_query::<rustc_query_impl[cd5064426dba224b]::plumbing::QueryCtxt, rustc_query_system[e96ca7e2da71ed3f]::query::caches::DefaultCache<rustc_middle[d605453e4543da8c]::infer::canonical::Canonical<rustc_middle[d605453e4543da8c]::ty::ParamEnvAnd<rustc_middle[d605453e4543da8c]::traits::query::type_op::Normalize<rustc_middle[d605453e4543da8c]::ty::sty::FnSig>>>, core[b141a5f448cbfdbc]::result::Result<&rustc_middle[d605453e4543da8c]::infer::canonical::Canonical<rustc_middle[d605453e4543da8c]::infer::canonical::QueryResponse<rustc_middle[d605453e4543da8c]::ty::sty::FnSig>>, rustc_middle[d605453e4543da8c]::traits::query::NoSolution>>>
  29:     0x7fe956f3cf0b - rustc_query_system[e96ca7e2da71ed3f]::query::plumbing::get_query::<rustc_query_impl[cd5064426dba224b]::queries::type_op_normalize_fn_sig, rustc_query_impl[cd5064426dba224b]::plumbing::QueryCtxt>
  30:     0x7fe956f3ce4e - <rustc_query_impl[cd5064426dba224b]::Queries as rustc_middle[d605453e4543da8c]::ty::query::QueryEngine>::type_op_normalize_fn_sig
  31:     0x7fe956cb8227 - <rustc_middle[d605453e4543da8c]::ty::sty::FnSig as rustc_trait_selection[63ea626d0d9dd5f]::traits::query::type_op::normalize::Normalizable>::type_op_method
  32:     0x7fe956cb60d3 - <rustc_middle[d605453e4543da8c]::traits::query::type_op::Normalize<rustc_middle[d605453e4543da8c]::ty::sty::FnSig> as rustc_trait_selection[63ea626d0d9dd5f]::traits::query::type_op::QueryTypeOp>::fully_perform_into
  33:     0x7fe9565ce82a - <rustc_borrowck[bf646eb435ac545c]::type_check::TypeChecker>::typeck_mir
  34:     0x7fe9565463c2 - rustc_borrowck[bf646eb435ac545c]::type_check::type_check
  35:     0x7fe956537878 - rustc_borrowck[bf646eb435ac545c]::nll::compute_regions
  36:     0x7fe95650b6b8 - rustc_borrowck[bf646eb435ac545c]::do_mir_borrowck
  37:     0x7fe956509fe0 - rustc_borrowck[bf646eb435ac545c]::mir_borrowck
  38:     0x7fe956509971 - <rustc_borrowck[bf646eb435ac545c]::provide::{closure#0} as core[b141a5f448cbfdbc]::ops::function::FnOnce<(rustc_middle[d605453e4543da8c]::ty::context::TyCtxt, rustc_span[f76be64242de1050]::def_id::LocalDefId)>>::call_once
  39:     0x7fe956afca05 - rustc_query_system[e96ca7e2da71ed3f]::query::plumbing::try_execute_query::<rustc_query_impl[cd5064426dba224b]::plumbing::QueryCtxt, rustc_query_system[e96ca7e2da71ed3f]::query::caches::DefaultCache<rustc_span[f76be64242de1050]::def_id::LocalDefId, &rustc_middle[d605453e4543da8c]::mir::query::BorrowCheckResult>>
  40:     0x7fe957523f0e - <rustc_query_impl[cd5064426dba224b]::Queries as rustc_middle[d605453e4543da8c]::ty::query::QueryEngine>::mir_borrowck
  41:     0x7fe9565fa3e8 - <rustc_borrowck[bf646eb435ac545c]::type_check::TypeChecker>::prove_closure_bounds
  42:     0x7fe9565d5c40 - <rustc_borrowck[bf646eb435ac545c]::type_check::TypeChecker>::typeck_mir
  43:     0x7fe9565463c2 - rustc_borrowck[bf646eb435ac545c]::type_check::type_check
  44:     0x7fe956537878 - rustc_borrowck[bf646eb435ac545c]::nll::compute_regions
  45:     0x7fe95650b6b8 - rustc_borrowck[bf646eb435ac545c]::do_mir_borrowck
  46:     0x7fe956509fe0 - rustc_borrowck[bf646eb435ac545c]::mir_borrowck
  47:     0x7fe956509971 - <rustc_borrowck[bf646eb435ac545c]::provide::{closure#0} as core[b141a5f448cbfdbc]::ops::function::FnOnce<(rustc_middle[d605453e4543da8c]::ty::context::TyCtxt, rustc_span[f76be64242de1050]::def_id::LocalDefId)>>::call_once
  48:     0x7fe956afca05 - rustc_query_system[e96ca7e2da71ed3f]::query::plumbing::try_execute_query::<rustc_query_impl[cd5064426dba224b]::plumbing::QueryCtxt, rustc_query_system[e96ca7e2da71ed3f]::query::caches::DefaultCache<rustc_span[f76be64242de1050]::def_id::LocalDefId, &rustc_middle[d605453e4543da8c]::mir::query::BorrowCheckResult>>
  49:     0x7fe957523f0e - <rustc_query_impl[cd5064426dba224b]::Queries as rustc_middle[d605453e4543da8c]::ty::query::QueryEngine>::mir_borrowck
  50:     0x7fe9572ddd0c - rustc_hir_analysis[4ff8e3b274fef4a]::collect::type_of::type_of
  51:     0x7fe956b155dc - rustc_query_system[e96ca7e2da71ed3f]::query::plumbing::get_query::<rustc_query_impl[cd5064426dba224b]::queries::type_of, rustc_query_impl[cd5064426dba224b]::plumbing::QueryCtxt>
  52:     0x7fe956cf6291 - rustc_hir_analysis[4ff8e3b274fef4a]::check::check::check_mod_item_types
  53:     0x7fe956ce3c05 - rustc_query_system[e96ca7e2da71ed3f]::query::plumbing::try_execute_query::<rustc_query_impl[cd5064426dba224b]::plumbing::QueryCtxt, rustc_query_system[e96ca7e2da71ed3f]::query::caches::DefaultCache<rustc_span[f76be64242de1050]::def_id::LocalDefId, ()>>
  54:     0x7fe95707a8f9 - rustc_query_system[e96ca7e2da71ed3f]::query::plumbing::get_query::<rustc_query_impl[cd5064426dba224b]::queries::check_mod_item_types, rustc_query_impl[cd5064426dba224b]::plumbing::QueryCtxt>
  55:     0x7fe957340abc - <rustc_middle[d605453e4543da8c]::hir::map::Map>::for_each_module::<rustc_hir_analysis[4ff8e3b274fef4a]::check_crate::{closure#6}::{closure#0}>
  56:     0x7fe957045aa5 - rustc_hir_analysis[4ff8e3b274fef4a]::check_crate
  57:     0x7fe9570455d7 - rustc_interface[a32f49e72690e02b]::passes::analysis
  58:     0x7fe9573e6354 - rustc_query_system[e96ca7e2da71ed3f]::query::plumbing::try_execute_query::<rustc_query_impl[cd5064426dba224b]::plumbing::QueryCtxt, rustc_query_system[e96ca7e2da71ed3f]::query::caches::DefaultCache<(), core[b141a5f448cbfdbc]::result::Result<(), rustc_errors[3c325fb77ff98e3b]::ErrorGuaranteed>>>
  59:     0x7fe9573e6087 - rustc_query_system[e96ca7e2da71ed3f]::query::plumbing::get_query::<rustc_query_impl[cd5064426dba224b]::queries::analysis, rustc_query_impl[cd5064426dba224b]::plumbing::QueryCtxt>
  60:     0x7fe9563a08e3 - <rustc_interface[a32f49e72690e02b]::passes::QueryContext>::enter::<rustc_driver[2feadf6a045f69f8]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[b141a5f448cbfdbc]::result::Result<(), rustc_errors[3c325fb77ff98e3b]::ErrorGuaranteed>>
  61:     0x7fe95639c826 - <rustc_interface[a32f49e72690e02b]::interface::Compiler>::enter::<rustc_driver[2feadf6a045f69f8]::run_compiler::{closure#1}::{closure#2}, core[b141a5f448cbfdbc]::result::Result<core[b141a5f448cbfdbc]::option::Option<rustc_interface[a32f49e72690e02b]::queries::Linker>, rustc_errors[3c325fb77ff98e3b]::ErrorGuaranteed>>
  62:     0x7fe956393edc - rustc_span[f76be64242de1050]::with_source_map::<core[b141a5f448cbfdbc]::result::Result<(), rustc_errors[3c325fb77ff98e3b]::ErrorGuaranteed>, rustc_interface[a32f49e72690e02b]::interface::run_compiler<core[b141a5f448cbfdbc]::result::Result<(), rustc_errors[3c325fb77ff98e3b]::ErrorGuaranteed>, rustc_driver[2feadf6a045f69f8]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  63:     0x7fe9563938a2 - <scoped_tls[98436e20d4fee4d6]::ScopedKey<rustc_span[f76be64242de1050]::SessionGlobals>>::set::<rustc_interface[a32f49e72690e02b]::interface::run_compiler<core[b141a5f448cbfdbc]::result::Result<(), rustc_errors[3c325fb77ff98e3b]::ErrorGuaranteed>, rustc_driver[2feadf6a045f69f8]::run_compiler::{closure#1}>::{closure#0}, core[b141a5f448cbfdbc]::result::Result<(), rustc_errors[3c325fb77ff98e3b]::ErrorGuaranteed>>
  64:     0x7fe956391fcf - std[5adef4932411bbc2]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[a32f49e72690e02b]::util::run_in_thread_pool_with_globals<rustc_interface[a32f49e72690e02b]::interface::run_compiler<core[b141a5f448cbfdbc]::result::Result<(), rustc_errors[3c325fb77ff98e3b]::ErrorGuaranteed>, rustc_driver[2feadf6a045f69f8]::run_compiler::{closure#1}>::{closure#0}, core[b141a5f448cbfdbc]::result::Result<(), rustc_errors[3c325fb77ff98e3b]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b141a5f448cbfdbc]::result::Result<(), rustc_errors[3c325fb77ff98e3b]::ErrorGuaranteed>>
  65:     0x7fe956391e3f - <<std[5adef4932411bbc2]::thread::Builder>::spawn_unchecked_<rustc_interface[a32f49e72690e02b]::util::run_in_thread_pool_with_globals<rustc_interface[a32f49e72690e02b]::interface::run_compiler<core[b141a5f448cbfdbc]::result::Result<(), rustc_errors[3c325fb77ff98e3b]::ErrorGuaranteed>, rustc_driver[2feadf6a045f69f8]::run_compiler::{closure#1}>::{closure#0}, core[b141a5f448cbfdbc]::result::Result<(), rustc_errors[3c325fb77ff98e3b]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b141a5f448cbfdbc]::result::Result<(), rustc_errors[3c325fb77ff98e3b]::ErrorGuaranteed>>::{closure#1} as core[b141a5f448cbfdbc]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  66:     0x7fe954d325f3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hecb24e4d2d235ab4
                               at /rustc/4b8f4319954ff2642690b9e5cbe4af352d095bf6/library/alloc/src/boxed.rs:1987:9
  67:     0x7fe954d325f3 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h05e0c493dc58e461
                               at /rustc/4b8f4319954ff2642690b9e5cbe4af352d095bf6/library/alloc/src/boxed.rs:1987:9
  68:     0x7fe954d325f3 - std::sys::unix::thread::Thread::new::thread_start::h6913f7fb82d0a425
                               at /rustc/4b8f4319954ff2642690b9e5cbe4af352d095bf6/library/std/src/sys/unix/thread.rs:108:17
  69:     0x7fe954c04609 - start_thread
  70:     0x7fe954b27133 - clone
  71:                0x0 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (4b8f43199 2022-10-19) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type bin -C embed-bitcode=no -C codegen-units=1 -C debuginfo=2

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [type_op_normalize_fn_sig] normalizing `([hyper::server::server::Server<hyper::server::tcp::AddrIncoming, hyper::service::make::MakeServiceFn<[closure@src/main.rs:14:51: 14:54]>>]; c_variadic: false)-><hyper::server::server::Server<hyper::server::tcp::AddrIncoming, hyper::service::make::MakeServiceFn<[closure@src/main.rs:14:51: 14:54]>> as core::future::into_future::IntoFuture>::IntoFuture`
#1 [mir_borrowck] borrow-checking `iceice::{closure#0}`
#2 [mir_borrowck] borrow-checking `iceice`
#3 [type_of] computing type of `iceice::{opaque#0}`
#4 [check_mod_item_types] checking item types in top-level module
#5 [analysis] running analysis passes on this crate
end of query stack
For more information about this error, try `rustc --explain E0425`.
