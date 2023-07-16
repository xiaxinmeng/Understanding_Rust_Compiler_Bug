plain
Successfully built 6aac670b3138
Successfully tagged rust-ci:latest
Built container sha256:6aac670b313856578b05df5f5f48e315ba3347abecc37bc2ea6ea9025d9f6b31
Uploading finished image to https://ci-caches.rust-lang.org/docker/5113f42bfd9b1c95a1d3883587ba8dad4c66787bb247c62a9135b39daac8eb518d79354d0685bc057d2ca9275608acb8acc0d283225531823eccf30a4e542d9b
upload failed: - to s3://rust-lang-ci-sccache2/docker/5113f42bfd9b1c95a1d3883587ba8dad4c66787bb247c62a9135b39daac8eb518d79354d0685bc057d2ca9275608acb8acc0d283225531823eccf30a4e542d9b Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-13]
---
failures:

---- [rustdoc] src/test/rustdoc/duplicate_impls/issue-33054.rs stdout ----

error: htmldocck failed!
status: exit status: 1
command: "/usr/bin/python3" "/checkout/src/etc/htmldocck.py" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/duplicate_impls/issue-33054" "/checkout/src/test/rustdoc/duplicate_impls/issue-33054.rs"
stdout: none
--- stderr -------------------------------
3: @has check failed
 File does not exist 'issue_33054/impls/struct.Foo.html'
 // @has issue_33054/impls/struct.Foo.html
4: @has check failed
 File does not exist 'issue_33054/impls/struct.Foo.html'
 // @has - '//h3[@class="code-header"]' 'impl Foo'
5: @has check failed
 File does not exist 'issue_33054/impls/struct.Foo.html'
 // @has - '//h3[@class="code-header"]' 'impl Bar for Foo'
6: @count check failed
 File does not exist 'issue_33054/impls/struct.Foo.html'
 // @count - '//*[@id="trait-implementations-list"]//*[@class="impl has-srclink"]' 1
7: @count check failed
 File does not exist 'issue_33054/impls/struct.Foo.html'
 // @count - '//*[@id="main-content"]/div[@id="implementations-list"]/details/summary/*[@class="impl has-srclink"]' 1
8: @has check failed
 File does not exist 'issue_33054/impls/bar/trait.Bar.html'
 // @has issue_33054/impls/bar/trait.Bar.html
9: @has check failed
 File does not exist 'issue_33054/impls/bar/trait.Bar.html'
 // @has - '//h3[@class="code-header"]' 'impl Bar for Foo'
10: @count check failed
 File does not exist 'issue_33054/impls/bar/trait.Bar.html'
 // @count - '//*[@class="struct"]' 1
Encountered 8 errors
------------------------------------------


