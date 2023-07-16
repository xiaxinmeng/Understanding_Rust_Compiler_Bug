
# bad: [78ff609d7375ee2a2c6d0222776ac612eb1b75be] Auto merge of #55152 - nikomatsakis:nll-issue-54571-type-annot-in-constants, r=pnkfelix
# good: [e7f5d48059aa14cc2808473564deadd72d1e818d] Auto merge of #54976 - davidtwco:issue-52663-special-case-closures, r=nikomatsakis
git bisect start '78ff609d7' 'e7f5d4805'
# good: [0724efd9a1aac9cf4620795786fb8e896fbb17b3] Rollup merge of #55013 - matthewjasper:propagate-generator-bounds, r=nikomatsakis
git bisect good 0724efd9a1aac9cf4620795786fb8e896fbb17b3
# skip: [e94959b9369d611aed86a4c179406a96e983278a] propagate user-type annotation for constants in expressions
git bisect skip e94959b9369d611aed86a4c179406a96e983278a
# skip: [a0a3b4c0585129405c09a9be46a1a3f8d3411b3b] replace `UserTypeAnnotation::AdtDef` with `TypeOf`
git bisect skip a0a3b4c0585129405c09a9be46a1a3f8d3411b3b
# bad: [9a7bb0ef249258aacf144d04f5d437ba70533128] normalize the self-type that we extract from impl
git bisect bad 9a7bb0ef249258aacf144d04f5d437ba70533128
# bad: [2d98e9e0aa6a990ec12f476c495be6720ad81f51] create type ascription for any cast
git bisect bad 2d98e9e0aa6a990ec12f476c495be6720ad81f51
# good: [dbab381da1a46a18e46a04a61156aec40c59a4f6] Auto merge of #55040 - scalexm:param-env, r=nikomatsakis
git bisect good dbab381da1a46a18e46a04a61156aec40c59a4f6
# skip: [e7ab33e7a61ff046f1736f1b027c16d9494e20b8] type_check/mod.rs: rustfmt
git bisect skip e7ab33e7a61ff046f1736f1b027c16d9494e20b8
# good: [ebdfda64f893f30675320e15349b2948b2194ea9] convert `FnDef` to `TypeOf`, which is more general
git bisect good ebdfda64f893f30675320e15349b2948b2194ea9
# bad: [121f3c8d19c3549ab0b51a14034ffb8b097faf42] normalize after substitution
git bisect bad 121f3c8d19c3549ab0b51a14034ffb8b097faf42
# skip: [f99300fcbdfec2908aeb93c823fc37f92a4d2d30] pull `relate_type_and_user_type` code into `type_check` module
git bisect skip f99300fcbdfec2908aeb93c823fc37f92a4d2d30
# only skipped commits left to test
# possible first bad commit: [121f3c8d19c3549ab0b51a14034ffb8b097faf42] normalize after substitution
# possible first bad commit: [f99300fcbdfec2908aeb93c823fc37f92a4d2d30] pull `relate_type_and_user_type` code into `type_check` module
# possible first bad commit: [e7ab33e7a61ff046f1736f1b027c16d9494e20b8] type_check/mod.rs: rustfmt
# possible first bad commit: [a0a3b4c0585129405c09a9be46a1a3f8d3411b3b] replace `UserTypeAnnotation::AdtDef` with `TypeOf`
# possible first bad commit: [e94959b9369d611aed86a4c179406a96e983278a] propagate user-type annotation for constants in expressions
