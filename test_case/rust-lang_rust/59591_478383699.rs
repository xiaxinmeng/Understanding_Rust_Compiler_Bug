plain
travis_time:end:26a91c47:start=1554062040685771283,finish=1554062042974614308,duration=2288843025
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:09:01]    |      ^^^^^^^^ cannot be resolved, ignoring
[01:09:01]    |
[01:09:01]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:01] 
[01:09:01] warning: `[std::needle::ext::rmatches]` cannot be resolved, ignoring it...
[01:09:01]    --> /checkout/src/libcore/needle/needle.rs:257:24
[01:09:01]     |
[01:09:01] 257 |     /// * [`rmatches`](std::needle::ext::rmatches)
[01:09:01]     |
[01:09:01]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:01] 
[01:09:01] 
[01:09:01] warning: `[std::needle::ext::rmatch_indices]` cannot be resolved, ignoring it...
[01:09:01]    --> /checkout/src/libcore/needle/needle.rs:258:30
[01:09:01]     |
[01:09:01] 258 |     /// * [`rmatch_indices`](std::needle::ext::rmatch_indices)
[01:09:01]     |
[01:09:01]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:01] 
[01:09:01] warning: `[std::needle::ext::find]` cannot be resolved, ignoring it...
[01:09:01] warning: `[std::needle::ext::find]` cannot be resolved, ignoring it...
[01:09:01]    --> /checkout/src/libcore/needle/needle.rs:259:21
[01:09:01]     |
[01:09:01] 259 |     /// * [`rfind`](std::needle::ext::find)
[01:09:01]     |
[01:09:01]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:01] 
[01:09:01] 
[01:09:01] warning: `[std::needle::ext::rmatch_ranges]` cannot be resolved, ignoring it...
[01:09:01]    --> /checkout/src/libcore/needle/needle.rs:260:29
[01:09:01]     |
[01:09:01] 260 |     /// * [`rmatch_ranges`](std::needle::ext::rmatch_ranges)
[01:09:01]     |
[01:09:01]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:01] 
[01:09:01] 
[01:09:01] warning: `[std::needle::ext::rfind_range]` cannot be resolved, ignoring it...
[01:09:01]    --> /checkout/src/libcore/needle/needle.rs:261:27
[01:09:01]     |
[01:09:01] 261 |     /// * [`rfind_range`](std::needle::ext::rfind_range)
[01:09:01]     |
[01:09:01]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:01] 
[01:09:01] 
[01:09:01] warning: `[std::needle::ext::rsplit]` cannot be resolved, ignoring it...
[01:09:01]    --> /checkout/src/libcore/needle/needle.rs:262:22
[01:09:01]     |
[01:09:01] 262 |     /// * [`rsplit`](std::needle::ext::rsplit)
[01:09:01]     |
[01:09:01]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:01] 
[01:09:01] warning: `[std::needle::ext::rsplit_terminator]` cannot be resolved, ignoring it...
[01:09:01] warning: `[std::needle::ext::rsplit_terminator]` cannot be resolved, ignoring it...
[01:09:01]    --> /checkout/src/libcore/needle/needle.rs:263:33
[01:09:01]     |
[01:09:01] 263 |     /// * [`rsplit_terminator`](std::needle::ext::rsplit_terminator)
[01:09:01]     |
[01:09:01]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:01] 
[01:09:01] 
[01:09:01] warning: `[std::needle::ext::rsplitn]` cannot be resolved, ignoring it...
[01:09:01]    --> /checkout/src/libcore/needle/needle.rs:264:23
[01:09:01]     |
[01:09:01] 264 |     /// * [`rsplitn`](std::needle::ext::rsplitn)
[01:09:01]     |
[01:09:01]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:01] 
[01:09:01] 
[01:09:01] warning: `[Span::into_parts]` cannot be resolved, ignoring it...
[01:09:01]    --> /checkout/src/libcore/needle/needle.rs:267:41
[01:09:01]     |
[01:09:01] 267 |     /// calling `span`[`.into_parts()`](Span::into_parts). The returned range
[01:09:01]     |
[01:09:01]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:01] 
[01:09:01] 
[01:09:01] warning: `[std::needle::ext::ends_with]` cannot be resolved, ignoring it...
[01:09:01]    --> /checkout/src/libcore/needle/needle.rs:317:25
[01:09:01]     |
[01:09:01] 317 |     /// [`ends_with()`](std::needle::ext::ends_with) as well as providing the default
[01:09:01]     |
[01:09:01]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:01] 
[01:09:01] 
[01:09:01] warning: `[ReverseConsumer::trim_end]` cannot be resolved, ignoring it...
[01:09:01]    --> /checkout/src/libcore/needle/needle.rs:318:44
[01:09:01]     |
[01:09:01] 318 |     /// implementation for [`.trim_end()`](ReverseConsumer::trim_end).
[01:09:01]     |
[01:09:01]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:01] 
[01:09:01] 
[01:09:01] warning: `[Span::into_parts]` cannot be resolved, ignoring it...
[01:09:01]    --> /checkout/src/libcore/needle/needle.rs:321:41
[01:09:01]     |
[01:09:01] 321 |     /// calling `span`[`.into_parts()`](Span::into_parts). If a needle can be
[01:09:01]     |
[01:09:01]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:01] 
[01:09:01] 
[01:09:01] warning: `[std::needle::ext::trim_end]` cannot be resolved, ignoring it...
[01:09:01]    --> /checkout/src/libcore/needle/needle.rs:357:24
[01:09:01]     |
[01:09:01] 357 |     /// [`trim_end()`](std::needle::ext::trim_end).
[01:09:01]     |
[01:09:01]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:01] 
[01:09:01] 
[01:09:01] warning: `[ReverseConsumer::rconsume]` cannot be resolved, ignoring it...
[01:09:01]    --> /checkout/src/libcore/needle/needle.rs:360:25
[01:09:01]     |
[01:09:01] 360 |     /// [`.rconsume()`](ReverseConsumer::rconsume) is provided by default.
[01:09:01]     |
[01:09:01]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:01] 
[01:09:01] 
[01:09:01] warning: `[SharedHaystack]` cannot be resolved, ignoring it...
[01:09:01]    --> /checkout/src/libcore/needle/haystack.rs:215:32
[01:09:01]     |
[01:09:01] 215 |     /// If this haystack is a [`SharedHaystack`], this method should never be
[01:09:01]     |
[01:09:01]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:01] 
[01:09:01] 
[01:09:01] warning: `[std::needle::ext::matches]` cannot be resolved, ignoring it...
[01:09:01]   --> /checkout/src/libcore/needle/needle.rs:93:23
[01:09:01]    |
[01:09:01] 93 |     /// * [`matches`](std::needle::ext::matches)
[01:09:01]    |
[01:09:01]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:01] 
[01:09:01] 
[01:09:01] warning: `[std::needle::ext::contains]` cannot be resolved, ignoring it...
[01:09:01]   --> /checkout/src/libcore/needle/needle.rs:94:24
[01:09:01]    |
[01:09:01] 94 |     /// * [`contains`](std::needle::ext::contains)
[01:09:01]    |
[01:09:01]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:01] 
[01:09:01] 
[01:09:01] warning: `[std::needle::ext::match_indices]` cannot be resolved, ignoring it...
[01:09:01]   --> /checkout/src/libcore/needle/needle.rs:95:29
[01:09:01]    |
[01:09:01] 95 |     /// * [`match_indices`](std::needle::ext::match_indices)
[01:09:01]    |
[01:09:01]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:01] 
[01:09:01] warning: `[std::needle::ext::find]` cannot be resolved, ignoring it...
[01:09:01] warning: `[std::needle::ext::find]` cannot be resolved, ignoring it...
[01:09:01]   --> /checkout/src/libcore/needle/needle.rs:96:20
[01:09:01]    |
[01:09:01] 96 |     /// * [`find`](std::needle::ext::find)
[01:09:01]    |
[01:09:01]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:01] 
[01:09:01] 
[01:09:01] warning: `[std::needle::ext::match_ranges]` cannot be resolved, ignoring it...
[01:09:01]   --> /checkout/src/libcore/needle/needle.rs:97:28
[01:09:01]    |
[01:09:01] 97 |     /// * [`match_ranges`](std::needle::ext::match_ranges)
[01:09:01]    |
[01:09:01]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:01] 
[01:09:01] warning: `[std::needle::ext::find_range]` cannot be resolved, ignoring it...
[01:09:01] warning: `[std::needle::ext::find_range]` cannot be resolved, ignoring it...
[01:09:01]   --> /checkout/src/libcore/needle/needle.rs:98:26
[01:09:01]    |
[01:09:01] 98 |     /// * [`find_range`](std::needle::ext::find_range)
[01:09:01]    |
[01:09:01]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:01] 
[01:09:01] warning: `[std::needle::ext::split]` cannot be resolved, ignoring it...
[01:09:01] warning: `[std::needle::ext::split]` cannot be resolved, ignoring it...
[01:09:01]   --> /checkout/src/libcore/needle/needle.rs:99:21
[01:09:01]    |
[01:09:01] 99 |     /// * [`split`](std::needle::ext::split)
[01:09:01]    |
[01:09:01]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:01] 
[01:09:01] warning: `[std::needle::ext::split_terminator]` cannot be resolved, ignoring it...
[01:09:01] warning: `[std::needle::ext::split_terminator]` cannot be resolved, ignoring it...
[01:09:01]    --> /checkout/src/libcore/needle/needle.rs:100:32
[01:09:01]     |
[01:09:01] 100 |     /// * [`split_terminator`](std::needle::ext::split_terminator)
[01:09:01]     |
[01:09:01]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:01] 
[01:09:01] 
[01:09:01] warning: `[std::needle::ext::splitn]` cannot be resolved, ignoring it...
[01:09:01]    --> /checkout/src/libcore/needle/needle.rs:101:22
[01:09:01]     |
[01:09:01] 101 |     /// * [`splitn`](std::needle::ext::splitn)
[01:09:01]     |
[01:09:01]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:01] 
[01:09:01] 
[01:09:01] warning: `[std::needle::ext::replace_with]` cannot be resolved, ignoring it...
[01:09:01]    --> /checkout/src/libcore/needle/needle.rs:102:28
[01:09:01]     |
[01:09:01] 102 |     /// * [`replace_with`](std::needle::ext::replace_with)
[01:09:01]     |
[01:09:01]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:01] 
[01:09:01] 
[01:09:01] warning: `[std::needle::ext::replacen_with]` cannot be resolved, ignoring it...
[01:09:01]    --> /checkout/src/libcore/needle/needle.rs:103:29
[01:09:01]     |
[01:09:01] 103 |     /// * [`replacen_with`](std::needle::ext::replacen_with)
[01:09:01]     |
[01:09:01]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:01] 
[01:09:01] 
[01:09:01] warning: `[Span::into_parts]` cannot be resolved, ignoring it...
[01:09:01]    --> /checkout/src/libcore/needle/needle.rs:106:41
[01:09:01]     |
[01:09:01] 106 |     /// calling `span`[`.into_parts()`](Span::into_parts). The range returned
[01:09:01]     |
[01:09:01]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:01] 
[01:09:01] 
[01:09:01] warning: `[std::needle::ext::starts_with]` cannot be resolved, ignoring it...
[01:09:01]    --> /checkout/src/libcore/needle/needle.rs:166:27
[01:09:01]     |
[01:09:01] 166 |     /// [`starts_with()`](std::needle::ext::starts_with) as well as providing the default
[01:09:01]     |
[01:09:01]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:01] 
[01:09:01] 
[01:09:01] warning: `[Consumer::trim_start]` cannot be resolved, ignoring it...
[01:09:01]    --> /checkout/src/libcore/needle/needle.rs:167:46
[01:09:01]     |
[01:09:01] 167 |     /// implementation for [`.trim_start()`](Consumer::trim_start).
[01:09:01]     |
[01:09:01]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:01] 
[01:09:01] 
[01:09:01] warning: `[Span::into_parts]` cannot be resolved, ignoring it...
[01:09:01]    --> /checkout/src/libcore/needle/needle.rs:170:41
[01:09:01]     |
[01:09:01] 170 |     /// calling `span`[`.into_parts()`](Span::into_parts). If a needle can be
[01:09:01]     |
[01:09:01]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:01] 
[01:09:01] 
[01:09:01] warning: `[std::needle::ext::trim_start]` cannot be resolved, ignoring it...
[01:09:01]    --> /checkout/src/libcore/needle/needle.rs:206:26
[01:09:01]     |
[01:09:01] 206 |     /// [`trim_start()`](std::needle::ext::trim_start).
[01:09:01]     |
[01:09:01]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:01] 
[01:09:01] 
[01:09:01] warning: `[Consumer::consume]` cannot be resolved, ignoring it...
[01:09:01]    --> /checkout/src/libcore/needle/needle.rs:211:24
[01:09:01]     |
[01:09:01] 211 |     /// [`.consume()`](Consumer::consume) is provided by default. Nevertheless,
[01:09:01]     |
[01:09:01]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:01] 
[01:09:02]     Finished release [optimized] target(s) in 39.44s
[01:09:02]     Finished release [optimized] target(s) in 39.44s
[01:09:03]  Documenting core v0.0.0 (/checkout/src/libcore)
[01:09:21] warning: `[rsplit]` cannot be resolved, ignoring it...
[01:09:21]      |
[01:09:21]      |
[01:09:21] 2883 |     /// Equivalent to [`rsplit`], except that the trailing substring is
[01:09:21]      |
[01:09:21] note: lint level defined here
[01:09:21]     --> src/libcore/lib.rs:63:9
[01:09:21]      |
[01:09:21]      |
[01:09:21] 63   | #![warn(intra_doc_link_resolution_failure)]
[01:09:21]      |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:09:21]      = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:21] 
[01:09:21] warning: `[Needle]` cannot be resolved, ignoring it...
[01:09:21]  --> src/libcore/needle/mod.rs:5:58
[01:09:21]   |
[01:09:21] 5 | //! This module provides traits to facilitate searching [`Needle`] in a [`Haystack`].
[01:09:21]   |
[01:09:21]   = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:21] 
[01:09:21] 
[01:09:21] warning: `[Haystack]` cannot be resolved, ignoring it...
[01:09:21]  --> src/libcore/needle/mod.rs:5:74
[01:09:21]   |
[01:09:21] 5 | //! This module provides traits to facilitate searching [`Needle`] in a [`Haystack`].
[01:09:21]   |
[01:09:21]   = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:21] 
[01:09:21] 
[01:09:21] warning: `[std::needle::ext::matches]` cannot be resolved, ignoring it...
[01:09:21]   --> src/libcore/needle/needle.rs:93:23
[01:09:21]    |
[01:09:21] 93 |     /// * [`matches`](std::needle::ext::matches)
[01:09:21]    |
[01:09:21]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:21] 
[01:09:21] 
[01:09:21] warning: `[std::needle::ext::contains]` cannot be resolved, ignoring it...
[01:09:21]   --> src/libcore/needle/needle.rs:94:24
[01:09:21]    |
[01:09:21] 94 |     /// * [`contains`](std::needle::ext::contains)
[01:09:21]    |
[01:09:21]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:21] 
[01:09:21] 
[01:09:21] warning: `[std::needle::ext::match_indices]` cannot be resolved, ignoring it...
[01:09:21]   --> src/libcore/needle/needle.rs:95:29
[01:09:21]    |
[01:09:21] 95 |     /// * [`match_indices`](std::needle::ext::match_indices)
[01:09:21]    |
[01:09:21]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:21] 
[01:09:21] warning: `[std::needle::ext::find]` cannot be resolved, ignoring it...
[01:09:21] warning: `[std::needle::ext::find]` cannot be resolved, ignoring it...
[01:09:21]   --> src/libcore/needle/needle.rs:96:20
[01:09:21]    |
[01:09:21] 96 |     /// * [`find`](std::needle::ext::find)
[01:09:21]    |
[01:09:21]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:21] 
[01:09:21] 
[01:09:21] warning: `[std::needle::ext::match_ranges]` cannot be resolved, ignoring it...
[01:09:21]   --> src/libcore/needle/needle.rs:97:28
[01:09:21]    |
[01:09:21] 97 |     /// * [`match_ranges`](std::needle::ext::match_ranges)
[01:09:21]    |
[01:09:21]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:21] 
[01:09:21] warning: `[std::needle::ext::find_range]` cannot be resolved, ignoring it...
[01:09:21] warning: `[std::needle::ext::find_range]` cannot be resolved, ignoring it...
[01:09:21]   --> src/libcore/needle/needle.rs:98:26
[01:09:21]    |
[01:09:21] 98 |     /// * [`find_range`](std::needle::ext::find_range)
[01:09:21]    |
[01:09:21]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:21] 
[01:09:21] warning: `[std::needle::ext::split]` cannot be resolved, ignoring it...
[01:09:21] warning: `[std::needle::ext::split]` cannot be resolved, ignoring it...
[01:09:21]   --> src/libcore/needle/needle.rs:99:21
[01:09:21]    |
[01:09:21] 99 |     /// * [`split`](std::needle::ext::split)
[01:09:21]    |
[01:09:21]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:21] 
[01:09:21] warning: `[std::needle::ext::split_terminator]` cannot be resolved, ignoring it...
[01:09:21] warning: `[std::needle::ext::split_terminator]` cannot be resolved, ignoring it...
[01:09:21]    --> src/libcore/needle/needle.rs:100:32
[01:09:21]     |
[01:09:21] 100 |     /// * [`split_terminator`](std::needle::ext::split_terminator)
[01:09:21]     |
[01:09:21]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:21] 
[01:09:21] 
[01:09:21] warning: `[std::needle::ext::splitn]` cannot be resolved, ignoring it...
[01:09:21]    --> src/libcore/needle/needle.rs:101:22
[01:09:21]     |
[01:09:21] 101 |     /// * [`splitn`](std::needle::ext::splitn)
[01:09:21]     |
[01:09:21]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:21] 
[01:09:21] 
[01:09:21] warning: `[std::needle::ext::replace_with]` cannot be resolved, ignoring it...
[01:09:21]    --> src/libcore/needle/needle.rs:102:28
[01:09:21]     |
[01:09:21] 102 |     /// * [`replace_with`](std::needle::ext::replace_with)
[01:09:21]     |
[01:09:21]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:21] 
[01:09:21] 
[01:09:21] warning: `[std::needle::ext::replacen_with]` cannot be resolved, ignoring it...
[01:09:21]    --> src/libcore/needle/needle.rs:103:29
[01:09:21]     |
[01:09:21] 103 |     /// * [`replacen_with`](std::needle::ext::replacen_with)
[01:09:21]     |
[01:09:21]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:21] 
[01:09:21] 
[01:09:21] warning: `[std::needle::ext::starts_with]` cannot be resolved, ignoring it...
[01:09:21]    --> src/libcore/needle/needle.rs:166:27
[01:09:21]     |
[01:09:21] 166 |     /// [`starts_with()`](std::needle::ext::starts_with) as well as providing the default
[01:09:21]     |
[01:09:21]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:21] 
[01:09:21] 
[01:09:21] warning: `[std::needle::ext::trim_start]` cannot be resolved, ignoring it...
[01:09:21]    --> src/libcore/needle/needle.rs:206:26
[01:09:21]     |
[01:09:21] 206 |     /// [`trim_start()`](std::needle::ext::trim_start).
[01:09:21]     |
[01:09:21]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:21] 
[01:09:21] 
[01:09:21] warning: `[std::needle::ext::rmatches]` cannot be resolved, ignoring it...
[01:09:21]    --> src/libcore/needle/needle.rs:257:24
[01:09:21]     |
[01:09:21] 257 |     /// * [`rmatches`](std::needle::ext::rmatches)
[01:09:21]     |
[01:09:21]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:21] 
[01:09:21] 
[01:09:21] warning: `[std::needle::ext::rmatch_indices]` cannot be resolved, ignoring it...
[01:09:21]    --> src/libcore/needle/needle.rs:258:30
[01:09:21]     |
[01:09:21] 258 |     /// * [`rmatch_indices`](std::needle::ext::rmatch_indices)
[01:09:21]     |
[01:09:21]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:21] 
[01:09:21] warning: `[std::needle::ext::find]` cannot be resolved, ignoring it...
[01:09:21] warning: `[std::needle::ext::find]` cannot be resolved, ignoring it...
[01:09:21]    --> src/libcore/needle/needle.rs:259:21
[01:09:21]     |
[01:09:21] 259 |     /// * [`rfind`](std::needle::ext::find)
[01:09:21]     |
[01:09:21]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:21] 
[01:09:21] 
[01:09:21] warning: `[std::needle::ext::rmatch_ranges]` cannot be resolved, ignoring it...
[01:09:21]    --> src/libcore/needle/needle.rs:260:29
[01:09:21]     |
[01:09:21] 260 |     /// * [`rmatch_ranges`](std::needle::ext::rmatch_ranges)
[01:09:21]     |
[01:09:21]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:21] 
[01:09:21] 
[01:09:21] warning: `[std::needle::ext::rfind_range]` cannot be resolved, ignoring it...
[01:09:21]    --> src/libcore/needle/needle.rs:261:27
[01:09:21]     |
[01:09:21] 261 |     /// * [`rfind_range`](std::needle::ext::rfind_range)
[01:09:21]     |
[01:09:21]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:21] 
[01:09:21] 
[01:09:21] warning: `[std::needle::ext::rsplit]` cannot be resolved, ignoring it...
[01:09:21]    --> src/libcore/needle/needle.rs:262:22
[01:09:21]     |
[01:09:21] 262 |     /// * [`rsplit`](std::needle::ext::rsplit)
[01:09:21]     |
[01:09:21]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:21] 
[01:09:21] warning: `[std::needle::ext::rsplit_terminator]` cannot be resolved, ignoring it...
[01:09:21] warning: `[std::needle::ext::rsplit_terminator]` cannot be resolved, ignoring it...
[01:09:21]    --> src/libcore/needle/needle.rs:263:33
[01:09:21]     |
[01:09:21] 263 |     /// * [`rsplit_terminator`](std::needle::ext::rsplit_terminator)
[01:09:21]     |
[01:09:21]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:21] 
[01:09:21] 
[01:09:21] warning: `[std::needle::ext::rsplitn]` cannot be resolved, ignoring it...
[01:09:21]    --> src/libcore/needle/needle.rs:264:23
[01:09:21]     |
[01:09:21] 264 |     /// * [`rsplitn`](std::needle::ext::rsplitn)
[01:09:21]     |
[01:09:21]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:21] 
[01:09:21] 
[01:09:21] warning: `[std::needle::ext::ends_with]` cannot be resolved, ignoring it...
[01:09:21]    --> src/libcore/needle/needle.rs:317:25
[01:09:21]     |
[01:09:21] 317 |     /// [`ends_with()`](std::needle::ext::ends_with) as well as providing the default
[01:09:21]     |
[01:09:21]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:21] 
[01:09:21] 
[01:09:21] warning: `[std::needle::ext::trim_end]` cannot be resolved, ignoring it...
[01:09:21]    --> src/libcore/needle/needle.rs:357:24
[01:09:21]     |
[01:09:21] 357 |     /// [`trim_end()`](std::needle::ext::trim_end).
[01:09:21]     |
[01:09:21]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:21] 
[01:09:21] warning: `[std::iter::DoubleEndedIterator]` cannot be resolved, ignoring it...
[01:09:21] warning: `[std::iter::DoubleEndedIterator]` cannot be resolved, ignoring it...
[01:09:21]    --> src/libcore/needle/needle.rs:394:36
[01:09:21]     |
[01:09:21] 394 | /// return [`DoubleEndedIterator`](std::iter::DoubleEndedIterator)s:
[01:09:21]     |
[01:09:21]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:21] 
[01:09:21] 
[01:09:21] warning: `[std::needle::ext::matches]` cannot be resolved, ignoring it...
[01:09:21]    --> src/libcore/needle/needle.rs:396:19
[01:09:21]     |
[01:09:21] 396 | /// * [`matches`](std::needle::ext::matches) /
[01:09:21]     |
[01:09:21]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:21] 
[01:09:21] 
[01:09:21] warning: `[std::needle::ext::rmatches]` cannot be resolved, ignoring it...
[01:09:21]    --> src/libcore/needle/needle.rs:397:22
[01:09:21]     |
[01:09:21] 397 | ///     [`rmatches`](std::needle::ext::rmatches)
[01:09:21]     |
[01:09:21]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:21] 
[01:09:21] 
[01:09:21] warning: `[std::needle::ext::match_indices]` cannot be resolved, ignoring it...
---
[01:09:34]     Checking rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[01:09:34]     Checking rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[01:09:34]     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[01:09:34]  Documenting std v0.0.0 (/checkout/src/libstd)
[01:09:42] error: `[matches]` cannot be resolved, ignoring it...
[01:09:42]   --> /checkout/src/libcore/needle/ext.rs:66:11
[01:09:42]    |
[01:09:42] 66 |         $(#[$forward_iterator_attribute])*
[01:09:42]    |
[01:09:42] note: lint level defined here
[01:09:42]   --> src/libstd/lib.rs:209:9
[01:09:42]    |
[01:09:42]    |
[01:09:42] 209| #![deny(intra_doc_link_resolution_failure)]
[01:09:42]    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:09:42]    = note: the link appears in this line:
[01:09:42]            
[01:09:42]            Created with the function [`matches`].
[01:09:42]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[rmatches]` cannot be resolved, ignoring it...
[01:09:42]   --> /checkout/src/libcore/needle/ext.rs:90:11
[01:09:42]    |
[01:09:42] 90 |         $(#[$reverse_iterator_attribute])*
[01:09:42]    |
[01:09:42]    = note: the link appears in this line:
[01:09:42]            
[01:09:42]            
[01:09:42]            Created with the function [`rmatches`].
[01:09:42]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[match_indices]` cannot be resolved, ignoring it...
[01:09:42]   --> /checkout/src/libcore/needle/ext.rs:66:11
[01:09:42]    |
[01:09:42] 66 |         $(#[$forward_iterator_attribute])*
[01:09:42]    |
[01:09:42]    = note: the link appears in this line:
[01:09:42]            
[01:09:42]            
[01:09:42]            Created with the function [`match_indices`].
[01:09:42]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[rmatch_indices]` cannot be resolved, ignoring it...
[01:09:42]   --> /checkout/src/libcore/needle/ext.rs:90:11
[01:09:42]    |
[01:09:42] 90 |         $(#[$reverse_iterator_attribute])*
[01:09:42]    |
[01:09:42]    = note: the link appears in this line:
[01:09:42]            
[01:09:42]            
[01:09:42]            Created with the function [`rmatch_indices`].
[01:09:42]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[match_ranges]` cannot be resolved, ignoring it...
[01:09:42]   --> /checkout/src/libcore/needle/ext.rs:66:11
[01:09:42]    |
[01:09:42] 66 |         $(#[$forward_iterator_attribute])*
[01:09:42]    |
[01:09:42]    = note: the link appears in this line:
[01:09:42]            
[01:09:42]            
[01:09:42]            Created with the function [`match_ranges`].
[01:09:42]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[rmatch_ranges]` cannot be resolved, ignoring it...
[01:09:42]   --> /checkout/src/libcore/needle/ext.rs:90:11
[01:09:42]    |
[01:09:42] 90 |         $(#[$reverse_iterator_attribute])*
[01:09:42]    |
[01:09:42]    = note: the link appears in this line:
[01:09:42]            
[01:09:42]            
[01:09:42]            Created with the function [`rmatch_ranges`].
[01:09:42]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[split]` cannot be resolved, ignoring it...
[01:09:42]   --> /checkout/src/libcore/needle/ext.rs:66:11
[01:09:42]    |
[01:09:42] 66 |         $(#[$forward_iterator_attribute])*
[01:09:42]    |
[01:09:42]    = note: the link appears in this line:
[01:09:42]            
[01:09:42]            
[01:09:42]            Created with the function [`split`].
[01:09:42]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[rsplit]` cannot be resolved, ignoring it...
[01:09:42]   --> /checkout/src/libcore/needle/ext.rs:90:11
[01:09:42]    |
[01:09:42] 90 |         $(#[$reverse_iterator_attribute])*
[01:09:42]    |
[01:09:42]    = note: the link appears in this line:
[01:09:42]            
[01:09:42]            
[01:09:42]            Created with the function [`rsplit`].
[01:09:42]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[split_terminator]` cannot be resolved, ignoring it...
[01:09:42]   --> /checkout/src/libcore/needle/ext.rs:66:11
[01:09:42]    |
[01:09:42] 66 |         $(#[$forward_iterator_attribute])*
[01:09:42]    |
[01:09:42]    = note: the link appears in this line:
[01:09:42]            
[01:09:42]            
[01:09:42]            Created with the function [`split_terminator`].
[01:09:42]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[rsplit_terminator]` cannot be resolved, ignoring it...
[01:09:42]   --> /checkout/src/libcore/needle/ext.rs:90:11
[01:09:42]    |
[01:09:42] 90 |         $(#[$reverse_iterator_attribute])*
[01:09:42]    |
[01:09:42]    = note: the link appears in this line:
[01:09:42]            
[01:09:42]            
[01:09:42]            Created with the function [`rsplit_terminator`].
[01:09:42]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[split]` cannot be resolved, ignoring it...
[01:09:42]    --> /checkout/src/libcore/needle/ext.rs:746:20
[01:09:42]     |
[01:09:42] 746 | /// Equivalent to [`split`], except that the trailing slice is skipped if empty.
[01:09:42]     |
[01:09:42]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[rsplit]` cannot be resolved, ignoring it...
[01:09:42]    --> /checkout/src/libcore/needle/ext.rs:767:20
[01:09:42]     |
[01:09:42] 767 | /// Equivalent to [`rsplit`], except that the trailing slice is skipped if empty.
[01:09:42]     |
[01:09:42]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[splitn]` cannot be resolved, ignoring it...
[01:09:42]   --> /checkout/src/libcore/needle/ext.rs:66:11
[01:09:42]    |
[01:09:42] 66 |         $(#[$forward_iterator_attribute])*
[01:09:42]    |
[01:09:42]    = note: the link appears in this line:
[01:09:42]            
[01:09:42]            
[01:09:42]            Created with the function [`splitn`].
[01:09:42]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[rsplitn]` cannot be resolved, ignoring it...
[01:09:42]   --> /checkout/src/libcore/needle/ext.rs:90:11
[01:09:42]    |
[01:09:42] 90 |         $(#[$reverse_iterator_attribute])*
[01:09:42]    |
[01:09:42]    = note: the link appears in this line:
[01:09:42]            
[01:09:42]            
[01:09:42]            Created with the function [`rsplitn`].
[01:09:42]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[Needle]` cannot be resolved, ignoring it...
[01:09:42]    --> /checkout/src/libcore/needle/needle.rs:149:34
[01:09:42]     |
[01:09:42] 149 | /// A consumer, for searching a [`Needle`] from a [`Hay`] anchored at the
[01:09:42]     |
[01:09:42]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[Hay]` cannot be resolved, ignoring it...
[01:09:42]    --> /checkout/src/libcore/needle/needle.rs:149:52
[01:09:42]     |
[01:09:42] 149 | /// A consumer, for searching a [`Needle`] from a [`Hay`] anchored at the
[01:09:42]     |
[01:09:42]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[Searcher]` cannot be resolved, ignoring it...
[01:09:42]    --> /checkout/src/libcore/needle/needle.rs:155:27
[01:09:42]     |
[01:09:42] 155 | /// See documentation of [`Searcher`] for an example.
[01:09:42]     |
[01:09:42]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[std::needle::ext::starts_with]` cannot be resolved, ignoring it...
[01:09:42]    --> /checkout/src/libcore/needle/needle.rs:166:27
[01:09:42]     |
[01:09:42] 166 |     /// [`starts_with()`](std::needle::ext::starts_with) as well as providing the default
[01:09:42]     |
[01:09:42]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[Consumer::trim_start]` cannot be resolved, ignoring it...
[01:09:42]    --> /checkout/src/libcore/needle/needle.rs:167:46
[01:09:42]     |
[01:09:42] 167 |     /// implementation for [`.trim_start()`](Consumer::trim_start).
[01:09:42]     |
[01:09:42]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[Span::into_parts]` cannot be resolved, ignoring it...
[01:09:42]    --> /checkout/src/libcore/needle/needle.rs:170:41
[01:09:42]     |
[01:09:42] 170 |     /// calling `span`[`.into_parts()`](Span::into_parts). If a needle can be
[01:09:42]     |
[01:09:42]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[std::needle::ext::trim_start]` cannot be resolved, ignoring it...
[01:09:42]    --> /checkout/src/libcore/needle/needle.rs:206:26
[01:09:42]     |
[01:09:42] 206 |     /// [`trim_start()`](std::needle::ext::trim_start).
[01:09:42]     |
[01:09:42]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[Consumer::consume]` cannot be resolved, ignoring it...
[01:09:42]    --> /checkout/src/libcore/needle/needle.rs:211:24
[01:09:42]     |
[01:09:42] 211 |     /// [`.consume()`](Consumer::consume) is provided by default. Nevertheless,
[01:09:42]     |
[01:09:42]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] error: `[std::iter::DoubleEndedIterator]` cannot be resolved, ignoring it...
[01:09:42] error: `[std::iter::DoubleEndedIterator]` cannot be resolved, ignoring it...
[01:09:42]    --> /checkout/src/libcore/needle/needle.rs:394:36
[01:09:42]     |
[01:09:42] 394 | /// return [`DoubleEndedIterator`](std::iter::DoubleEndedIterator)s:
[01:09:42]     |
[01:09:42]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[std::needle::ext::matches]` cannot be resolved, ignoring it...
[01:09:42]    --> /checkout/src/libcore/needle/needle.rs:396:19
[01:09:42]     |
[01:09:42] 396 | /// * [`matches`](std::needle::ext::matches) /
[01:09:42]     |
[01:09:42]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[std::needle::ext::rmatches]` cannot be resolved, ignoring it...
[01:09:42]    --> /checkout/src/libcore/needle/needle.rs:397:22
[01:09:42]     |
[01:09:42] 397 | ///     [`rmatches`](std::needle::ext::rmatches)
[01:09:42]     |
[01:09:42]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[std::needle::ext::match_indices]` cannot be resolved, ignoring it...
[01:09:42]    --> /checkout/src/libcore/needle/needle.rs:398:25
[01:09:42]     |
[01:09:42] 398 | /// * [`match_indices`](std::needle::ext::match_indices) /
[01:09:42]     |
[01:09:42]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[std::needle::ext::rmatch_indices]` cannot be resolved, ignoring it...
[01:09:42]    --> /checkout/src/libcore/needle/needle.rs:399:28
[01:09:42]     |
[01:09:42] 399 | ///     [`rmatch_indices`](std::needle::ext::rmatch_indices)
[01:09:42]     |
[01:09:42]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[std::needle::ext::match_ranges]` cannot be resolved, ignoring it...
[01:09:42]    --> /checkout/src/libcore/needle/needle.rs:400:24
[01:09:42]     |
[01:09:42] 400 | /// * [`match_ranges`](std::needle::ext::match_ranges) /
[01:09:42]     |
[01:09:42]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[std::needle::ext::rmatch_ranges]` cannot be resolved, ignoring it...
[01:09:42]    --> /checkout/src/libcore/needle/needle.rs:401:27
[01:09:42]     |
[01:09:42] 401 | ///     [`rmatch_ranges`](std::needle::ext::rmatch_ranges)
[01:09:42]     |
[01:09:42]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[std::needle::ext::split]` cannot be resolved, ignoring it...
[01:09:42]    --> /checkout/src/libcore/needle/needle.rs:402:17
[01:09:42]     |
[01:09:42] 402 | /// * [`split`](std::needle::ext::split) /
[01:09:42]     |
[01:09:42]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[std::needle::ext::rsplit]` cannot be resolved, ignoring it...
[01:09:42]    --> /checkout/src/libcore/needle/needle.rs:403:20
[01:09:42]     |
[01:09:42] 403 | ///     [`rsplit`](std::needle::ext::rsplit)
[01:09:42]     |
[01:09:42]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[std::needle::ext::split_terminator]` cannot be resolved, ignoring it...
[01:09:42]    --> /checkout/src/libcore/needle/needle.rs:404:28
[01:09:42]     |
[01:09:42] 404 | /// * [`split_terminator`](std::needle::ext::split_terminator) /
[01:09:42]     |
[01:09:42]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[std::needle::ext::rsplit_terminator]` cannot be resolved, ignoring it...
[01:09:42]    --> /checkout/src/libcore/needle/needle.rs:405:31
[01:09:42]     |
[01:09:42] 405 | ///     [`rsplit_terminator`](std::needle::ext::rsplit_terminator)
[01:09:42]     |
[01:09:42]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[std::needle::ext::splitn]` cannot be resolved, ignoring it...
[01:09:42]    --> /checkout/src/libcore/needle/needle.rs:406:18
[01:09:42]     |
[01:09:42] 406 | /// * [`splitn`](std::needle::ext::splitn) /
[01:09:42]     |
[01:09:42]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[std::needle::ext::rsplitn]` cannot be resolved, ignoring it...
[01:09:42]    --> /checkout/src/libcore/needle/needle.rs:407:21
[01:09:42]     |
[01:09:42] 407 | ///     [`rsplitn`](std::needle::ext::rsplitn)
[01:09:42]     |
[01:09:42]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[std::needle::ext::trim]` cannot be resolved, ignoring it...
[01:09:42]    --> /checkout/src/libcore/needle/needle.rs:445:16
[01:09:42]     |
[01:09:42] 445 | /// * [`trim`](std::needle::ext::trim)
[01:09:42]     |
[01:09:42]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[std::needle::ext::trim_start]` cannot be resolved, ignoring it...
[01:09:42]    --> /checkout/src/libcore/needle/needle.rs:448:20
[01:09:42]     |
[01:09:42] 448 | /// [`trim_start`](std::needle::ext::trim_start) and [`trim_end`](std::needle::ext::trim_end)
[01:09:42]     |
[01:09:42]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[std::needle::ext::trim_end]` cannot be resolved, ignoring it...
[01:09:42]    --> /checkout/src/libcore/needle/needle.rs:448:67
[01:09:42]     |
[01:09:42] 448 | /// [`trim_start`](std::needle::ext::trim_start) and [`trim_end`](std::needle::ext::trim_end)
[01:09:42]     |
[01:09:42]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[std::needle::ext::split]` cannot be resolved, ignoring it...
[01:09:42]    --> /checkout/src/libcore/needle/needle.rs:480:51
[01:09:42]     |
[01:09:42] 480 | /// When using search algorithms like [`split()`](std::needle::ext::split), users will
[01:09:42]     |
[01:09:42]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[Searcher]` cannot be resolved, ignoring it...
[01:09:42]    --> /checkout/src/libcore/needle/needle.rs:484:6
[01:09:42]     |
[01:09:42] 484 | /// [`Searcher`] of this needle.
[01:09:42]     |
[01:09:42]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[Haystack]` cannot be resolved, ignoring it...
[01:09:42]  --> /checkout/src/libcore/needle/haystack.rs:6:12
[01:09:42]   |
[01:09:42] 6 | /// Every [`Haystack`] type can be borrowed as references to `Hay` types. This
[01:09:42]   |
[01:09:42]   = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[std::needle::ext::ends_with]` cannot be resolved, ignoring it...
[01:09:42]    --> /checkout/src/libcore/needle/needle.rs:317:25
[01:09:42]     |
[01:09:42] 317 |     /// [`ends_with()`](std::needle::ext::ends_with) as well as providing the default
[01:09:42]     |
[01:09:42]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[ReverseConsumer::trim_end]` cannot be resolved, ignoring it...
[01:09:42]    --> /checkout/src/libcore/needle/needle.rs:318:44
[01:09:42]     |
[01:09:42] 318 |     /// implementation for [`.trim_end()`](ReverseConsumer::trim_end).
[01:09:42]     |
[01:09:42]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[Span::into_parts]` cannot be resolved, ignoring it...
[01:09:42]    --> /checkout/src/libcore/needle/needle.rs:321:41
[01:09:42]     |
[01:09:42] 321 |     /// calling `span`[`.into_parts()`](Span::into_parts). If a needle can be
[01:09:42]     |
[01:09:42]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[std::needle::ext::trim_end]` cannot be resolved, ignoring it...
[01:09:42]    --> /checkout/src/libcore/needle/needle.rs:357:24
[01:09:42]     |
[01:09:42] 357 |     /// [`trim_end()`](std::needle::ext::trim_end).
[01:09:42]     |
[01:09:42]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[ReverseConsumer::rconsume]` cannot be resolved, ignoring it...
[01:09:42]    --> /checkout/src/libcore/needle/needle.rs:360:25
[01:09:42]     |
[01:09:42] 360 |     /// [`.rconsume()`](ReverseConsumer::rconsume) is provided by default.
[01:09:42]     |
[01:09:42]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[Needle]` cannot be resolved, ignoring it...
[01:09:42]  --> /checkout/src/libcore/needle/needle.rs:5:34
[01:09:42]   |
[01:09:42] 5 | /// A searcher, for searching a [`Needle`] from a [`Hay`].
[01:09:42]   |
[01:09:42]   = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[Hay]` cannot be resolved, ignoring it...
[01:09:42]  --> /checkout/src/libcore/needle/needle.rs:5:52
[01:09:42]   |
[01:09:42] 5 | /// A searcher, for searching a [`Needle`] from a [`Hay`].
[01:09:42]   |
[01:09:42]   = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[std::needle::ext::matches]` cannot be resolved, ignoring it...
[01:09:42]   --> /checkout/src/libcore/needle/needle.rs:93:23
[01:09:42]    |
[01:09:42] 93 |     /// * [`matches`](std::needle::ext::matches)
[01:09:42]    |
[01:09:42]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[std::needle::ext::contains]` cannot be resolved, ignoring it...
[01:09:42]   --> /checkout/src/libcore/needle/needle.rs:94:24
[01:09:42]    |
[01:09:42] 94 |     /// * [`contains`](std::needle::ext::contains)
[01:09:42]    |
[01:09:42]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[std::needle::ext::match_indices]` cannot be resolved, ignoring it...
[01:09:42]   --> /checkout/src/libcore/needle/needle.rs:95:29
[01:09:42]    |
[01:09:42] 95 |     /// * [`match_indices`](std::needle::ext::match_indices)
[01:09:42]    |
[01:09:42]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[std::needle::ext::find]` cannot be resolved, ignoring it...
[01:09:42]   --> /checkout/src/libcore/needle/needle.rs:96:20
[01:09:42]    |
[01:09:42] 96 |     /// * [`find`](std::needle::ext::find)
[01:09:42]    |
[01:09:42]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[std::needle::ext::match_ranges]` cannot be resolved, ignoring it...
[01:09:42]   --> /checkout/src/libcore/needle/needle.rs:97:28
[01:09:42]    |
[01:09:42] 97 |     /// * [`match_ranges`](std::needle::ext::match_ranges)
[01:09:42]    |
[01:09:42]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[std::needle::ext::find_range]` cannot be resolved, ignoring it...
[01:09:42]   --> /checkout/src/libcore/needle/needle.rs:98:26
[01:09:42]    |
[01:09:42] 98 |     /// * [`find_range`](std::needle::ext::find_range)
[01:09:42]    |
[01:09:42]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[std::needle::ext::split]` cannot be resolved, ignoring it...
[01:09:42]   --> /checkout/src/libcore/needle/needle.rs:99:21
[01:09:42]    |
[01:09:42] 99 |     /// * [`split`](std::needle::ext::split)
[01:09:42]    |
[01:09:42]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:09:42] 
[01:09:42] 
[01:09:42] error: `[std::needle::ext::split_terminator]` cannot be resolved, ignoring it...
[01:09:42]    --> /checkout/src/libcore/needle/needle.rs:100:32
[01:09:42]     |
---
travis_time:end:08507e7e:start=1554066238812619179,finish=1554066238819502066,duration=6882887
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:030f74b8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06adf500
travis_time:start:06adf500
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:081e298e
$ dmesg | grep -i kill
