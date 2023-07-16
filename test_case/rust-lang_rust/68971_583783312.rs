plain
2020-02-08T22:19:42.4124889Z ========================== Starting Command Output ===========================
2020-02-08T22:19:42.4127041Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/ad04143e-ea5b-4913-8b93-a2ec59f9464c.sh
2020-02-08T22:19:42.4127098Z 
2020-02-08T22:19:42.4129864Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-08T22:19:42.4135678Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68971/merge to s
2020-02-08T22:19:42.4137861Z Task         : Get sources
2020-02-08T22:19:42.4137898Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-08T22:19:42.4137931Z Version      : 1.0.0
2020-02-08T22:19:42.4138000Z Author       : Microsoft
---
2020-02-08T22:19:43.2674669Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-08T22:19:43.2769576Z ##[command]git config gc.auto 0
2020-02-08T22:19:43.2854604Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-08T22:19:43.2907186Z ##[command]git config --get-all http.proxy
2020-02-08T22:19:44.0314879Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68971/merge:refs/remotes/pull/68971/merge
---
2020-02-08T22:26:25.0069757Z 1   |     ($ item : item) => { }
2020-02-08T22:26:25.0070486Z     |     ---------------------- in this expansion of `#[derive(RustcEncodable)]` (#3)
2020-02-08T22:26:25.0070896Z    --> <::rustc_index::vec::newtype_index macros>:126:14
2020-02-08T22:26:25.0071257Z     |
2020-02-08T22:26:25.0071974Z 1   |   / ($ (# [$ attrs : meta]) * $ v : vis struct $ name : ident { .. }) =>
2020-02-08T22:26:25.0072480Z 2   |   | ($ crate :: newtype_index !
2020-02-08T22:26:25.0072991Z 3   |   |  (@ attrs [$ (# [$ attrs]) *] @ type [$ name] @ max [0xFFFF_FF00] @ vis [$ v]
2020-02-08T22:26:25.0073435Z 4   |   |   @ debug_format ["{}"]) ;) ;
2020-02-08T22:26:25.0074739Z 7   |   | ($ crate :: newtype_index !
2020-02-08T22:26:25.0075171Z     |  _|__-
2020-02-08T22:26:25.0075171Z     |  _|__-
2020-02-08T22:26:25.0075689Z 8   | | |  (@ attrs [$ (# [$ attrs]) *] @ type [$ name] @ max [0xFFFF_FF00] @ vis [$ v]
2020-02-08T22:26:25.0076355Z 9   | | |   @ debug_format ["{}"] $ ($ tokens) +) ;) ;
2020-02-08T22:26:25.0077380Z ...     |
2020-02-08T22:26:25.0077380Z ...     |
2020-02-08T22:26:25.0078021Z 126 |   |  (@ derives [RustcEncodable,] @ attrs [$ (# [$ attrs]) *] @ type [$ type] @
2020-02-08T22:26:25.0079154Z ...     |
2020-02-08T22:26:25.0079154Z ...     |
2020-02-08T22:26:25.0079635Z 169 |   |  (@ derives [$ ($ derives,) *] @ attrs [$ (# [$ attrs]) *] @ type [$ type] @
2020-02-08T22:26:25.0080151Z 170 |   |   max [$ max] @ vis [$ v] @ debug_format [$ debug_format] $ ($ tokens) *) ;) ;
2020-02-08T22:26:25.0081164Z     |   |                                                                              |
2020-02-08T22:26:25.0081164Z     |   |                                                                              |
2020-02-08T22:26:25.0081851Z     |   |______________________________________________________________________________in this expansion of `rustc_index::newtype_index!` (#1)
2020-02-08T22:26:25.0082521Z     |                                                                                  in this expansion of `$crate::newtype_index!` (#2)
2020-02-08T22:26:25.0083265Z    ::: src/librustc_span/def_id.rs:105:1
2020-02-08T22:26:25.0083601Z     |
2020-02-08T22:26:25.0083601Z     |
2020-02-08T22:26:25.0083917Z 105 | /   rustc_index::newtype_index! {
2020-02-08T22:26:25.0084342Z 106 | |       /// A DefIndex is an index into the hir-map for a crate, identifying a
2020-02-08T22:26:25.0085053Z 107 | |       /// particular definition. It should really be considered an interned
2020-02-08T22:26:25.0085524Z 108 | |       /// shorthand for a particular DefPath.
2020-02-08T22:26:25.0086453Z 115 | |       }
2020-02-08T22:26:25.0086862Z 116 | |   }
2020-02-08T22:26:25.0087456Z     | |___- in this macro invocation (#1)
2020-02-08T22:26:25.0087827Z     |
2020-02-08T22:26:25.0087827Z     |
2020-02-08T22:26:25.0088221Z     = note: conflicting implementation in crate `serialize`:
2020-02-08T22:26:25.0088602Z             - impl<T> serialize::serialize::Encodable for T
2020-02-08T22:26:25.0088982Z               where T: serialize::serialize::UseSpecializedEncodable, T: ?Sized;
2020-02-08T22:26:25.0258673Z error[E0119]: conflicting implementations of trait `serialize::serialize::Decodable` for type `def_id::DefIndex`:
2020-02-08T22:26:25.0259132Z    --> <::rustc_index::vec::newtype_index macros>:130:2
2020-02-08T22:26:25.0259350Z     |
2020-02-08T22:26:25.0259350Z     |
2020-02-08T22:26:25.0259850Z 1   |    / ($ (# [$ attrs : meta]) * $ v : vis struct $ name : ident { .. }) =>
2020-02-08T22:26:25.0260150Z 2   |    | ($ crate :: newtype_index !
2020-02-08T22:26:25.0260959Z 3   |    |  (@ attrs [$ (# [$ attrs]) *] @ type [$ name] @ max [0xFFFF_FF00] @ vis [$ v]
2020-02-08T22:26:25.0261279Z 4   |    |   @ debug_format ["{}"]) ;) ;
2020-02-08T22:26:25.0262013Z 7   |    | ($ crate :: newtype_index !
2020-02-08T22:26:25.0262256Z     |  __|__-
2020-02-08T22:26:25.0262256Z     |  __|__-
2020-02-08T22:26:25.0262851Z 8   | |  |  (@ attrs [$ (# [$ attrs]) *] @ type [$ name] @ max [0xFFFF_FF00] @ vis [$ v]
2020-02-08T22:26:25.0263207Z 9   | |  |   @ debug_format ["{}"] $ ($ tokens) +) ;) ;
2020-02-08T22:26:25.0263716Z     | |__|_________________________________________- in this macro invocation (#2)
2020-02-08T22:26:25.0263971Z ...      |
2020-02-08T22:26:25.0264508Z 127 |    |   max [$ max] @ vis [$ v] @ debug_format [$ debug_format] $ ($ tokens) *) ; $
2020-02-08T22:26:25.0264988Z     |  __|_____________________________________________________________________________-
2020-02-08T22:26:25.0265978Z 128 | |  |  crate :: newtype_index ! (@ decodable $ type) ;) ;
2020-02-08T22:26:25.0266320Z     | |__|________________________________________________- in this macro invocation (#3)
2020-02-08T22:26:25.0267047Z 129 |    | (@ decodable $ type : ident) =>
2020-02-08T22:26:25.0267694Z 130 |    | (impl :: rustc_serialize :: Decodable for $ type
2020-02-08T22:26:25.0268024Z     |    |  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2020-02-08T22:26:25.0268480Z ...      |
2020-02-08T22:26:25.0268993Z 169 |    |  (@ derives [$ ($ derives,) *] @ attrs [$ (# [$ attrs]) *] @ type [$ type] @
2020-02-08T22:26:25.0269705Z 170 |    |   max [$ max] @ vis [$ v] @ debug_format [$ debug_format] $ ($ tokens) *) ;) ;
2020-02-08T22:26:25.0271219Z     |    |                                                                              |
2020-02-08T22:26:25.0271219Z     |    |                                                                              |
2020-02-08T22:26:25.0271849Z     |    |                                                                              in this expansion of `rustc_index::newtype_index!` (#1)
2020-02-08T22:26:25.0272275Z     |    |______________________________________________________________________________in this expansion of `$crate::newtype_index!` (#2)
2020-02-08T22:26:25.0272690Z     |                                                                                   in this expansion of `$crate::newtype_index!` (#3)
2020-02-08T22:26:25.0273190Z    ::: src/librustc_span/def_id.rs:105:1
2020-02-08T22:26:25.0273426Z     |
2020-02-08T22:26:25.0273740Z 105 | /    rustc_index::newtype_index! {
2020-02-08T22:26:25.0273740Z 105 | /    rustc_index::newtype_index! {
2020-02-08T22:26:25.0274110Z 106 | |        /// A DefIndex is an index into the hir-map for a crate, identifying a
2020-02-08T22:26:25.0274499Z 107 | |        /// particular definition. It should really be considered an interned
2020-02-08T22:26:25.0275348Z 108 | |        /// shorthand for a particular DefPath.
2020-02-08T22:26:25.0275972Z 115 | |        }
2020-02-08T22:26:25.0276201Z 116 | |    }
2020-02-08T22:26:25.0276569Z     | |____- in this macro invocation (#1)
2020-02-08T22:26:25.0276781Z     |
2020-02-08T22:26:25.0276781Z     |
2020-02-08T22:26:25.0277012Z     = note: conflicting implementation in crate `serialize`:
2020-02-08T22:26:25.0277228Z             - impl<T> serialize::serialize::Decodable for T
2020-02-08T22:26:25.0277431Z               where T: serialize::serialize::UseSpecializedDecodable;
2020-02-08T22:26:25.0295283Z error: aborting due to 2 previous errors
2020-02-08T22:26:25.0298579Z 
2020-02-08T22:26:25.0305002Z For more information about this error, try `rustc --explain E0119`.
2020-02-08T22:26:25.0451910Z error: could not compile `rustc_span`.
2020-02-08T22:26:25.0451910Z error: could not compile `rustc_span`.
2020-02-08T22:26:25.0451977Z 
2020-02-08T22:26:25.0452210Z To learn more, run the command again with --verbose.
2020-02-08T22:26:25.0483462Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-02-08T22:26:25.0494219Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-02-08T22:26:25.0494297Z Build completed unsuccessfully in 0:04:23
2020-02-08T22:26:25.0550233Z == clock drift check ==
2020-02-08T22:26:25.0562723Z   local time: Sat Feb  8 22:26:25 UTC 2020
2020-02-08T22:26:25.0562723Z   local time: Sat Feb  8 22:26:25 UTC 2020
2020-02-08T22:26:25.3402940Z   network time: Sat, 08 Feb 2020 22:26:25 GMT
2020-02-08T22:26:25.3407960Z == end clock drift check ==
2020-02-08T22:26:26.0140636Z 
2020-02-08T22:26:26.0242750Z ##[error]Bash exited with code '1'.
2020-02-08T22:26:26.0255612Z ##[section]Finishing: Run build
2020-02-08T22:26:26.0270824Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68971/merge to s
2020-02-08T22:26:26.0273102Z Task         : Get sources
2020-02-08T22:26:26.0273143Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-08T22:26:26.0273199Z Version      : 1.0.0
2020-02-08T22:26:26.0273234Z Author       : Microsoft
2020-02-08T22:26:26.0273234Z Author       : Microsoft
2020-02-08T22:26:26.0273274Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-08T22:26:26.0273333Z ==============================================================================
2020-02-08T22:26:26.3747511Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-08T22:26:26.3784889Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68971/merge to s
2020-02-08T22:26:26.3882436Z Cleaning up task key
2020-02-08T22:26:26.3883117Z Start cleaning up orphan processes.
2020-02-08T22:26:26.3976074Z Terminate orphan process: pid (4635) (python)
2020-02-08T22:26:26.4146985Z ##[section]Finishing: Finalize Job
