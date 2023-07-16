
2019-07-30T12:40:57.4101519Z +   | ^
2019-07-30T12:40:57.4101866Z + help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
2019-07-30T12:40:57.4102029Z +   |
2019-07-30T12:40:57.4102336Z + 1 | '``
2019-07-30T12:40:57.4107146Z + 
2019-07-30T12:40:57.4107205Z + error: unknown start of token: \
2019-07-30T12:40:57.4107606Z +  --> <rustdoc-highlighting>:2:1
2019-07-30T12:40:57.4107655Z +   |
---
2019-07-30T12:40:57.4109720Z +  --> <rustdoc-highlighting>:3:30
2019-07-30T12:40:57.4109760Z +   |
2019-07-30T12:40:57.4109814Z + 3 |    |     ^^^^^^ did you mean `baz::foobar`?
2019-07-30T12:40:57.4109856Z +   |                              ^
2019-07-30T12:40:57.4110095Z + help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
2019-07-30T12:40:57.4110155Z +   |
2019-07-30T12:40:57.4110357Z + 3 |    |     ^^^^^^ did you mean 'baz::foobar`?
2019-07-30T12:40:57.4110436Z + 
2019-07-30T12:40:57.4110489Z + error: unknown start of token: \
2019-07-30T12:40:57.4110678Z +  --> <rustdoc-highlighting>:1:1
2019-07-30T12:40:57.4110716Z +   |
2019-07-30T12:40:57.4110716Z +   |
2019-07-30T12:40:57.4110969Z + 1 | \__________pkt->size___________/          \_result->size_/ \__pkt->size__/
2019-07-30T12:40:57.4111054Z 97 
2019-07-30T12:40:57.4111088Z 98 
2019-07-30T12:40:57.4111130Z 
2019-07-30T12:40:57.4111153Z 
2019-07-30T12:40:57.4111153Z 
2019-07-30T12:40:57.4111193Z The actual stderr differed from the expected stderr.
2019-07-30T12:40:57.4111469Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/invalid-syntax/invalid-syntax.stderr
2019-07-30T12:40:57.4111929Z To update references, rerun the tests and pass the `--bless` flag
2019-07-30T12:40:57.4112159Z To only update this specific test, also pass `--test-args invalid-syntax.rs`
2019-07-30T12:40:57.4112243Z error: 1 errors occurred comparing output.
2019-07-30T12:40:57.4112281Z status: exit code: 0
2019-07-30T12:40:57.4112281Z status: exit code: 0
2019-07-30T12:40:57.4112886Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/invalid-syntax.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/invalid-syntax" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/invalid-syntax/auxiliary"
2019-07-30T12:40:57.4113163Z ------------------------------------------
2019-07-30T12:40:57.4113215Z 
2019-07-30T12:40:57.4113405Z ------------------------------------------
2019-07-30T12:40:57.4113444Z stderr:
2019-07-30T12:40:57.4113444Z stderr:
2019-07-30T12:40:57.4113639Z ------------------------------------------
2019-07-30T12:40:57.4113681Z error: unknown start of token: \
2019-07-30T12:40:57.4136600Z  --> <doctest>:1:1
2019-07-30T12:40:57.4136663Z   |
2019-07-30T12:40:57.4136947Z 1 | \__________pkt->size___________/          \_result->size_/ \__pkt->size__/
2019-07-30T12:40:57.4137051Z 
2019-07-30T12:40:57.4137112Z error: unknown start of token: \
2019-07-30T12:40:57.4137334Z  --> <doctest>:1:43
2019-07-30T12:40:57.4137390Z   |
2019-07-30T12:40:57.4137390Z   |
2019-07-30T12:40:57.4137662Z 1 | \__________pkt->size___________/          \_result->size_/ \__pkt->size__/
2019-07-30T12:40:57.4137759Z 
2019-07-30T12:40:57.4137802Z error: unknown start of token: \
2019-07-30T12:40:57.4138368Z  --> <doctest>:1:60
2019-07-30T12:40:57.4138420Z   |
2019-07-30T12:40:57.4138420Z   |
2019-07-30T12:40:57.4138872Z 1 | \__________pkt->size___________/          \_result->size_/ \__pkt->size__/
2019-07-30T12:40:57.4138949Z 
2019-07-30T12:40:57.4139000Z warning: could not parse code block as Rust code
2019-07-30T12:40:57.4139265Z   --> /checkout/src/test/rustdoc-ui/invalid-syntax.rs:3:5
2019-07-30T12:40:57.4139308Z    |
2019-07-30T12:40:57.4139308Z    |
2019-07-30T12:40:57.4139345Z LL |   /// 