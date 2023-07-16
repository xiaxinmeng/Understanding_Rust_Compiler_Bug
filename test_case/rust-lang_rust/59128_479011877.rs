plain
travis_time:end:0176e499:start=1554209458571173143,finish=1554209544378375130,duration=85807201987
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[01:11:47] .................................................................................................... 2900/5517
[01:11:51] .................................................................................................... 3000/5517
[01:11:54] .................................................................................................... 3100/5517
[01:11:58] .................................................................................................... 3200/5517
[01:12:02] ..........................................................F......................................... 3300/5517
[01:12:09] ............................................................................................ii...i.. 3500/5517
[01:12:13] ii.................................................................................................. 3600/5517
[01:12:17] .................................................................................................... 3700/5517
[01:12:20] .................................................................................................... 3800/5517
---
[01:13:26] diff of stderr:
[01:13:26] 
[01:13:26] 380       "rendered": null
[01:13:26] 381     }
[01:13:26] 382   ],
[01:13:26] -   "rendered": "/u001b[0m/u001b[1m/u001b[38;5;9merror[E0412]/u001b[0m/u001b[0m/u001b[1m: cannot find type `Iter` in this scope/u001b[0m
[01:13:26] - /u001b[0m  /u001b[0m/u001b[0m/u001b[1m/u001b[38;5;12m--> /u001b[0m/u001b[0m$DIR/use_suggestion_json.rs:11:12/u001b[0m
[01:13:26] - /u001b[0m   /u001b[0m/u001b[0m/u001b[1m/u001b[38;5;12m|/u001b[0m
[01:13:26] - /u001b[0m/u001b[1m/u001b[38;5;12mLL/u001b[0m/u001b[0m /u001b[0m/u001b[0m/u001b[1m/u001b[38;5;12m| /u001b[0m/u001b[0m    let x: Iter;/u001b[0m
[01:13:26] - /u001b[0m   /u001b[0m/u001b[0m/u001b[1m/u001b[38;5;12m| /u001b[0m/u001b[0m           /u001b[0m/u001b[0m/u001b[1m/u001b[38;5;9m^^^^/u001b[0m/u001b[0m /u001b[0m/u001b[0m/u001b[1m/u001b[38;5;9mnot found in this scope/u001b[0m
[01:13:26] - /u001b[0m/u001b[1m/u001b[38;5;14mhelp/u001b[0m/u001b[0m: possible candidates are found in other modules, you can import them into scope/u001b[0m
[01:13:26] - /u001b[0m   /u001b[0m/u001b[0m/u001b[1m/u001b[38;5;12m|/u001b[0m
[01:13:26] - /u001b[0m/u001b[1m/u001b[38;5;12mLL/u001b[0m/u001b[0m /u001b[0m/u001b[0m/u001b[1m/u001b[38;5;12m| /u001b[0m/u001b[0muse std::collections::binary_heap::Iter;/u001b[0m
[01:13:26] - /u001b[0m   /u001b[0m/u001b[0m/u001b[1m/u001b[38;5;12m|/u001b[0m
[01:13:26] - /u001b[0m/u001b[1m/u001b[38;5;12mLL/u001b[0m/u001b[0m /u001b[0m/u001b[0m/u001b[1m/u001b[38;5;12m| /u001b[0m/u001b[0muse std::collections::btree_map::Iter;/u001b[0m
[01:13:26] - /u001b[0m   /u001b[0m/u001b[0m/u001b[1m/u001b[38;5;12m|/u001b[0m
[01:13:26] - /u001b[0m/u001b[1m/u001b[38;5;12mLL/u001b[0m/u001b[0m /u001b[0m/u001b[0m/u001b[1m/u001b[38;5;12m| /u001b[0m/u001b[0muse std::collections::btree_set::Iter;/u001b[0m
[01:13:26] - /u001b[0m   /u001b[0m/u001b[0m/u001b[1m/u001b[38;5;12m|/u001b[0m
[01:13:26] - /u001b[0m/u001b[1m/u001b[38;5;12mLL/u001b[0m/u001b[0m /u001b[0m/u001b[0m/u001b[1m/u001b[38;5;12m| /u001b[0m/u001b[0muse std::collections::hash_map::Iter;/u001b[0m
[01:13:26] - /u001b[0m   /u001b[0m/u001b[0m/u001b[1m/u001b[38;5;12m|/u001b[0m
[01:13:26] - /u001b[0mand 8 other candidates/u001b[0m
[01:13:26] +   "rendered": "\u001b[0m\u001b[1m\u001b[38;5;9merror[E0412]\u001b[0m\u001b[0m\u001b[1m: cannot find type `Iter` in this scope\u001b[0m
[01:13:26] + \u001b[0m  \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m--> \u001b[0m\u001b[0m$DIR/use_suggestion_json.rs:11:12\u001b[0m
[01:13:26] + \u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m|\u001b[0m
[01:13:26] + \u001b[0m\u001b[1m\u001b[38;5;12mLL\u001b[0m\u001b[0m \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m| \u001b[0m\u001b[0m    let x: Iter;\u001b[0m
[01:13:26] + \u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m| \u001b[0m\u001b[0m           \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;9m^^^^\u001b[0m\u001b[0m \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;9mnot found in this scope\u001b[0m
[01:13:26] + \u001b[0m\u001b[1m\u001b[38;5;14mhelp\u001b[0m\u001b[0m: possible candidates are found in other modules, you can import them into scope\u001b[0m
[01:13:26] + \u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m|\u001b[0m
[01:13:26] + \u001b[0m\u001b[1m\u001b[38;5;12mLL\u001b[0m\u001b[0m \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m| \u001b[0m\u001b[0muse std::collections::binary_heap::Iter;\u001b[0m
[01:13:26] + \u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m|\u001b[0m
[01:13:26] + \u001b[0m\u001b[1m\u001b[38;5;12mLL\u001b[0m\u001b[0m \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m| \u001b[0m\u001b[0muse std::collections::btree_map::Iter;\u001b[0m
[01:13:26] + \u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m|\u001b[0m
[01:13:26] + \u001b[0m\u001b[1m\u001b[38;5;12mLL\u001b[0m\u001b[0m \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m| \u001b[0m\u001b[0muse std::collections::btree_set::Iter;\u001b[0m
[01:13:26] + \u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m|\u001b[0m
[01:13:26] + \u001b[0m\u001b[1m\u001b[38;5;12mLL\u001b[0m\u001b[0m \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m| \u001b[0m\u001b[0muse std::collections::hash_map::Iter;\u001b[0m
[01:13:26] + \u001b[0m   \u001b[0m\u001b[0m\u001b[1m\u001b[38;5;12m|\u001b[0m
[01:13:26] + \u001b[0mand 8 other candidates\u001b[0m
[01:13:26] 400 "
[01:13:26] 401 }
[01:13:26] 
[01:13:26] 405   "level": "error",
[01:13:26] 405   "level": "error",
[01:13:26] 406   "spans": [],
[01:13:26] 407   "children": [],
[01:13:26] -   "rendered": "/u001b[0m/u001b[1m/u001b[38;5;9merror/u001b[0m/u001b[0m/u001b[1m: aborting due to previous error/u001b[0m
[01:13:26] +   "rendered": "\u001b[0m\u001b[1m\u001b[38;5;9merror\u001b[0m\u001b[0m\u001b[1m: aborting due to previous error\u001b[0m
[01:13:26] 410 "
[01:13:26] 411 }
[01:13:26] 
[01:13:26] 
[01:13:26] 415   "level": "",
[01:13:26] 416   "spans": [],
[01:13:26] 417   "children": [],
[01:13:26] -   "rendered": "/u001b[0m/u001b[1mFor more information about this error, try `rustc --explain E0412`./u001b[0m
[01:13:26] +   "rendered": "\u001b[0m\u001b[1mFor more information about this error, try `rustc --explain E0412`.\u001b[0m
[01:13:26] 419 "
[01:13:26] 421 
[01:13:26] 
[01:13:26] 
[01:13:26] The actual stderr differed from the expected stderr.
[01:13:26] The actual stderr differed from the expected stderr.
[01:13:26] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/use_suggestion_json/use_suggestion_json.stderr
[01:13:26] To update references, rerun the tests and pass the `--bless` flag
[01:13:26] To only update this specific test, also pass `--test-args lint/use_suggestion_json.rs`
[01:13:26] error: 1 errors occurred comparing output.
[01:13:26] error: 1 errors occurred comparing output.
[01:13:26] failed to decode compiler output as json: `EOF while parsing an object at line 1 column 1`
[01:13:26] line: {
[01:13:26] output: {
[01:13:26]   "message": "cannot find type `Iter` in this scope",
[01:13:26]   "code": {
[01:13:26]     "code": "E0412",
[01:13:26]     "explanation": "\nThe type name used is not in scope.\n\nErroneous code examples:\n\n