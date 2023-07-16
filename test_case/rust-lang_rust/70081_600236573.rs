plain
2020-03-17T18:06:57.9984622Z ========================== Starting Command Output ===========================
2020-03-17T18:06:57.9987319Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/e665069c-3648-4b2f-b377-f5e97300721c.sh
2020-03-17T18:06:57.9987622Z 
2020-03-17T18:06:57.9992008Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-17T18:06:58.0013707Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70081/merge to s
2020-03-17T18:06:58.0016994Z Task         : Get sources
2020-03-17T18:06:58.0017427Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-17T18:06:58.0017745Z Version      : 1.0.0
2020-03-17T18:06:58.0017919Z Author       : Microsoft
---
2020-03-17T18:06:58.9910273Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-17T18:06:58.9915421Z ##[command]git config gc.auto 0
2020-03-17T18:06:58.9918983Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-17T18:06:58.9922261Z ##[command]git config --get-all http.proxy
2020-03-17T18:06:58.9928151Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70081/merge:refs/remotes/pull/70081/merge
---
2020-03-17T18:43:10.0005753Z    Compiling rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-17T18:43:14.2278155Z error: unnecessary parentheses around block return value
2020-03-17T18:43:14.2283771Z   --> src/librustc/query/mod.rs:91:30
2020-03-17T18:43:14.2284393Z    |
2020-03-17T18:43:14.2285141Z 91 |             cache_on_disk_if { key.is_local() }
2020-03-17T18:43:14.2292257Z    |
2020-03-17T18:43:14.2293360Z    = note: `-D unused-parens` implied by `-D warnings`
2020-03-17T18:43:14.2293646Z 
2020-03-17T18:43:14.2294660Z error: unnecessary parentheses around block return value
2020-03-17T18:43:14.2294660Z error: unnecessary parentheses around block return value
2020-03-17T18:43:14.2295320Z    --> src/librustc/query/mod.rs:102:30
2020-03-17T18:43:14.2295776Z     |
2020-03-17T18:43:14.2296371Z 102 |             cache_on_disk_if { key.is_local() }
2020-03-17T18:43:14.2297981Z 
2020-03-17T18:43:14.2298444Z error: unnecessary parentheses around block return value
2020-03-17T18:43:14.2299173Z    --> src/librustc/query/mod.rs:126:30
2020-03-17T18:43:14.2299818Z     |
2020-03-17T18:43:14.2299818Z     |
2020-03-17T18:43:14.2300957Z 126 |             cache_on_disk_if { key.is_local() }
2020-03-17T18:43:14.2302447Z 
2020-03-17T18:43:14.2309065Z error: unnecessary parentheses around block return value
2020-03-17T18:43:14.2315874Z    --> src/librustc/query/mod.rs:163:30
2020-03-17T18:43:14.2316372Z     |
2020-03-17T18:43:14.2316372Z     |
2020-03-17T18:43:14.2316985Z 163 |             cache_on_disk_if { key.is_local() }
2020-03-17T18:43:14.2318536Z 
2020-03-17T18:43:14.2319065Z error: unnecessary parentheses around block return value
2020-03-17T18:43:14.2319655Z    --> src/librustc/query/mod.rs:191:30
2020-03-17T18:43:14.2320486Z     |
2020-03-17T18:43:14.2320486Z     |
2020-03-17T18:43:14.2321455Z 191 |             cache_on_disk_if { key.is_local() }
2020-03-17T18:43:14.2322991Z 
2020-03-17T18:43:14.2323463Z error: unnecessary parentheses around block return value
2020-03-17T18:43:14.2324221Z    --> src/librustc/query/mod.rs:204:30
2020-03-17T18:43:14.2324884Z     |
2020-03-17T18:43:14.2324884Z     |
2020-03-17T18:43:14.2325422Z 204 |             cache_on_disk_if { key.is_local() }
2020-03-17T18:43:14.2326765Z 
2020-03-17T18:43:14.2327206Z error: unnecessary parentheses around block return value
2020-03-17T18:43:14.2327953Z    --> src/librustc/query/mod.rs:399:30
2020-03-17T18:43:14.2328400Z     |
2020-03-17T18:43:14.2328400Z     |
2020-03-17T18:43:14.2328957Z 399 |             cache_on_disk_if { key.is_local() }
2020-03-17T18:43:14.2331181Z 
2020-03-17T18:43:14.2331671Z error: unnecessary parentheses around block return value
2020-03-17T18:43:14.2332381Z    --> src/librustc/query/mod.rs:469:30
2020-03-17T18:43:14.2333022Z     |
2020-03-17T18:43:14.2333022Z     |
2020-03-17T18:43:14.2334169Z 469 |             cache_on_disk_if { key.is_local() }
2020-03-17T18:43:14.2336378Z 
2020-03-17T18:43:14.2336908Z error: unnecessary parentheses around block return value
2020-03-17T18:43:14.2337880Z    --> src/librustc/query/mod.rs:472:30
2020-03-17T18:43:14.2338485Z     |
2020-03-17T18:43:14.2338485Z     |
2020-03-17T18:43:14.2339056Z 472 |             cache_on_disk_if { key.is_local() }
2020-03-17T18:43:14.2340750Z 
2020-03-17T18:43:14.2341253Z error: unnecessary parentheses around block return value
2020-03-17T18:43:14.2346804Z    --> src/librustc/query/mod.rs:485:30
2020-03-17T18:43:14.2347585Z     |
2020-03-17T18:43:14.2347585Z     |
2020-03-17T18:43:14.2348353Z 485 |             cache_on_disk_if { key.is_local() }
2020-03-17T18:43:14.2349711Z 
2020-03-17T18:43:14.2403702Z error: unnecessary parentheses around block return value
2020-03-17T18:43:14.2404330Z    --> src/librustc/query/mod.rs:593:30
2020-03-17T18:43:14.2404800Z     |
2020-03-17T18:43:14.2404800Z     |
2020-03-17T18:43:14.2405361Z 593 |             cache_on_disk_if { key.is_local() }
2020-03-17T18:43:14.2406671Z 
2020-03-17T18:43:14.2407125Z error: unnecessary parentheses around block return value
2020-03-17T18:43:14.2407705Z    --> src/librustc/query/mod.rs:625:30
2020-03-17T18:43:14.2408796Z     |
2020-03-17T18:43:14.2408796Z     |
2020-03-17T18:43:14.2409430Z 625 |             cache_on_disk_if { true }
2020-03-17T18:43:14.2410694Z 
2020-03-17T18:43:14.2411153Z error: unnecessary parentheses around block return value
2020-03-17T18:43:14.2411723Z    --> src/librustc/query/mod.rs:645:30
2020-03-17T18:43:14.2412173Z     |
2020-03-17T18:43:14.2412173Z     |
2020-03-17T18:43:14.2412704Z 645 |             cache_on_disk_if { true }
2020-03-17T18:43:14.2414105Z 
2020-03-17T18:43:14.2414598Z error: unnecessary parentheses around block return value
2020-03-17T18:43:14.2415174Z    --> src/librustc/query/mod.rs:678:30
2020-03-17T18:43:14.2415622Z     |
2020-03-17T18:43:14.2415622Z     |
2020-03-17T18:43:14.2416234Z 678 |             cache_on_disk_if { true }
2020-03-17T18:43:14.2417673Z 
2020-03-17T18:43:14.2418116Z error: unnecessary parentheses around block return value
2020-03-17T18:43:14.2418667Z    --> src/librustc/query/mod.rs:695:30
2020-03-17T18:43:14.2419111Z     |
2020-03-17T18:43:14.2419111Z     |
2020-03-17T18:43:14.2419626Z 695 |             cache_on_disk_if { true }
2020-03-17T18:43:14.2420827Z 
2020-03-17T18:43:14.2421272Z error: unnecessary parentheses around block return value
2020-03-17T18:43:14.2421828Z    --> src/librustc/query/mod.rs:731:30
2020-03-17T18:43:14.2422264Z     |
2020-03-17T18:43:14.2422264Z     |
2020-03-17T18:43:14.2422782Z 731 |             cache_on_disk_if { true }
2020-03-17T18:43:14.2423972Z 
2020-03-17T18:43:15.8920457Z    Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-17T18:43:29.2253594Z error: aborting due to 16 previous errors
2020-03-17T18:43:29.2254669Z 
---
2020-03-17T18:43:49.1111989Z   local time: Tue Mar 17 18:43:49 UTC 2020
2020-03-17T18:43:49.4084228Z   network time: Tue, 17 Mar 2020 18:43:49 GMT
2020-03-17T18:43:49.4090120Z == end clock drift check ==
2020-03-17T18:43:49.9119142Z 
2020-03-17T18:43:49.9202582Z ##[error]Bash exited with code '1'.
2020-03-17T18:43:49.9216642Z ##[section]Finishing: Run build
2020-03-17T18:43:49.9269224Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70081/merge to s
2020-03-17T18:43:49.9274583Z Task         : Get sources
2020-03-17T18:43:49.9274939Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-17T18:43:49.9275417Z Version      : 1.0.0
2020-03-17T18:43:49.9275639Z Author       : Microsoft
2020-03-17T18:43:49.9275639Z Author       : Microsoft
2020-03-17T18:43:49.9276007Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-17T18:43:49.9276413Z ==============================================================================
2020-03-17T18:43:50.3085083Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-17T18:43:50.3145027Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70081/merge to s
2020-03-17T18:43:50.3234275Z Cleaning up task key
2020-03-17T18:43:50.3235469Z Start cleaning up orphan processes.
2020-03-17T18:43:50.3445934Z Terminate orphan process: pid (4268) (python)
2020-03-17T18:43:50.3640737Z ##[section]Finishing: Finalize Job
