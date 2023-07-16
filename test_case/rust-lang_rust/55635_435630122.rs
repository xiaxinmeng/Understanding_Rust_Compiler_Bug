plain
travis_time:end:05fad40f:start=1541284876858100045,finish=1541284930131123479,duration=53273023434
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:04:41]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:04:41]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:04:42]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:04:42]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:04:50] warning: unnecessary `unsafe` block
[00:04:50]     |
[00:04:50]     |
[00:04:50] 73  |                       $Ty(unsafe { NonZero(n) })
[00:04:50]     |                           ^^^^^^ unnecessary `unsafe` block
[00:04:50] ...
[00:04:50] 110 | / nonzero_integers! {
[00:04:50] 111 | |     NonZeroU8(u8);
[00:04:50] 112 | |     NonZeroU16(u16);
[00:04:50] 113 | |     NonZeroU32(u32);
[00:04:50] 116 | |     NonZeroUsize(usize);
[00:04:50] 117 | | }
[00:04:50]     | |_- in this macro invocation
[00:04:50]     |
[00:04:50]     |
[00:04:50]     = note: #[warn(unused_unsafe)] on by default
[00:04:50] 
[00:04:50] warning: unnecessary `unsafe` block
[00:04:50]     |
[00:04:50]     |
[00:04:50] 73  |                       $Ty(unsafe { NonZero(n) })
[00:04:50]     |                           ^^^^^^ unnecessary `unsafe` block
[00:04:50] ...
[00:04:50] 110 | / nonzero_integers! {
[00:04:50] 111 | |     NonZeroU8(u8);
[00:04:50] 112 | |     NonZeroU16(u16);
[00:04:50] 113 | |     NonZeroU32(u32);
[00:04:50] 116 | |     NonZeroUsize(usize);
[00:04:50] 117 | | }
[00:04:50]     | |_- in this macro invocation
[00:04:50] 
[00:04:50] 
[00:04:50] warning: unnecessary `unsafe` block
[00:04:50]     |
[00:04:50]     |
[00:04:50] 81  |                           Some($Ty(unsafe { NonZero(n) }))
[00:04:50]     |                                    ^^^^^^ unnecessary `unsafe` block
[00:04:50] ...
[00:04:50] 110 | / nonzero_integers! {
[00:04:50] 111 | |     NonZeroU8(u8);
[00:04:50] 112 | |     NonZeroU16(u16);
[00:04:50] 113 | |     NonZeroU32(u32);
[00:04:50] 116 | |     NonZeroUsize(usize);
[00:04:50] 117 | | }
[00:04:50]     | |_- in this macro invocation
[00:04:50] 
[00:04:50] 
[00:04:50] warning: unnecessary `unsafe` block
[00:04:50]     --> libcore/ptr.rs:2755:36
[00:04:50]      |
[00:04:50] 2755 |             Some(Unique { pointer: unsafe { NonZero(ptr as _) }, _marker: PhantomData })
[00:04:50]      |                                    ^^^^^^ unnecessary `unsafe` block
[00:04:50] 
[00:04:50] warning: unnecessary `unsafe` block
[00:04:50]     --> libcore/ptr.rs:2811:27
[00:04:50]      |
[00:04:50] 2811 |         Unique { pointer: unsafe { NonZero(reference as _) }, _marker: PhantomData }
[00:04:50]      |                           ^^^^^^ unnecessary `unsafe` block
[00:04:50] 
[00:04:50] warning: unnecessary `unsafe` block
[00:04:50]     --> libcore/ptr.rs:2818:27
[00:04:50]      |
[00:04:50] 2818 |         Unique { pointer: unsafe { NonZero(reference as _) }, _marker: PhantomData }
[00:04:50]      |                           ^^^^^^ unnecessary `unsafe` block
[00:04:50] 
[00:04:50] warning: unnecessary `unsafe` block
[00:04:50]     --> libcore/ptr.rs:2891:28
[00:04:50]      |
[00:04:50] 2891 |         NonNull { pointer: unsafe { NonZero(ptr as _) } }
[00:04:50]      |                            ^^^^^^ unnecessary `unsafe` block
[00:04:50] 
[00:04:50] warning: unnecessary `unsafe` block
[00:04:50]     --> libcore/ptr.rs:2899:37
[00:04:50]      |
[00:04:50] 2899 |             Some(NonNull { pointer: unsafe { NonZero(ptr as _) } })
[00:04:50]      |                                     ^^^^^^ unnecessary `unsafe` block
[00:04:50] 
[00:04:50] warning: unnecessary `unsafe` block
[00:04:50]     --> libcore/ptr.rs:3021:28
[00:04:50]      |
[00:04:50] 3021 |         NonNull { pointer: unsafe { NonZero(reference as _) } }
[00:04:50]      |                            ^^^^^^ unnecessary `unsafe` block
[00:04:50] 
[00:04:50] warning: unnecessary `unsafe` block
[00:04:50]     --> libcore/ptr.rs:3029:28
[00:04:50]      |
[00:04:50] 3029 |         NonNull { pointer: unsafe { NonZero(reference as _) } }
[00:04:50]      |                            ^^^^^^ unnecessary `unsafe` block
[00:04:50] 
[00:04:50] warning: unnecessary `unsafe` block
[00:04:50]    |
[00:04:50]    |
[00:04:50] 26 |         unsafe { NonZero(self.0) }
[00:04:50]    |         ^^^^^^ unnecessary `unsafe` block
[00:04:57]    Compiling libc v0.0.0 (/checkout/src/rustc/libc_shim)
[00:04:57]    Compiling alloc v0.0.0 (/checkout/src/liballoc)
[00:04:58]    Compiling alloc_system v0.0.0 (/checkout/src/liballoc_system)
[00:04:58]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
---
[00:50:03] .................................................................................................... 100/4992
[00:50:06] .................................................................................................... 200/4992
[00:50:08] ...........................................................................................ii....... 300/4992
[00:50:11] .........................................................................................iii........ 400/4992
[00:50:14] iiiiiiii.iii...........................iii...........................................i...........i.. 500/4992
[00:50:21] .................................................................................................... 700/4992
[00:50:27] .....................................................................i...........i.................. 800/4992
[00:50:30] ........................................................................................iiiii....... 900/4992
[00:50:33] .................................................................................................... 1000/4992
---
[00:51:08] .................................................................................................... 2200/4992
[00:51:12] .................................................................................................... 2300/4992
[00:51:16] .................................................................................................... 2400/4992
[00:51:20] .................................................................................................... 2500/4992
[00:51:23] .........................................................................iiiiiiiii.................. 2600/4992
[00:51:30] ........................ii.......................................................................... 2800/4992
[00:51:33] .................................................................................................... 2900/4992
[00:51:36] .................................................................................................... 3000/4992
[00:51:39] ...................i................................................................................ 3100/4992
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:02] 
[01:05:02] running 115 tests
[01:05:05] i..ii...iii..iii.....i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..ii 100/115
[01:05:06] .i....iiii.....
[01:05:06] 
[01:05:06]  finished in 3.417
[01:05:06] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:20] 
[01:05:20] running 118 tests
[01:05:44] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:05:48] ......iii.i.....ii
[01:05:48] 
[01:05:48]  finished in 28.294
[01:05:48] travis_fold:end:test_debuginfo

