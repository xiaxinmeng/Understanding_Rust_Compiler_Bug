plain
2019-08-03T12:00:03.6012108Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-03T12:00:03.6214963Z ##[command]git config gc.auto 0
2019-08-03T12:00:03.6283877Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-03T12:00:03.6352742Z ##[command]git config --get-all http.proxy
2019-08-03T12:00:03.6486654Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63236/merge:refs/remotes/pull/63236/merge
---
2019-08-03T12:00:43.5201273Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-03T12:00:43.5201312Z 
2019-08-03T12:00:43.5201561Z   git checkout -b <new-branch-name>
2019-08-03T12:00:43.5201615Z 
2019-08-03T12:00:43.5201673Z HEAD is now at 23c235e38 Merge 95f9be04b530c0ea162cea7fb2b05a2d546e4a3c into d7270712cb446aad0817040bbca73a8d024f67b0
2019-08-03T12:00:43.5370163Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-03T12:00:43.5373546Z ==============================================================================
2019-08-03T12:00:43.5373617Z Task         : Bash
2019-08-03T12:00:43.5373674Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-03T12:05:39.7285431Z    |
2019-08-03T12:05:39.7285905Z 76 | #![feature(const_generics)]
2019-08-03T12:05:39.7286351Z    |            ^^^^^^^^^^^^^^
2019-08-03T12:05:39.7286564Z 
2019-08-03T12:05:41.6161491Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6162153Z     |
2019-08-03T12:05:41.6162153Z     |
2019-08-03T12:05:41.6162418Z 305 | #[doc(alias = "?")]
2019-08-03T12:05:41.6163019Z     |
2019-08-03T12:05:41.6163019Z     |
2019-08-03T12:05:41.6163499Z     = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6163841Z     = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6163886Z 
2019-08-03T12:05:41.6521678Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6522078Z   --> src/libcore/ops/arith.rs:80:1
2019-08-03T12:05:41.6522304Z    |
2019-08-03T12:05:41.6522669Z 80 | #[doc(alias = "+")]
2019-08-03T12:05:41.6523266Z    |
2019-08-03T12:05:41.6523266Z    |
2019-08-03T12:05:41.6523701Z    = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6524057Z    = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6524098Z 
2019-08-03T12:05:41.6524352Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6524715Z    --> src/libcore/ops/arith.rs:178:1
2019-08-03T12:05:41.6524921Z     |
2019-08-03T12:05:41.6525171Z 178 | #[doc(alias = "-")]
2019-08-03T12:05:41.6525643Z     |
2019-08-03T12:05:41.6525643Z     |
2019-08-03T12:05:41.6538544Z     = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6539653Z     = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6539906Z 
2019-08-03T12:05:41.6540270Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6540535Z    --> src/libcore/ops/arith.rs:298:1
2019-08-03T12:05:41.6540753Z     |
2019-08-03T12:05:41.6597977Z 298 | #[doc(alias = "*")]
2019-08-03T12:05:41.6598730Z     |
2019-08-03T12:05:41.6598730Z     |
2019-08-03T12:05:41.6599597Z     = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6599958Z     = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6600002Z 
2019-08-03T12:05:41.6600423Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6600893Z    --> src/libcore/ops/arith.rs:422:1
2019-08-03T12:05:41.6601172Z     |
2019-08-03T12:05:41.6601430Z 422 | #[doc(alias = "/")]
2019-08-03T12:05:41.6601923Z     |
2019-08-03T12:05:41.6601923Z     |
2019-08-03T12:05:41.6602390Z     = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6602790Z     = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6602826Z 
2019-08-03T12:05:41.6603048Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6603286Z    --> src/libcore/ops/arith.rs:507:1
2019-08-03T12:05:41.6603493Z     |
2019-08-03T12:05:41.6603719Z 507 | #[doc(alias = "%")]
2019-08-03T12:05:41.6604141Z     |
2019-08-03T12:05:41.6604141Z     |
2019-08-03T12:05:41.6604430Z     = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6604717Z     = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6604753Z 
2019-08-03T12:05:41.6604973Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6605211Z    --> src/libcore/ops/arith.rs:607:1
2019-08-03T12:05:41.6605396Z     |
2019-08-03T12:05:41.6605621Z 607 | #[doc(alias = "-")]
2019-08-03T12:05:41.6606066Z     |
2019-08-03T12:05:41.6606066Z     |
2019-08-03T12:05:41.6606352Z     = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6606641Z     = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6606676Z 
2019-08-03T12:05:41.6606896Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6607132Z    --> src/libcore/ops/arith.rs:684:1
2019-08-03T12:05:41.6607316Z     |
2019-08-03T12:05:41.6607541Z 684 | #[doc(alias = "+")]
2019-08-03T12:05:41.6608110Z     |
2019-08-03T12:05:41.6608110Z     |
2019-08-03T12:05:41.6608424Z     = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6608936Z     = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6608976Z 
2019-08-03T12:05:41.6609716Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6610003Z    --> src/libcore/ops/arith.rs:685:1
2019-08-03T12:05:41.6610216Z     |
2019-08-03T12:05:41.6610473Z 685 | #[doc(alias = "+=")]
2019-08-03T12:05:41.6610962Z     |
2019-08-03T12:05:41.6610962Z     |
2019-08-03T12:05:41.6611464Z     = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6611847Z     = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6611889Z 
2019-08-03T12:05:41.6612164Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6612649Z    --> src/libcore/ops/arith.rs:740:1
2019-08-03T12:05:41.6612845Z     |
2019-08-03T12:05:41.6613193Z 740 | #[doc(alias = "-")]
2019-08-03T12:05:41.6613617Z     |
2019-08-03T12:05:41.6613617Z     |
2019-08-03T12:05:41.6613929Z     = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6614322Z     = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6614367Z 
2019-08-03T12:05:41.6614620Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6614857Z    --> src/libcore/ops/arith.rs:741:1
2019-08-03T12:05:41.6618703Z     |
2019-08-03T12:05:41.6619624Z 741 | #[doc(alias = "-=")]
2019-08-03T12:05:41.6620115Z     |
2019-08-03T12:05:41.6620115Z     |
2019-08-03T12:05:41.6620497Z     = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6620816Z     = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6620858Z 
2019-08-03T12:05:41.6621146Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6621412Z    --> src/libcore/ops/arith.rs:787:1
2019-08-03T12:05:41.6621629Z     |
2019-08-03T12:05:41.6621904Z 787 | #[doc(alias = "*")]
2019-08-03T12:05:41.6622601Z     |
2019-08-03T12:05:41.6622601Z     |
2019-08-03T12:05:41.6622906Z     = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6623174Z     = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6623209Z 
2019-08-03T12:05:41.6623447Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6623674Z    --> src/libcore/ops/arith.rs:788:1
2019-08-03T12:05:41.6624047Z     |
2019-08-03T12:05:41.6624296Z 788 | #[doc(alias = "*=")]
2019-08-03T12:05:41.6624700Z     |
2019-08-03T12:05:41.6624700Z     |
2019-08-03T12:05:41.6625016Z     = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6625288Z     = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6625323Z 
2019-08-03T12:05:41.6625560Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6626048Z    --> src/libcore/ops/arith.rs:834:1
2019-08-03T12:05:41.6626246Z     |
2019-08-03T12:05:41.6626765Z 834 | #[doc(alias = "/")]
2019-08-03T12:05:41.6627246Z     |
2019-08-03T12:05:41.6627246Z     |
2019-08-03T12:05:41.6627588Z     = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6627876Z     = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6627936Z 
2019-08-03T12:05:41.6628181Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6628416Z    --> src/libcore/ops/arith.rs:835:1
2019-08-03T12:05:41.6628730Z     |
2019-08-03T12:05:41.6629514Z 835 | #[doc(alias = "/=")]
2019-08-03T12:05:41.6630045Z     |
2019-08-03T12:05:41.6630045Z     |
2019-08-03T12:05:41.6630419Z     = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6630732Z     = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6630790Z 
2019-08-03T12:05:41.6631046Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6631299Z    --> src/libcore/ops/arith.rs:884:1
2019-08-03T12:05:41.6631531Z     |
2019-08-03T12:05:41.6631792Z 884 | #[doc(alias = "%")]
2019-08-03T12:05:41.6632270Z     |
2019-08-03T12:05:41.6632270Z     |
2019-08-03T12:05:41.6632816Z     = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6633086Z     = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6633140Z 
2019-08-03T12:05:41.6633364Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6633584Z    --> src/libcore/ops/arith.rs:885:1
2019-08-03T12:05:41.6633783Z     |
2019-08-03T12:05:41.6634010Z 885 | #[doc(alias = "%=")]
2019-08-03T12:05:41.6634430Z     |
2019-08-03T12:05:41.6634430Z     |
2019-08-03T12:05:41.6634718Z     = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6635159Z     = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6635219Z 
2019-08-03T12:05:41.6635448Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6635666Z    --> src/libcore/ops/bit.rs:113:1
2019-08-03T12:05:41.6635868Z     |
2019-08-03T12:05:41.6636091Z 113 | #[doc(alias = "&")]
2019-08-03T12:05:41.6636517Z     |
2019-08-03T12:05:41.6636517Z     |
2019-08-03T12:05:41.6636806Z     = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6637091Z     = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6637134Z 
2019-08-03T12:05:41.6637441Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6637694Z    --> src/libcore/ops/bit.rs:197:1
2019-08-03T12:05:41.6638021Z     |
2019-08-03T12:05:41.6638260Z 197 | #[doc(alias = "|")]
2019-08-03T12:05:41.6638831Z     |
2019-08-03T12:05:41.6638831Z     |
2019-08-03T12:05:41.6639612Z     = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6639957Z     = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6639998Z 
2019-08-03T12:05:41.6640251Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6640527Z    --> src/libcore/ops/bit.rs:284:1
2019-08-03T12:05:41.6640766Z     |
2019-08-03T12:05:41.6641024Z 284 | #[doc(alias = "^")]
2019-08-03T12:05:41.6641510Z     |
2019-08-03T12:05:41.6641510Z     |
2019-08-03T12:05:41.6641835Z     = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6642155Z     = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6642195Z 
2019-08-03T12:05:41.6642449Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6642817Z    --> src/libcore/ops/bit.rs:372:1
2019-08-03T12:05:41.6643026Z     |
2019-08-03T12:05:41.6643295Z 372 | #[doc(alias = "<<")]
2019-08-03T12:05:41.6643767Z     |
2019-08-03T12:05:41.6643767Z     |
2019-08-03T12:05:41.6644203Z     = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6644527Z     = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6644567Z 
2019-08-03T12:05:41.6657969Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6667691Z    --> src/libcore/ops/bit.rs:481:1
2019-08-03T12:05:41.6668225Z     |
2019-08-03T12:05:41.6668760Z 481 | #[doc(alias = ">>")]
2019-08-03T12:05:41.6687832Z     |
2019-08-03T12:05:41.6687832Z     |
2019-08-03T12:05:41.6688536Z     = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6689418Z     = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6689685Z 
2019-08-03T12:05:41.6690247Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6690697Z    --> src/libcore/ops/bit.rs:597:1
2019-08-03T12:05:41.6691086Z     |
2019-08-03T12:05:41.6691554Z 597 | #[doc(alias = "&=")]
2019-08-03T12:05:41.6692411Z     |
2019-08-03T12:05:41.6692411Z     |
2019-08-03T12:05:41.6693117Z     = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6693723Z     = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6693908Z 
2019-08-03T12:05:41.6694308Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6694748Z    --> src/libcore/ops/bit.rs:646:1
2019-08-03T12:05:41.6695132Z     |
2019-08-03T12:05:41.6695555Z 646 | #[doc(alias = "|=")]
2019-08-03T12:05:41.6696386Z     |
2019-08-03T12:05:41.6696386Z     |
2019-08-03T12:05:41.6697138Z     = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6697658Z     = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6697823Z 
2019-08-03T12:05:41.6698218Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6698728Z    --> src/libcore/ops/bit.rs:695:1
2019-08-03T12:05:41.6699633Z     |
2019-08-03T12:05:41.6700193Z 695 | #[doc(alias = "^=")]
2019-08-03T12:05:41.6701029Z     |
2019-08-03T12:05:41.6701029Z     |
2019-08-03T12:05:41.6701551Z     = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6702079Z     = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6702263Z 
2019-08-03T12:05:41.6712202Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6712677Z    --> src/libcore/ops/bit.rs:742:1
2019-08-03T12:05:41.6712956Z     |
2019-08-03T12:05:41.6713230Z 742 | #[doc(alias = "<<=")]
2019-08-03T12:05:41.6713728Z     |
2019-08-03T12:05:41.6713728Z     |
2019-08-03T12:05:41.6714082Z     = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6714414Z     = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6714461Z 
2019-08-03T12:05:41.6714734Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6715191Z    --> src/libcore/ops/bit.rs:810:1
2019-08-03T12:05:41.6715429Z     |
2019-08-03T12:05:41.6715684Z 810 | #[doc(alias = ">>=")]
2019-08-03T12:05:41.6716166Z     |
2019-08-03T12:05:41.6716166Z     |
2019-08-03T12:05:41.6716501Z     = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6716829Z     = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6716870Z 
2019-08-03T12:05:41.6743661Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6744034Z   --> src/libcore/ops/deref.rs:61:1
2019-08-03T12:05:41.6744446Z    |
2019-08-03T12:05:41.6744770Z 61 | #[doc(alias = "*")]
2019-08-03T12:05:41.6745322Z    |
2019-08-03T12:05:41.6745322Z    |
2019-08-03T12:05:41.6745840Z    = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6746322Z    = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6746365Z 
2019-08-03T12:05:41.6746609Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6746868Z   --> src/libcore/ops/deref.rs:62:1
2019-08-03T12:05:41.6747069Z    |
2019-08-03T12:05:41.6747324Z 62 | #[doc(alias = "&*")]
2019-08-03T12:05:41.6747852Z    |
2019-08-03T12:05:41.6747852Z    |
2019-08-03T12:05:41.6748197Z    = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6748491Z    = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6748529Z 
2019-08-03T12:05:41.6755713Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6756051Z    --> src/libcore/ops/deref.rs:158:1
2019-08-03T12:05:41.6756269Z     |
2019-08-03T12:05:41.6756524Z 158 | #[doc(alias = "*")]
2019-08-03T12:05:41.6757160Z     |
2019-08-03T12:05:41.6757160Z     |
2019-08-03T12:05:41.6757552Z     = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6757879Z     = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6757922Z 
2019-08-03T12:05:41.6838431Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6838902Z   --> src/libcore/ops/index.rs:58:1
2019-08-03T12:05:41.6855909Z    |
2019-08-03T12:05:41.6856347Z 58 | #[doc(alias = "]")]
2019-08-03T12:05:41.6856867Z    |
2019-08-03T12:05:41.6856867Z    |
2019-08-03T12:05:41.6857273Z    = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6857823Z    = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6857872Z 
2019-08-03T12:05:41.6858148Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6858523Z   --> src/libcore/ops/index.rs:59:1
2019-08-03T12:05:41.6858742Z    |
2019-08-03T12:05:41.6859366Z 59 | #[doc(alias = "[")]
2019-08-03T12:05:41.6859884Z    |
2019-08-03T12:05:41.6859884Z    |
2019-08-03T12:05:41.6860260Z    = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6860717Z    = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6860779Z 
2019-08-03T12:05:41.6861099Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6861356Z   --> src/libcore/ops/index.rs:60:1
2019-08-03T12:05:41.6861569Z    |
2019-08-03T12:05:41.6861846Z 60 | #[doc(alias = "[]")]
2019-08-03T12:05:41.6862320Z    |
2019-08-03T12:05:41.6862320Z    |
2019-08-03T12:05:41.6862683Z    = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6862998Z    = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6863039Z 
2019-08-03T12:05:41.6863335Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6863597Z    --> src/libcore/ops/index.rs:163:1
2019-08-03T12:05:41.6863923Z     |
2019-08-03T12:05:41.6864191Z 163 | #[doc(alias = "[")]
2019-08-03T12:05:41.6864650Z     |
2019-08-03T12:05:41.6864650Z     |
2019-08-03T12:05:41.6864989Z     = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6865417Z     = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6865457Z 
2019-08-03T12:05:41.6865903Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6866212Z    --> src/libcore/ops/index.rs:164:1
2019-08-03T12:05:41.6866442Z     |
2019-08-03T12:05:41.6866727Z 164 | #[doc(alias = "]")]
2019-08-03T12:05:41.6867199Z     |
2019-08-03T12:05:41.6867199Z     |
2019-08-03T12:05:41.6867555Z     = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6867869Z     = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6867911Z 
2019-08-03T12:05:41.6868185Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6868440Z    --> src/libcore/ops/index.rs:165:1
2019-08-03T12:05:41.6868654Z     |
2019-08-03T12:05:41.6870329Z 165 | #[doc(alias = "[]")]
2019-08-03T12:05:41.6870908Z     |
2019-08-03T12:05:41.6870908Z     |
2019-08-03T12:05:41.6871291Z     = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6871610Z     = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6871651Z 
2019-08-03T12:05:41.6871929Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6872183Z   --> src/libcore/ops/range.rs:41:1
2019-08-03T12:05:41.6872508Z    |
2019-08-03T12:05:41.6872781Z 41 | #[doc(alias = "..")]
2019-08-03T12:05:41.6873424Z    |
2019-08-03T12:05:41.6873424Z    |
2019-08-03T12:05:41.6873774Z    = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6874076Z    = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6874132Z 
2019-08-03T12:05:41.6874379Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6874622Z   --> src/libcore/ops/range.rs:73:1
2019-08-03T12:05:41.6874971Z    |
2019-08-03T12:05:41.6875238Z 73 | #[doc(alias = "..")]
2019-08-03T12:05:41.6875705Z    |
2019-08-03T12:05:41.6875705Z    |
2019-08-03T12:05:41.6876081Z    = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6876397Z    = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6876454Z 
2019-08-03T12:05:41.6876712Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6876965Z    --> src/libcore/ops/range.rs:178:1
2019-08-03T12:05:41.6877196Z     |
2019-08-03T12:05:41.6877460Z 178 | #[doc(alias = "..")]
2019-08-03T12:05:41.6877944Z     |
2019-08-03T12:05:41.6877944Z     |
2019-08-03T12:05:41.6878341Z     = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6878682Z     = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6878743Z 
2019-08-03T12:05:41.6879288Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6879577Z    --> src/libcore/ops/range.rs:262:1
2019-08-03T12:05:41.6879814Z     |
2019-08-03T12:05:41.6880079Z 262 | #[doc(alias = "..")]
2019-08-03T12:05:41.6880563Z     |
2019-08-03T12:05:41.6880563Z     |
2019-08-03T12:05:41.6880910Z     = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6881224Z     = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6881414Z 
2019-08-03T12:05:41.6881721Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6881983Z    --> src/libcore/ops/range.rs:332:1
2019-08-03T12:05:41.6882325Z     |
2019-08-03T12:05:41.6882582Z 332 | #[doc(alias = "..=")]
2019-08-03T12:05:41.6883056Z     |
2019-08-03T12:05:41.6883056Z     |
2019-08-03T12:05:41.6883382Z     = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6883702Z     = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6883741Z 
2019-08-03T12:05:41.6883988Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6884329Z    --> src/libcore/ops/range.rs:603:1
2019-08-03T12:05:41.6884598Z     |
2019-08-03T12:05:41.6884852Z 603 | #[doc(alias = "..=")]
2019-08-03T12:05:41.6885328Z     |
2019-08-03T12:05:41.6885328Z     |
2019-08-03T12:05:41.6885653Z     = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6885974Z     = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6886013Z 
2019-08-03T12:05:41.6886259Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6886501Z   --> src/libcore/ops/try.rs:21:1
2019-08-03T12:05:41.6886743Z    |
2019-08-03T12:05:41.6887001Z 21 | #[doc(alias = "?")]
2019-08-03T12:05:41.6887472Z    |
2019-08-03T12:05:41.6887472Z    |
2019-08-03T12:05:41.6887790Z    = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6888230Z    = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6888269Z 
2019-08-03T12:05:41.6888525Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6888791Z    --> src/libcore/cmp.rs:183:1
2019-08-03T12:05:41.6889298Z     |
2019-08-03T12:05:41.6889577Z 183 | #[doc(alias = "==")]
2019-08-03T12:05:41.6890104Z     |
2019-08-03T12:05:41.6890104Z     |
2019-08-03T12:05:41.6890451Z     = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6890786Z     = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6890828Z 
2019-08-03T12:05:41.6891084Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6891349Z    --> src/libcore/cmp.rs:184:1
2019-08-03T12:05:41.6891566Z     |
2019-08-03T12:05:41.6891826Z 184 | #[doc(alias = "!=")]
2019-08-03T12:05:41.6892315Z     |
2019-08-03T12:05:41.6892315Z     |
2019-08-03T12:05:41.6892827Z     = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6893171Z     = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6893212Z 
2019-08-03T12:05:41.6893568Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6893827Z    --> src/libcore/cmp.rs:242:1
2019-08-03T12:05:41.6894033Z     |
2019-08-03T12:05:41.6894283Z 242 | #[doc(alias = "==")]
2019-08-03T12:05:41.6894753Z     |
2019-08-03T12:05:41.6894753Z     |
2019-08-03T12:05:41.6895391Z     = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6895885Z     = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6895936Z 
2019-08-03T12:05:41.6896222Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6896493Z    --> src/libcore/cmp.rs:243:1
2019-08-03T12:05:41.6896708Z     |
2019-08-03T12:05:41.6896968Z 243 | #[doc(alias = "!=")]
2019-08-03T12:05:41.6897456Z     |
2019-08-03T12:05:41.6897456Z     |
2019-08-03T12:05:41.6897810Z     = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6898236Z     = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6898291Z 
2019-08-03T12:05:41.6898540Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6898921Z    --> src/libcore/cmp.rs:514:1
2019-08-03T12:05:41.6904039Z     |
2019-08-03T12:05:41.6904317Z 514 | #[doc(alias = "<")]
2019-08-03T12:05:41.6904816Z     |
2019-08-03T12:05:41.6904816Z     |
2019-08-03T12:05:41.6905464Z     = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6947578Z     = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6947653Z 
2019-08-03T12:05:41.6948014Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6948350Z    --> src/libcore/cmp.rs:515:1
2019-08-03T12:05:41.6948580Z     |
2019-08-03T12:05:41.6948849Z 515 | #[doc(alias = ">")]
2019-08-03T12:05:41.6949713Z     |
2019-08-03T12:05:41.6949713Z     |
2019-08-03T12:05:41.6950115Z     = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6950448Z     = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6950492Z 
2019-08-03T12:05:41.6950768Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6951023Z    --> src/libcore/cmp.rs:516:1
2019-08-03T12:05:41.6951506Z     |
2019-08-03T12:05:41.6951799Z 516 | #[doc(alias = "<=")]
2019-08-03T12:05:41.6952284Z     |
2019-08-03T12:05:41.6952284Z     |
2019-08-03T12:05:41.6952648Z     = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6952969Z     = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6953010Z 
2019-08-03T12:05:41.6953286Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6953541Z    --> src/libcore/cmp.rs:517:1
2019-08-03T12:05:41.6953761Z     |
2019-08-03T12:05:41.6954045Z 517 | #[doc(alias = ">=")]
2019-08-03T12:05:41.6954683Z     |
2019-08-03T12:05:41.6954683Z     |
2019-08-03T12:05:41.6955048Z     = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6955366Z     = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6955406Z 
2019-08-03T12:05:41.6955763Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6956019Z    --> src/libcore/cmp.rs:722:1
2019-08-03T12:05:41.6956238Z     |
2019-08-03T12:05:41.6956524Z 722 | #[doc(alias = ">")]
2019-08-03T12:05:41.6957021Z     |
2019-08-03T12:05:41.6957021Z     |
2019-08-03T12:05:41.6957385Z     = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6957699Z     = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6957756Z 
2019-08-03T12:05:41.6958017Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6958271Z    --> src/libcore/cmp.rs:723:1
2019-08-03T12:05:41.6958505Z     |
2019-08-03T12:05:41.6958771Z 723 | #[doc(alias = "<")]
2019-08-03T12:05:41.6959607Z     |
2019-08-03T12:05:41.6959607Z     |
2019-08-03T12:05:41.6959959Z     = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6960297Z     = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6960357Z 
2019-08-03T12:05:41.6960621Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6960874Z    --> src/libcore/cmp.rs:724:1
2019-08-03T12:05:41.6961109Z     |
2019-08-03T12:05:41.6961375Z 724 | #[doc(alias = "<=")]
2019-08-03T12:05:41.6961870Z     |
2019-08-03T12:05:41.6961870Z     |
2019-08-03T12:05:41.6962206Z     = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6962531Z     = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6962720Z 
2019-08-03T12:05:41.6963022Z error[E0658]: #[doc(alias = "...")] is experimental
2019-08-03T12:05:41.6963274Z    --> src/libcore/cmp.rs:725:1
2019-08-03T12:05:41.6963513Z     |
2019-08-03T12:05:41.6963778Z 725 | #[doc(alias = ">=")]
2019-08-03T12:05:41.6964272Z     |
2019-08-03T12:05:41.6964272Z     |
2019-08-03T12:05:41.6964615Z     = note: for more information, see ***/issues/50146
2019-08-03T12:05:41.6964944Z     = help: add #![feature(doc_alias)] to the crate attributes to enable
2019-08-03T12:05:41.6964985Z 
2019-08-03T12:05:41.7281044Z error[E0658]: #[doc(alias = "...")] is experimental