---
travis_time:start:test_run-pass-fulldeps
Check compiletest suite=run-pass-fulldeps mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:26] 
[01:06:26] running 97 tests
[01:08:27] ........................................F..............test [run-pass] run-pass-fulldeps/myriad-closures.rs has been running for over 60 seconds
e used\" ] [ ! ( value <= $ max ) as usize ] ;","highlight_start":1,"highlight_end":74},{"text":"unsafe { $ type { private : value } } } # [ inline ] $ v const unsafe fn","highlight_start":1,"highlight_end":73},{"text":"from_u32_unchecked ( value : u32 ) -> Self {","highlight_start":1,"highlight_end":45},{"text":"unsafe { $ type { private : value } } }","highlight_start":1,"highlight_end":40},{"text":"/// Extract value of this index as an integer.","highlight_start":1,"highlight_end":47},{"text":" # [ inline ] $ v fn index ( self ) -> usize { self . as_usize (  ) }","highlight_start":1,"highlight_end":70},{"text":"/// Extract value of this index as a usize.","highlight_start":1,"highlight_end":44},{"text":" # [ inline ] $ v fn as_u32 ( self ) -> u32 { self . private }","highlight_start":1,"highlight_end":63},{"text":"/// Extract value of this index as a u32.","highlight_start":1,"highlight_end":42},{"text":" # [ inline ] $ v fn as_usize ( self ) -> usize { self . as_u32 (  ) as usize","highlight_start":1,"highlight_end":78},{"text":"} } impl Idx for $ type {","highlight_start":1,"highlight_end":26},{"text":"# [ inline ] fn new ( value : usize ) -> Self { Self :: from ( value ) } # [","highlight_start":1,"highlight_end":77},{"text":"inline ] fn index ( self ) -> usize { usize :: from ( self ) } } impl :: std","highlight_start":1,"highlight_end":77},{"text":":: iter :: Step for $ type {","highlight_start":1,"highlight_end":29},{"text":"# [ inline ] fn steps_between ( start : & Self , end : & Self ) -> Option <","highlight_start":1,"highlight_end":76},{"text":"usize > {","highlight_start":1,"highlight_end":10},{"text":"< usize as :: std :: iter :: Step > :: steps_between (","highlight_start":1,"highlight_end":55},{"text":"& Idx :: index ( * start ) , & Idx :: index ( * end ) , ) } # [ inline ] fn","highlight_start":1,"highlight_end":76},{"text":"replace_one ( & mut self ) -> Self {","highlight_start":1,"highlight_end":37},{"text":":: std :: mem :: replace ( self , Self :: new ( 1 ) ) } # [ inline ] fn","highlight_start":1,"highlight_end":72},{"text":"replace_zero ( & mut self ) -> Self {","highlight_start":1,"highlight_end":38},{"text":":: std :: mem :: replace ( self , Self :: new ( 0 ) ) } # [ inline ] fn","highlight_start":1,"highlight_end":72},{"text":"add_one ( & self ) -> Self { Self :: new ( Idx :: index ( * self ) + 1 ) } # [","highlight_start":1,"highlight_end":79},{"text":"inline ] fn sub_one ( & self ) -> Self {","highlight_start":1,"highlight_end":41},{"text":"Self :: new ( Idx :: index ( * self ) - 1 ) } # [ inline ] fn add_usize (","highlight_start":1,"highlight_end":74},{"text":"& self , u : usize ) -> Option < Self > {","highlight_start":1,"highlight_end":42},{"text":"Idx :: index ( * self ) . checked_add ( u ) . map ( Self :: new ) } } impl","highlight_start":1,"highlight_end":75},{"text":"From < $ type > for u32 {","highlight_start":1,"highlight_end":26},{"text":"# [ inline ] fn from ( v : $ type ) -> u32 { v . as_u32 (  ) } } impl From < $","highlight_start":1,"highlight_end":79},{"text":"type > for usize {","highlight_start":1,"highlight_end":19},{"text":"# [ inline ] fn from ( v : $ type ) -> usize { v . as_usize (  ) } } impl From","highlight_start":1,"highlight_end":79},{"text":"< usize > for $ type {","highlight_start":1,"highlight_end":23},{"text":"# [ inline ] fn from ( value : usize ) -> Self {","highlight_start":1,"highlight_end":49},{"text":"$ type :: from_usize ( value ) } } impl From < u32 > for $ type {","highlight_start":1,"highlight_end":66},{"text":"# [ inline ] fn from ( value : u32 ) -> Self { $ type :: from_u32 ( value ) }","highlight_start":1,"highlight_end":78},{"text":"} newtype_index ! (","highlight_start":1,"highlight_end":20},{"text":"@ handle_debug @ derives [ $ ( $ derives , ) * ] @ type [ $ type ] @","highlight_start":1,"highlight_end":69},{"text":"debug_format [ $ debug_format ] ) ; ) ; (","highlight_start":1,"highlight_end":42},{"text":"@ handle_debug @ derives [ $ ( $ _derives : ident , ) * ] @ type [","highlight_start":1,"highlight_end":67},{"text":"$ type : ident ] @ debug_format [ custom ] ) => (  ) ; (","highlight_start":1,"highlight_end":57},{"text":"@ handle_debug @ derives [  ] @ type [ $ type : ident ] @ debug_format [","highlight_start":1,"highlight_end":73},{"text":"$ debug_format : tt ] ) => (","highlight_start":1,"highlight_end":29},{"text":"impl :: std :: fmt :: Debug for $ type {","highlight_start":1,"highlight_end":41},{"text":"fn fmt ( & self , fmt : & mut :: std :: fmt :: Formatter ) -> :: std :: fmt ::","highlight_start":1,"highlight_end":79},{"text":"Result { write ! ( fmt , $ debug_format , self . as_u32 (  ) ) } } ) ; (","highlight_start":1,"highlight_end":73},{"text":"@ handle_debug @ derives [ Debug , $ ( $ derives : ident , ) * ] @ type [","highlight_start":1,"highlight_end":74},{"text":"$ type : ident ] @ debug_format [ $ debug_format : tt ] ) => (  ) ; (","highlight_start":1,"highlight_end":70},{"text":"@ handle_debug @ derives [ $ _derive : ident , $ ( $ derives : ident , ) * ] @","highlight_start":1,"highlight_end":79},{"text":"type [ $ type : ident ] @ debug_format [ $ debug_format : tt ] ) => (","highlight_start":1,"highlight_end":70},{"text":"newtype_index ! (","highlight_start":1,"highlight_end":18},{"text":"@ handle_debug @ derives [ $ ( $ derives , ) * ] @ type [ $ type ] @","highlight_start":1,"highlight_end":69},{"text":"debug_format [ $ debug_format ] ) ; ) ; (","highlight_start":1,"highlight_end":42},{"text":"@ type [ $ type : ident ] @ max [ $ max : expr ] @ vis [ $ v : vis ] @","highlight_start":1,"highlight_end":71},{"text":"debug_format [ $ debug_format : tt ] derive [ $ ( $ derives : ident ) , * ] $","highlight_start":1,"highlight_end":78},{"text":"( $ tokens : tt ) * ) => (","highlight_start":1,"highlight_end":27},{"text":"newtype_index ! (","highlight_start":1,"highlight_end":18},{"text":"@ type [ $ type ] @ max [ $ max ] @ vis [ $ v ] @ debug_format [","highlight_start":1,"highlight_end":65},{"text":"$ debug_format ] derive [ $ ( $ derives , ) * ] $ ( $ tokens ) * ) ; ) ; (","highlight_start":1,"highlight_end":75},{"text":"@ type [ $ type : ident ] @ max [ $ max : expr ] @ vis [ $ v : vis ] @","highlight_start":1,"highlight_end":71},{"text":"debug_format [ $ debug_format : tt ] derive [ $ ( $ derives : ident , ) + ]","highlight_start":1,"highlight_end":76},{"text":"ENCODABLE = custom $ ( $ tokens : tt ) * ) => (","highlight_start":1,"highlight_end":48},{"text":"newtype_index ! (","highlight_start":1,"highlight_end":18},{"text":"@ derives [ $ ( $ derives , ) + ] @ type [ $ type ] @ max [ $ max ] @) ; ) ; (","highlight_start":1,"highlight_end":46},{"text":"@ type [ $ type : ident ] @ max [ $ max : expr ] @ vis [ $ v : vis ] @","highlight_start":1,"highlight_end":71},{"text":"debug_format [ $ debug_format : tt ] $ ( $ tokens : tt ) * ) => (","highlight_start":1,"highlight_end":66},{"text":"newtype_index ! (","highlight_start":1,"highlight_end":18},{"text":"@ derives [ RustcEncodable , ] @ type [ $ type ] @ max [ $ max ] @ vis [ $ v ]","highlight_start":1,"highlight_end":79},{"text":"@ debug_format [ $ debug_format ] $ ( $ tokens ) * ) ; impl Decodable for $","highlight_start":1,"highlight_end":76},{"text":"type {","highlight_start":1,"highlight_end":7},{"text":"fn decode < D : Decoder > ( d : & mut D ) -> Result < Self , D :: Error > {","highlight_start":1,"highlight_end":76},{"text":"d . read_u32 (  ) . map ( Self :: from ) } } ) ; (","highlight_start":1,"highlight_end":51},{"text":"@ derives [ $ ( $ derives : ident , ) * ] @ type [ $ type : ident ] @ max [","highlight_start":1,"highlight_end":76},{"text":"$ max : expr ] @ vis [ $ v : vis ] @ debug_format [ $ debug_format : tt ] $","highlight_start":1,"highlight_end":76},{"text":"name : ident = $ constant : expr ) => (","highlight_start":1,"highlight_end":40},{"text":"newtype_index ! (","highlight_start":1,"highlight_end":18},{"text":"@ derives [ $ ( $ derives , ) * ] @ type [ $ type ] @ max [ $ max ] @ vis [","highlight_start":1,"highlight_end":76},{"text":"$ v ] @ debug_format [ $ debug_format ] $ name = $ constant , ) ; ) ; (","highlight_start":1,"highlight_end":72},{"text":"@ derives [ $ ( $ derives : ident , ) * ] @ type [ $ type : ident ] @ max [","highlight_startt ) * ) => (","highlight_start":1,"highlight_end":66},{"text":"newtype_index ! (","highlight_start":1,"highlight_end":18},{"text":"@ derives [ $ ( $ derives , ) * ] @ type [ $ type ] @ max [ $ max ] @ vis [","highlight_start":1,"highlight_end":76},{"text":"$ v ] @ debug_format [ $ debug_format ] $ ( $ tokens ) * ) ; ) ; (","highlight_start":1,"highlight_end":67},{"text":"@ derives [ $ ( $ derives : ident , ) * ] @ type [ $ type : ident ] @ max [","highlight_start":1,"highlight_end":76},{"text":"$ max : expr ] @ vis [ $ v : vis ] @ debug_format [ $ debug_format : tt ] $ (","highlight_start":1,"highlight_end":78},{"text":"# [ doc = $ doc : expr ] ) * const $ name : ident = $ constant : expr , $ (","highlight_start":1,"highlight_end":76},{"text":"$ tokens : tt ) * ) => (","highlight_start":1,"highlight_end":25},{"text":"$ ( # [ doc = $ doc ] ) * pub const $ name : $ type = $ type :: from_u32_const","highlight_start":1,"highlight_end":79},{"text":"( $ constant ) ; newtype_index ! (","highlight_start":1,"highlight_end":35},{"text":"@ derives [ $ ( $ derives , ) * ] @ type [ $ type ] @ max [ $ max ] @ vis [","highlight_start":1,"highlight_end":76},{"text":"$ v ] @ debug_format [ $ debug_format ] $ ( $ tokens ) * ) ; ) ;","highlight_start":1,"highlight_end":65}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}}},"macro_decl_name":"newtype_index!","def_site_span":{"file_name":"<::rustc_data_structures::indexed_vec::newtype_index macros>","byte_start":0,"byte_end":8246,"line_start":1,"line_end":146,"column_start":1,"column_end":65,"is_primary":false,"text":[{"text":"( $ v : vis struct $ name@ max [","highlight_start":1,"highlight_end":77},{"text":"$ max ] @ vis [ $ v ] @ debug_format [ $ debug_format ] $ ( $ tokens ) * ) ;","highlight_start":1,"highlight_end":77},{"text":"impl Decodable for $ type {","highlight_start":1,"highlight_end":28},{"text":"fn decode < D : Decoder > ( d : & mut D ) -> Result < Self , D :: Error > {","highlight_start":1,"highlight_end":76},{"text":"d . read_u32 (  ) . into (  ) } } ) ; (","highlight_start":1,"highlight_end":40},{"text":"@ type [ $ type : ident ] @ max [ $ max : expr ] @ vis [ $ v : vis ] @","highlight_start":1,"highlight_end":71},{"text":"debug_format [ $ debug_format : tt ] ENCODABLE = custom $ ( $ tokens : tt ) *","highlight_start":1,"highlight_end":78},{"text":") => (","highlight_start":1,"highlight_end":7},{"text":"newtype_index ! (","highlight_start":1,"highlight_end":18},{"text":"@ derives [  ] @ type [ $ type ] @ max [ $ max ] @ vis [ $ v ] @ debug_format","highlight_start":1,"highlight_end":78},{"text":"[ $ debug_format ] $ ( $ tokens ) * ) ; ) ; (","highlight_start":1,"highlight_end":46},{"text":"@ type [ $ type : ident ] @ max [ $ max : expr ] @ vis [ $ v : vis ] @","highlight_start":1,"highlight_end":71},{"text":"debug_format [ $ debug_format : tt ] $ ( $ tokens : tt ) * ) => (","highlight_start":1,"highlight_end":66},{"text":"newtype_index ! (","highlight_start":1,"highlight_end":18},{"text":"@ derives [ RustcEncodable , ] @ type [ $ type ] @ max [ $ max ] @ vis [ $ v ]","highlight_start":1,"highlight_end":79},{"text":"@ debug_format [ $ debug_format ] $ ( $ tokens ) * ) ; impl Decodable for $","highlight_start":1,"highlight_end":76},{"text":"type {","highlight_start":1,"highlight_end":7},{"text":"fn decode < D : Decoder > ( d : & mut D ) -> Result < Self , D :: Error > {","highlight_start":1,"highlight_end":76},{"text":"d . read_u32 (  ) . map ( Self :: from ) } } ) ; (","highlight_start":1,"highlight_end":51},{"text":"@ derives [ $ ( $ derives : ident , ) * ] @ type [ $ type : ident ] @ max [","highlight_start":1,"highlight_end":76},{"text":"$ max : expr ] @ vis [ $ v : vis ] @ debug_format [ $ debug_format : tt ] $","highlight_start":1,"highlight_end":76},{"text":"name : ident = $ constant : expr ) => (","highlight_start":1,"highlight_end":40},{"text":"newtype_index ! (","highlight_start":1,"highlight_end":18},{"text":"@ derives [ $ ( $ derives , ) * ] @ type [ $ type ] @ max [ $ max ] @ vis [","highlight_start":1,"highlight_end":76},{"text":"$ v ] @ debug_format [ $ debug_format ] $ name = $ constant , ) ; ) ; (","highlight_start":1,"highlight_end":72},{"text":"@ derives [ $ ( $ derives : ident , ) * ] @ type [ $ type : ident ] @ max [","highlight_start":1,"highlight_end":76},{"text":"$ _max : expr ] @ vis [ $ v : vis ] @ debug_format [ $ debug_format : tt ] $ (","highlight_start":1,"highlight_end":79},{"text":"# [ doc = $ doc : expr ] ) * const $ name : ident = $ constant : expr ) => (","highlight_start":1,"highlight_end":77},{"text":"newtype_index ! (","highlight_start":1,"highlight_end":18},{"text":"@ derives [ $ ( $ derives , ) * ] @ type [ $ type ] @ max [ $ max ] @ vis [","highlight_start":1,"highlight_end":76},{"text":"$ v ] @ debug_format [ $ debug_format ] $ ( # [ doc = $ doc ] ) * const $ name","highlight_start":1,"highlight_end":79},{"text":"= $ constant , ) ; ) ; (tside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)\nhelp: possible candidate is found in another module, you can import it into scope\n   |\nLL | use rustc_serialize::Decodable;\n   |\n\n"}
[01:11:10] {"message":"cannot find trait `Decoder` in this scope","code":{"code":"E0405","explanation":"\nThe code refers to a trait that is not in scope.\n\nErroneous code example:\n\n