plain
.................................................................................................... 9300/11680
.................................................................................................... 9400/11680
.................................................................................................... 9500/11680
.......................i......i..................................................................... 9600/11680
...................F..F..........F...................................iiiiiii..iiiiii.i.............. 9700/11680
.................................................................................................... 9900/11680
.................................................................................................... 10000/11680
.............................................................F...................................... 10100/11680
.................................................................................................... 10200/11680
---
failures:

---- [ui] ui/associated-type-bounds/duplicate.rs#full_tait stdout ----

error in revision `full_tait`: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:116: unexpected error: '116:40: 116:50: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]'

error in revision `full_tait`: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:118: unexpected error: '118:43: 118:56: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]'

error in revision `full_tait`: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:121: unexpected error: '121:36: 121:46: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]'

error in revision `full_tait`: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:123: unexpected error: '123:36: 123:46: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]'

error in revision `full_tait`: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:125: unexpected error: '125:39: 125:52: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]'

error in revision `full_tait`: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:127: unexpected error: '127:34: 127:44: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]'

error in revision `full_tait`: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:129: unexpected error: '129:34: 129:44: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]'

error in revision `full_tait`: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:131: unexpected error: '131:37: 131:50: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]'

error in revision `full_tait`: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:133: unexpected error: '133:45: 133:55: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]'

error in revision `full_tait`: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:135: unexpected error: '135:45: 135:55: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]'

error in revision `full_tait`: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:137: unexpected error: '137:48: 137:61: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]'

error in revision `full_tait`: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:139: unexpected error: '139:46: 139:56: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]'

error in revision `full_tait`: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:139: unexpected error: '139:46: 139:56: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]'

error in revision `full_tait`: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:142: unexpected error: '142:46: 142:56: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]'

error in revision `full_tait`: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:142: unexpected error: '142:46: 142:56: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]'

error in revision `full_tait`: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:145: unexpected error: '145:49: 145:62: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]'

error in revision `full_tait`: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:145: unexpected error: '145:49: 145:62: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]'

error in revision `full_tait`: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:155: unexpected error: '155:40: 155:50: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]'

error in revision `full_tait`: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:157: unexpected error: '157:44: 157:54: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]'

error in revision `full_tait`: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:159: unexpected error: '159:43: 159:56: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]'

error in revision `full_tait`: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:148: unexpected error: '148:43: 148:53: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]'

error in revision `full_tait`: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:150: unexpected error: '150:43: 150:53: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]'

error in revision `full_tait`: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:152: unexpected error: '152:46: 152:59: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]'

error in revision `full_tait`: 23 unexpected errors found, 0 expected errors not found
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-type-bounds/duplicate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full_tait" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/duplicate.full_tait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/duplicate.full_tait/auxiliary"
    Error {
        line_num: 116,
        kind: Some(
            Error,
            Error,
        ),
        msg: "116:40: 116:50: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]",
    Error {
        line_num: 118,
        kind: Some(
            Error,
            Error,
        ),
        msg: "118:43: 118:56: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]",
    Error {
        line_num: 121,
        kind: Some(
            Error,
            Error,
        ),
        msg: "121:36: 121:46: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]",
    Error {
        line_num: 123,
        kind: Some(
            Error,
            Error,
        ),
        msg: "123:36: 123:46: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]",
    Error {
        line_num: 125,
        kind: Some(
            Error,
            Error,
        ),
        msg: "125:39: 125:52: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]",
    Error {
        line_num: 127,
        kind: Some(
            Error,
            Error,
        ),
        msg: "127:34: 127:44: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]",
    Error {
        line_num: 129,
        kind: Some(
            Error,
            Error,
        ),
        msg: "129:34: 129:44: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]",
    Error {
        line_num: 131,
        kind: Some(
            Error,
            Error,
        ),
        msg: "131:37: 131:50: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]",
    Error {
        line_num: 133,
        kind: Some(
            Error,
            Error,
        ),
        msg: "133:45: 133:55: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]",
    Error {
        line_num: 135,
        kind: Some(
            Error,
            Error,
        ),
        msg: "135:45: 135:55: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]",
    Error {
        line_num: 137,
        kind: Some(
            Error,
            Error,
        ),
        msg: "137:48: 137:61: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]",
    Error {
        line_num: 139,
        kind: Some(
            Error,
            Error,
        ),
        msg: "139:46: 139:56: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]",
    Error {
        line_num: 139,
        kind: Some(
            Error,
            Error,
        ),
        msg: "139:46: 139:56: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]",
    Error {
        line_num: 142,
        kind: Some(
            Error,
            Error,
        ),
        msg: "142:46: 142:56: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]",
    Error {
        line_num: 142,
        kind: Some(
            Error,
            Error,
        ),
        msg: "142:46: 142:56: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]",
    Error {
        line_num: 145,
        kind: Some(
            Error,
            Error,
        ),
        msg: "145:49: 145:62: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]",
    Error {
        line_num: 145,
        kind: Some(
            Error,
            Error,
        ),
        msg: "145:49: 145:62: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]",
    Error {
        line_num: 155,
        kind: Some(
            Error,
            Error,
        ),
        msg: "155:40: 155:50: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]",
    Error {
        line_num: 157,
        kind: Some(
            Error,
            Error,
        ),
        msg: "157:44: 157:54: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]",
    Error {
        line_num: 159,
        kind: Some(
            Error,
            Error,
        ),
        msg: "159:43: 159:56: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]",
    Error {
        line_num: 148,
        kind: Some(
            Error,
            Error,
        ),
        msg: "148:43: 148:53: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]",
    Error {
        line_num: 150,
        kind: Some(
            Error,
            Error,
        ),
        msg: "150:43: 150:53: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]",
    Error {
        line_num: 152,
        kind: Some(
            Error,
            Error,
        ),
        msg: "152:46: 152:59: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]",
]


thread '[ui] ui/associated-type-bounds/duplicate.rs#full_tait' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1492:13

---- [ui] ui/associated-type-bounds/duplicate.rs#min_tait stdout ----


error in revision `min_tait`: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:116: unexpected error: '116:40: 116:50: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]'

error in revision `min_tait`: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:118: unexpected error: '118:43: 118:56: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]'

error in revision `min_tait`: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:121: unexpected error: '121:36: 121:46: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]'

error in revision `min_tait`: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:123: unexpected error: '123:36: 123:46: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]'

error in revision `min_tait`: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:125: unexpected error: '125:39: 125:52: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]'

error in revision `min_tait`: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:127: unexpected error: '127:34: 127:44: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]'

error in revision `min_tait`: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:129: unexpected error: '129:34: 129:44: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]'

error in revision `min_tait`: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:131: unexpected error: '131:37: 131:50: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]'

error in revision `min_tait`: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:133: unexpected error: '133:45: 133:55: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]'

error in revision `min_tait`: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:135: unexpected error: '135:45: 135:55: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]'

error in revision `min_tait`: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:137: unexpected error: '137:48: 137:61: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]'

error in revision `min_tait`: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:139: unexpected error: '139:46: 139:56: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]'

error in revision `min_tait`: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:139: unexpected error: '139:46: 139:56: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]'

error in revision `min_tait`: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:142: unexpected error: '142:46: 142:56: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]'

error in revision `min_tait`: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:142: unexpected error: '142:46: 142:56: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]'

error in revision `min_tait`: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:145: unexpected error: '145:49: 145:62: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]'

error in revision `min_tait`: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:145: unexpected error: '145:49: 145:62: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]'

error in revision `min_tait`: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:155: unexpected error: '155:40: 155:50: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]'

error in revision `min_tait`: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:157: unexpected error: '157:44: 157:54: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]'

error in revision `min_tait`: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:159: unexpected error: '159:43: 159:56: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]'

error in revision `min_tait`: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:148: unexpected error: '148:43: 148:53: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]'

error in revision `min_tait`: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:150: unexpected error: '150:43: 150:53: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]'

error in revision `min_tait`: /checkout/src/test/ui/associated-type-bounds/duplicate.rs:152: unexpected error: '152:46: 152:59: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]'

error in revision `min_tait`: 23 unexpected errors found, 0 expected errors not found
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-type-bounds/duplicate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "min_tait" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/duplicate.min_tait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-type-bounds/duplicate.min_tait/auxiliary"
    Error {
        line_num: 116,
        kind: Some(
            Error,
            Error,
        ),
        msg: "116:40: 116:50: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]",
    Error {
        line_num: 118,
        kind: Some(
            Error,
            Error,
        ),
        msg: "118:43: 118:56: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]",
    Error {
        line_num: 121,
        kind: Some(
            Error,
            Error,
        ),
        msg: "121:36: 121:46: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]",
    Error {
        line_num: 123,
        kind: Some(
            Error,
            Error,
        ),
        msg: "123:36: 123:46: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]",
    Error {
        line_num: 125,
        kind: Some(
            Error,
            Error,
        ),
        msg: "125:39: 125:52: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]",
    Error {
        line_num: 127,
        kind: Some(
            Error,
            Error,
        ),
        msg: "127:34: 127:44: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]",
    Error {
        line_num: 129,
        kind: Some(
            Error,
            Error,
        ),
        msg: "129:34: 129:44: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]",
    Error {
        line_num: 131,
        kind: Some(
            Error,
            Error,
        ),
        msg: "131:37: 131:50: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]",
    Error {
        line_num: 133,
        kind: Some(
            Error,
            Error,
        ),
        msg: "133:45: 133:55: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]",
    Error {
        line_num: 135,
        kind: Some(
            Error,
            Error,
        ),
        msg: "135:45: 135:55: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]",
    Error {
        line_num: 137,
        kind: Some(
            Error,
            Error,
        ),
        msg: "137:48: 137:61: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]",
    Error {
        line_num: 139,
        kind: Some(
            Error,
            Error,
        ),
        msg: "139:46: 139:56: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]",
    Error {
        line_num: 139,
        kind: Some(
            Error,
            Error,
        ),
        msg: "139:46: 139:56: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]",
    Error {
        line_num: 142,
        kind: Some(
            Error,
            Error,
        ),
        msg: "142:46: 142:56: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]",
    Error {
        line_num: 142,
        kind: Some(
            Error,
            Error,
        ),
        msg: "142:46: 142:56: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]",
    Error {
        line_num: 145,
        kind: Some(
            Error,
            Error,
        ),
        msg: "145:49: 145:62: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]",
    Error {
        line_num: 145,
        kind: Some(
            Error,
            Error,
        ),
        msg: "145:49: 145:62: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]",
    Error {
        line_num: 155,
        kind: Some(
            Error,
            Error,
        ),
        msg: "155:40: 155:50: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]",
    Error {
        line_num: 157,
        kind: Some(
            Error,
            Error,
        ),
        msg: "157:44: 157:54: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]",
    Error {
        line_num: 159,
        kind: Some(
            Error,
            Error,
        ),
        msg: "159:43: 159:56: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]",
    Error {
        line_num: 148,
        kind: Some(
            Error,
            Error,
        ),
        msg: "148:43: 148:53: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]",
    Error {
        line_num: 150,
        kind: Some(
            Error,
            Error,
        ),
        msg: "150:43: 150:53: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]",
    Error {
        line_num: 152,
        kind: Some(
            Error,
            Error,
        ),
        msg: "152:46: 152:59: the value of the associated type `Item` (from trait `Iterator`) is already specified [E0719]",
]


thread '[ui] ui/associated-type-bounds/duplicate.rs#min_tait' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1492:13
---- [ui] ui/consts/const-int-unchecked.rs stdout ----

error: /checkout/src/test/ui/consts/const-int-unchecked.rs:148: unexpected warning: '148:42: 148:78: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!'


error: /checkout/src/test/ui/consts/const-int-unchecked.rs:151: unexpected error: '151:42: 151:79: any use of this value will cause an error [const_err]'
error: /checkout/src/test/ui/consts/const-int-unchecked.rs:151: unexpected warning: '151:42: 151:79: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!'


error: /checkout/src/test/ui/consts/const-int-unchecked.rs:154: unexpected error: '154:42: 154:79: any use of this value will cause an error [const_err]'
error: /checkout/src/test/ui/consts/const-int-unchecked.rs:154: unexpected warning: '154:42: 154:79: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!'


error: /checkout/src/test/ui/consts/const-int-unchecked.rs:157: unexpected error: '157:44: 157:82: any use of this value will cause an error [const_err]'
error: /checkout/src/test/ui/consts/const-int-unchecked.rs:157: unexpected warning: '157:44: 157:82: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!'


error: /checkout/src/test/ui/consts/const-int-unchecked.rs:163: unexpected error: '163:25: 163:72: any use of this value will cause an error [const_err]'
error: /checkout/src/test/ui/consts/const-int-unchecked.rs:163: unexpected warning: '163:25: 163:72: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!'


error: /checkout/src/test/ui/consts/const-int-unchecked.rs:167: unexpected error: '167:25: 167:66: any use of this value will cause an error [const_err]'
error: /checkout/src/test/ui/consts/const-int-unchecked.rs:167: unexpected warning: '167:25: 167:66: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!'


error: /checkout/src/test/ui/consts/const-int-unchecked.rs:171: unexpected error: '171:25: 171:71: any use of this value will cause an error [const_err]'
error: /checkout/src/test/ui/consts/const-int-unchecked.rs:171: unexpected warning: '171:25: 171:71: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!'


error: /checkout/src/test/ui/consts/const-int-unchecked.rs:175: unexpected error: '175:25: 175:61: any use of this value will cause an error [const_err]'
error: /checkout/src/test/ui/consts/const-int-unchecked.rs:175: unexpected warning: '175:25: 175:61: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!'


error: /checkout/src/test/ui/consts/const-int-unchecked.rs:178: unexpected error: '178:25: 178:69: any use of this value will cause an error [const_err]'
error: /checkout/src/test/ui/consts/const-int-unchecked.rs:178: unexpected warning: '178:25: 178:69: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!'


error: /checkout/src/test/ui/consts/const-int-unchecked.rs:182: unexpected error: '182:25: 182:61: any use of this value will cause an error [const_err]'
error: /checkout/src/test/ui/consts/const-int-unchecked.rs:182: unexpected warning: '182:25: 182:61: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!'


error: /checkout/src/test/ui/consts/const-int-unchecked.rs:185: unexpected error: '185:25: 185:69: any use of this value will cause an error [const_err]'
error: /checkout/src/test/ui/consts/const-int-unchecked.rs:185: unexpected warning: '185:25: 185:69: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!'

error: 21 unexpected errors found, 0 expected errors not found
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-int-unchecked.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-unchecked" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-unchecked/auxiliary"
    Error {
        line_num: 148,
        kind: Some(
            Warning,
---
        line_num: 151,
        kind: Some(
            Error,
        ),
        msg: "151:42: 151:79: any use of this value will cause an error [const_err]",
    Error {
        line_num: 151,
        kind: Some(
            Warning,
---
        line_num: 154,
        kind: Some(
            Error,
        ),
        msg: "154:42: 154:79: any use of this value will cause an error [const_err]",
    Error {
        line_num: 154,
        kind: Some(
            Warning,
---
        line_num: 157,
        kind: Some(
            Error,
        ),
        msg: "157:44: 157:82: any use of this value will cause an error [const_err]",
    Error {
        line_num: 157,
        kind: Some(
            Warning,
---
        line_num: 163,
        kind: Some(
            Error,
        ),
        msg: "163:25: 163:72: any use of this value will cause an error [const_err]",
    Error {
        line_num: 163,
        kind: Some(
            Warning,
---
        line_num: 167,
        kind: Some(
            Error,
        ),
        msg: "167:25: 167:66: any use of this value will cause an error [const_err]",
    Error {
        line_num: 167,
        kind: Some(
            Warning,
---
        line_num: 171,
        kind: Some(
            Error,
        ),
        msg: "171:25: 171:71: any use of this value will cause an error [const_err]",
    Error {
        line_num: 171,
        kind: Some(
            Warning,
---
        line_num: 175,
        kind: Some(
            Error,
        ),
        msg: "175:25: 175:61: any use of this value will cause an error [const_err]",
    Error {
        line_num: 175,
        kind: Some(
            Warning,
---
        line_num: 178,
        kind: Some(
            Error,
        ),
        msg: "178:25: 178:69: any use of this value will cause an error [const_err]",
    Error {
        line_num: 178,
        kind: Some(
            Warning,
---
        line_num: 182,
        kind: Some(
            Error,
        ),
        msg: "182:25: 182:61: any use of this value will cause an error [const_err]",
    Error {
        line_num: 182,
        kind: Some(
            Warning,
---
        line_num: 185,
        kind: Some(
            Error,
        ),
        msg: "185:25: 185:69: any use of this value will cause an error [const_err]",
    Error {
        line_num: 185,
        kind: Some(
            Warning,
---
thread '[ui] ui/consts/const-int-unchecked.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1492:13

---- [ui] ui/deprecation/deprecation-lint.rs stdout ----

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:152: unexpected error: '152:13: 152:24: use of deprecated struct `deprecation_lint::Deprecated2`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:163: unexpected error: '163:9: 163:35: use of deprecated function `deprecation_lint::deprecated_mod::deprecated`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:246: unexpected error: '246:9: 246:19: use of deprecated function `this_crate::deprecated`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:251: unexpected error: '251:9: 251:32: use of deprecated associated function `this_crate::Trait::trait_deprecated`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:253: unexpected error: '253:9: 253:41: use of deprecated associated function `this_crate::Trait::trait_deprecated`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:255: unexpected error: '255:9: 255:24: use of deprecated function `this_crate::deprecated_text`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:260: unexpected error: '260:9: 260:37: use of deprecated associated function `this_crate::Trait::trait_deprecated_text`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:262: unexpected error: '262:9: 262:46: use of deprecated associated function `this_crate::Trait::trait_deprecated_text`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:265: unexpected error: '265:9: 265:26: use of deprecated function `this_crate::deprecated_future`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:266: unexpected error: '266:9: 266:31: use of deprecated function `this_crate::deprecated_future_text`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:268: unexpected error: '268:17: 268:33: use of deprecated struct `this_crate::DeprecatedStruct`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:273: unexpected error: '273:17: 273:37: use of deprecated unit struct `this_crate::DeprecatedUnitStruct`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:275: unexpected error: '275:17: 275:40: use of deprecated unit variant `this_crate::Enum::DeprecatedVariant`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:277: unexpected error: '277:17: 277:38: use of deprecated tuple struct `this_crate::DeprecatedTupleStruct`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:279: unexpected error: '279:17: 279:41: use of deprecated struct `this_crate::nested::DeprecatedStruct`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:284: unexpected error: '284:17: 284:45: use of deprecated unit struct `this_crate::nested::DeprecatedUnitStruct`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:286: unexpected error: '286:17: 286:48: use of deprecated unit variant `this_crate::nested::Enum::DeprecatedVariant`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:288: unexpected error: '288:17: 288:46: use of deprecated tuple struct `this_crate::nested::DeprecatedTupleStruct`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:293: unexpected error: '293:9: 293:32: use of deprecated associated function `this_crate::Trait::trait_deprecated`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:295: unexpected error: '295:9: 295:41: use of deprecated associated function `this_crate::Trait::trait_deprecated`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:297: unexpected error: '297:9: 297:37: use of deprecated associated function `this_crate::Trait::trait_deprecated_text`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:299: unexpected error: '299:9: 299:46: use of deprecated associated function `this_crate::Trait::trait_deprecated_text`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:317: unexpected error: '317:13: 317:16: use of deprecated function `this_crate::test_fn_closure_body::{closure#0}::bar` [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:336: unexpected error: '336:10: 336:25: use of deprecated trait `this_crate::DeprecatedTrait`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:338: unexpected error: '338:24: 338:39: use of deprecated trait `this_crate::DeprecatedTrait`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:390: unexpected error: '390:17: 390:27: use of deprecated struct `this_crate2::Deprecated`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:399: unexpected error: '399:13: 399:23: use of deprecated struct `this_crate2::Deprecated`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:405: unexpected error: '405:13: 405:23: use of deprecated struct `this_crate2::Deprecated`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:410: unexpected error: '410:17: 410:28: use of deprecated tuple struct `this_crate2::Deprecated2`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:420: unexpected error: '420:13: 420:24: use of deprecated tuple struct `this_crate2::Deprecated2`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:429: unexpected error: '429:13: 429:24: use of deprecated tuple struct `this_crate2::Deprecated2`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:147: unexpected error: '147:14: 147:15: use of deprecated field `deprecation_lint::Deprecated2::1`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:149: unexpected error: '149:14: 149:15: use of deprecated field `deprecation_lint::Deprecated2::2`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:247: unexpected error: '247:13: 247:30: use of deprecated associated function `this_crate::MethodTester::method_deprecated`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:248: unexpected error: '248:9: 248:31: use of deprecated associated function `this_crate::MethodTester::method_deprecated`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:249: unexpected error: '249:9: 249:33: use of deprecated associated function `this_crate::MethodTester::method_deprecated`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:250: unexpected error: '250:13: 250:29: use of deprecated associated function `this_crate::Trait::trait_deprecated`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:252: unexpected error: '252:9: 252:32: use of deprecated associated function `this_crate::Trait::trait_deprecated`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:256: unexpected error: '256:13: 256:35: use of deprecated associated function `this_crate::MethodTester::method_deprecated_text`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:257: unexpected error: '257:9: 257:36: use of deprecated associated function `this_crate::MethodTester::method_deprecated_text`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:258: unexpected error: '258:9: 258:38: use of deprecated associated function `this_crate::MethodTester::method_deprecated_text`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:259: unexpected error: '259:13: 259:34: use of deprecated associated function `this_crate::Trait::trait_deprecated_text`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:261: unexpected error: '261:9: 261:37: use of deprecated associated function `this_crate::Trait::trait_deprecated_text`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:270: unexpected error: '270:13: 270:17: use of deprecated field `this_crate::DeprecatedStruct::i`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:281: unexpected error: '281:13: 281:17: use of deprecated field `this_crate::nested::DeprecatedStruct::i`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:292: unexpected error: '292:13: 292:29: use of deprecated associated function `this_crate::Trait::trait_deprecated`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:294: unexpected error: '294:9: 294:32: use of deprecated associated function `this_crate::Trait::trait_deprecated`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:296: unexpected error: '296:13: 296:34: use of deprecated associated function `this_crate::Trait::trait_deprecated_text`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:298: unexpected error: '298:9: 298:37: use of deprecated associated function `this_crate::Trait::trait_deprecated_text`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:303: unexpected error: '303:13: 303:29: use of deprecated associated function `this_crate::Trait::trait_deprecated`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:304: unexpected error: '304:13: 304:34: use of deprecated associated function `this_crate::Trait::trait_deprecated_text`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:363: unexpected error: '363:13: 363:25: use of deprecated field `this_crate2::Stable::override2`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:367: unexpected error: '367:17: 367:28: use of deprecated field `this_crate2::Stable::override2`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:371: unexpected error: '371:13: 371:25: use of deprecated field `this_crate2::Stable::override2`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:379: unexpected error: '379:17: 379:20: use of deprecated field `this_crate2::Stable2::2`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:384: unexpected error: '384:20: 384:21: use of deprecated field `this_crate2::Stable2::2`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:392: unexpected error: '392:13: 392:23: use of deprecated field `this_crate2::Deprecated::inherit`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:396: unexpected error: '396:17: 396:26: use of deprecated field `this_crate2::Deprecated::inherit`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:401: unexpected error: '401:13: 401:23: use of deprecated field `this_crate2::Deprecated::inherit`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:413: unexpected error: '413:17: 413:20: use of deprecated field `this_crate2::Deprecated2::0`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:415: unexpected error: '415:17: 415:20: use of deprecated field `this_crate2::Deprecated2::1`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:417: unexpected error: '417:17: 417:20: use of deprecated field `this_crate2::Deprecated2::2`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:422: unexpected error: '422:14: 422:15: use of deprecated field `this_crate2::Deprecated2::0`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:424: unexpected error: '424:14: 424:15: use of deprecated field `this_crate2::Deprecated2::1`: text [deprecated]'

error: /checkout/src/test/ui/deprecation/deprecation-lint.rs:426: unexpected error: '426:14: 426:15: use of deprecated field `this_crate2::Deprecated2::2`: text [deprecated]'
error: 65 unexpected errors found, 0 expected errors not found
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/deprecation/deprecation-lint.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deprecation/deprecation-lint" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deprecation/deprecation-lint/auxiliary"
    Error {
        line_num: 152,
        kind: Some(
            Error,
            Error,
        ),
        msg: "152:13: 152:24: use of deprecated struct `deprecation_lint::Deprecated2`: text [deprecated]",
    Error {
        line_num: 163,
        kind: Some(
            Error,
            Error,
        ),
        msg: "163:9: 163:35: use of deprecated function `deprecation_lint::deprecated_mod::deprecated`: text [deprecated]",
    Error {
        line_num: 246,
        kind: Some(
            Error,
            Error,
        ),
        msg: "246:9: 246:19: use of deprecated function `this_crate::deprecated`: text [deprecated]",
    Error {
        line_num: 251,
        kind: Some(
            Error,
            Error,
        ),
        msg: "251:9: 251:32: use of deprecated associated function `this_crate::Trait::trait_deprecated`: text [deprecated]",
    Error {
        line_num: 253,
        kind: Some(
            Error,
            Error,
        ),
        msg: "253:9: 253:41: use of deprecated associated function `this_crate::Trait::trait_deprecated`: text [deprecated]",
    Error {
        line_num: 255,
        kind: Some(
            Error,
            Error,
        ),
        msg: "255:9: 255:24: use of deprecated function `this_crate::deprecated_text`: text [deprecated]",
    Error {
        line_num: 260,
        kind: Some(
            Error,
            Error,
        ),
        msg: "260:9: 260:37: use of deprecated associated function `this_crate::Trait::trait_deprecated_text`: text [deprecated]",
    Error {
        line_num: 262,
        kind: Some(
            Error,
            Error,
        ),
        msg: "262:9: 262:46: use of deprecated associated function `this_crate::Trait::trait_deprecated_text`: text [deprecated]",
    Error {
        line_num: 265,
        kind: Some(
            Error,
            Error,
        ),
        msg: "265:9: 265:26: use of deprecated function `this_crate::deprecated_future`: text [deprecated]",
    Error {
        line_num: 266,
        kind: Some(
            Error,
            Error,
        ),
        msg: "266:9: 266:31: use of deprecated function `this_crate::deprecated_future_text`: text [deprecated]",
    Error {
        line_num: 268,
        kind: Some(
            Error,
            Error,
        ),
        msg: "268:17: 268:33: use of deprecated struct `this_crate::DeprecatedStruct`: text [deprecated]",
    Error {
        line_num: 273,
        kind: Some(
            Error,
            Error,
        ),
        msg: "273:17: 273:37: use of deprecated unit struct `this_crate::DeprecatedUnitStruct`: text [deprecated]",
    Error {
        line_num: 275,
        kind: Some(
            Error,
            Error,
        ),
        msg: "275:17: 275:40: use of deprecated unit variant `this_crate::Enum::DeprecatedVariant`: text [deprecated]",
    Error {
        line_num: 277,
        kind: Some(
            Error,
            Error,
        ),
        msg: "277:17: 277:38: use of deprecated tuple struct `this_crate::DeprecatedTupleStruct`: text [deprecated]",
    Error {
        line_num: 279,
        kind: Some(
            Error,
            Error,
        ),
        msg: "279:17: 279:41: use of deprecated struct `this_crate::nested::DeprecatedStruct`: text [deprecated]",
    Error {
        line_num: 284,
        kind: Some(
            Error,
            Error,
        ),
        msg: "284:17: 284:45: use of deprecated unit struct `this_crate::nested::DeprecatedUnitStruct`: text [deprecated]",
    Error {
        line_num: 286,
        kind: Some(
            Error,
            Error,
        ),
        msg: "286:17: 286:48: use of deprecated unit variant `this_crate::nested::Enum::DeprecatedVariant`: text [deprecated]",
    Error {
        line_num: 288,
        kind: Some(
            Error,
            Error,
        ),
        msg: "288:17: 288:46: use of deprecated tuple struct `this_crate::nested::DeprecatedTupleStruct`: text [deprecated]",
    Error {
        line_num: 293,
        kind: Some(
            Error,
            Error,
        ),
        msg: "293:9: 293:32: use of deprecated associated function `this_crate::Trait::trait_deprecated`: text [deprecated]",
    Error {
        line_num: 295,
        kind: Some(
            Error,
            Error,
        ),
        msg: "295:9: 295:41: use of deprecated associated function `this_crate::Trait::trait_deprecated`: text [deprecated]",
    Error {
        line_num: 297,
        kind: Some(
            Error,
            Error,
        ),
        msg: "297:9: 297:37: use of deprecated associated function `this_crate::Trait::trait_deprecated_text`: text [deprecated]",
    Error {
        line_num: 299,
        kind: Some(
            Error,
            Error,
        ),
        msg: "299:9: 299:46: use of deprecated associated function `this_crate::Trait::trait_deprecated_text`: text [deprecated]",
    Error {
        line_num: 317,
        kind: Some(
            Error,
            Error,
        ),
        msg: "317:13: 317:16: use of deprecated function `this_crate::test_fn_closure_body::{closure#0}::bar` [deprecated]",
    Error {
        line_num: 336,
        kind: Some(
            Error,
            Error,
        ),
        msg: "336:10: 336:25: use of deprecated trait `this_crate::DeprecatedTrait`: text [deprecated]",
    Error {
        line_num: 338,
        kind: Some(
            Error,
            Error,
        ),
        msg: "338:24: 338:39: use of deprecated trait `this_crate::DeprecatedTrait`: text [deprecated]",
    Error {
        line_num: 390,
        kind: Some(
            Error,
            Error,
        ),
        msg: "390:17: 390:27: use of deprecated struct `this_crate2::Deprecated`: text [deprecated]",
    Error {
        line_num: 399,
        kind: Some(
            Error,
            Error,
        ),
        msg: "399:13: 399:23: use of deprecated struct `this_crate2::Deprecated`: text [deprecated]",
    Error {
        line_num: 405,
        kind: Some(
            Error,
            Error,
        ),
        msg: "405:13: 405:23: use of deprecated struct `this_crate2::Deprecated`: text [deprecated]",
    Error {
        line_num: 410,
        kind: Some(
            Error,
            Error,
        ),
        msg: "410:17: 410:28: use of deprecated tuple struct `this_crate2::Deprecated2`: text [deprecated]",
    Error {
        line_num: 420,
        kind: Some(
            Error,
            Error,
        ),
        msg: "420:13: 420:24: use of deprecated tuple struct `this_crate2::Deprecated2`: text [deprecated]",
    Error {
        line_num: 429,
        kind: Some(
            Error,
            Error,
        ),
        msg: "429:13: 429:24: use of deprecated tuple struct `this_crate2::Deprecated2`: text [deprecated]",
    Error {
        line_num: 147,
        kind: Some(
            Error,
            Error,
        ),
        msg: "147:14: 147:15: use of deprecated field `deprecation_lint::Deprecated2::1`: text [deprecated]",
    Error {
        line_num: 149,
        kind: Some(
            Error,
            Error,
        ),
        msg: "149:14: 149:15: use of deprecated field `deprecation_lint::Deprecated2::2`: text [deprecated]",
    Error {
        line_num: 247,
        kind: Some(
            Error,
            Error,
        ),
        msg: "247:13: 247:30: use of deprecated associated function `this_crate::MethodTester::method_deprecated`: text [deprecated]",
    Error {
        line_num: 248,
        kind: Some(
            Error,
            Error,
        ),
        msg: "248:9: 248:31: use of deprecated associated function `this_crate::MethodTester::method_deprecated`: text [deprecated]",
    Error {
        line_num: 249,
        kind: Some(
            Error,
            Error,
        ),
        msg: "249:9: 249:33: use of deprecated associated function `this_crate::MethodTester::method_deprecated`: text [deprecated]",
    Error {
        line_num: 250,
        kind: Some(
            Error,
            Error,
        ),
        msg: "250:13: 250:29: use of deprecated associated function `this_crate::Trait::trait_deprecated`: text [deprecated]",
    Error {
        line_num: 252,
        kind: Some(
            Error,
            Error,
        ),
        msg: "252:9: 252:32: use of deprecated associated function `this_crate::Trait::trait_deprecated`: text [deprecated]",
    Error {
        line_num: 256,
        kind: Some(
            Error,
            Error,
        ),
        msg: "256:13: 256:35: use of deprecated associated function `this_crate::MethodTester::method_deprecated_text`: text [deprecated]",
    Error {
        line_num: 257,
        kind: Some(
            Error,
            Error,
        ),
        msg: "257:9: 257:36: use of deprecated associated function `this_crate::MethodTester::method_deprecated_text`: text [deprecated]",
    Error {
        line_num: 258,
        kind: Some(
            Error,
            Error,
        ),
        msg: "258:9: 258:38: use of deprecated associated function `this_crate::MethodTester::method_deprecated_text`: text [deprecated]",
    Error {
        line_num: 259,
        kind: Some(
            Error,
            Error,
        ),
        msg: "259:13: 259:34: use of deprecated associated function `this_crate::Trait::trait_deprecated_text`: text [deprecated]",
    Error {
        line_num: 261,
        kind: Some(
            Error,
            Error,
        ),
        msg: "261:9: 261:37: use of deprecated associated function `this_crate::Trait::trait_deprecated_text`: text [deprecated]",
    Error {
        line_num: 270,
        kind: Some(
            Error,
            Error,
        ),
        msg: "270:13: 270:17: use of deprecated field `this_crate::DeprecatedStruct::i`: text [deprecated]",
    Error {
        line_num: 281,
        kind: Some(
            Error,
            Error,
        ),
        msg: "281:13: 281:17: use of deprecated field `this_crate::nested::DeprecatedStruct::i`: text [deprecated]",
    Error {
        line_num: 292,
        kind: Some(
            Error,
            Error,
        ),
        msg: "292:13: 292:29: use of deprecated associated function `this_crate::Trait::trait_deprecated`: text [deprecated]",
    Error {
        line_num: 294,
        kind: Some(
            Error,
            Error,
        ),
        msg: "294:9: 294:32: use of deprecated associated function `this_crate::Trait::trait_deprecated`: text [deprecated]",
    Error {
        line_num: 296,
        kind: Some(
            Error,
            Error,
        ),
        msg: "296:13: 296:34: use of deprecated associated function `this_crate::Trait::trait_deprecated_text`: text [deprecated]",
    Error {
        line_num: 298,
        kind: Some(
            Error,
            Error,
        ),
        msg: "298:9: 298:37: use of deprecated associated function `this_crate::Trait::trait_deprecated_text`: text [deprecated]",
    Error {
        line_num: 303,
        kind: Some(
            Error,
            Error,
        ),
        msg: "303:13: 303:29: use of deprecated associated function `this_crate::Trait::trait_deprecated`: text [deprecated]",
    Error {
        line_num: 304,
        kind: Some(
            Error,
            Error,
        ),
        msg: "304:13: 304:34: use of deprecated associated function `this_crate::Trait::trait_deprecated_text`: text [deprecated]",
    Error {
        line_num: 363,
        kind: Some(
            Error,
            Error,
        ),
---
        line_num: 500,
        kind: Some(
            Warning,
        ),
        msg: "500:1: 500:8: attribute should be applied to a function [unused_attributes]",
    Error {
        line_num: 503,
        kind: Some(
            Note,
---
        line_num: 529,
        kind: Some(
            Warning,
        ),
        msg: "529:1: 529:22: attribute should be applied to a foreign function or static [unused_attributes]",
    Error {
        line_num: 532,
        kind: Some(
            Note,
---
        line_num: 568,
        kind: Some(
            Warning,
        ),
        msg: "568:1: 568:25: attribute should be applied to a function or static [unused_attributes]",
    Error {
        line_num: 571,
        kind: Some(
            Note,
---
        line_num: 338,
        kind: Some(
            Warning,
        ),
        msg: "338:17: 338:30: attribute should be applied to a function or static [unused_attributes]",
    Error {
        line_num: 338,
        kind: Some(
            Note,
---
        line_num: 345,
        kind: Some(
            Warning,
        ),
        msg: "345:5: 345:17: attribute should be applied to a function or static [unused_attributes]",
    Error {
        line_num: 345,
        kind: Some(
            Note,
---
        line_num: 350,
        kind: Some(
            Warning,
        ),
        msg: "350:5: 350:17: attribute should be applied to a function or static [unused_attributes]",
    Error {
        line_num: 350,
        kind: Some(
            Note,
---
        line_num: 355,
        kind: Some(
            Warning,
        ),
        msg: "355:5: 355:17: attribute should be applied to a function or static [unused_attributes]",
    Error {
        line_num: 355,
        kind: Some(
            Note,
---
        line_num: 506,
        kind: Some(
            Warning,
        ),
        msg: "506:17: 506:25: attribute should be applied to a function [unused_attributes]",
    Error {
        line_num: 506,
        kind: Some(
            Note,
---
        line_num: 513,
        kind: Some(
            Warning,
        ),
        msg: "513:5: 513:12: attribute should be applied to a function [unused_attributes]",
    Error {
        line_num: 513,
        kind: Some(
            Note,
---
        line_num: 518,
        kind: Some(
            Warning,
        ),
        msg: "518:5: 518:12: attribute should be applied to a function [unused_attributes]",
    Error {
        line_num: 518,
        kind: Some(
            Note,
---
        line_num: 523,
        kind: Some(
            Warning,
        ),
        msg: "523:5: 523:12: attribute should be applied to a function [unused_attributes]",
    Error {
        line_num: 523,
        kind: Some(
            Note,
---
        line_num: 535,
        kind: Some(
            Warning,
        ),
        msg: "535:5: 535:26: attribute should be applied to a foreign function or static [unused_attributes]",
    Error {
        line_num: 539,
        kind: Some(
            Note,
---
        line_num: 535,
        kind: Some(
            Help,
        ),
        msg: "535:5: 535:26: try `#[link(name = \"1900\")]` instead",
    Error {
        line_num: 542,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "542:17: 542:37: attribute should be applied to a foreign function or static [unused_attributes]",
    Error {
        line_num: 542,
        kind: Some(
            Note,
---
        line_num: 547,
        kind: Some(
            Warning,
        ),
        msg: "547:5: 547:26: attribute should be applied to a foreign function or static [unused_attributes]",
    Error {
        line_num: 547,
        kind: Some(
            Note,
---
        line_num: 552,
        kind: Some(
            Warning,
        ),
        msg: "552:5: 552:26: attribute should be applied to a foreign function or static [unused_attributes]",
    Error {
        line_num: 552,
        kind: Some(
            Note,
---
        line_num: 557,
        kind: Some(
            Warning,
        ),
        msg: "557:5: 557:26: attribute should be applied to a foreign function or static [unused_attributes]",
    Error {
        line_num: 557,
        kind: Some(
            Note,
---
        line_num: 562,
        kind: Some(
            Warning,
        ),
        msg: "562:5: 562:26: attribute should be applied to a foreign function or static [unused_attributes]",
    Error {
        line_num: 562,
        kind: Some(
            Note,
---
        line_num: 574,
        kind: Some(
            Warning,
        ),
        msg: "574:17: 574:40: attribute should be applied to a function or static [unused_attributes]",
    Error {
        line_num: 574,
        kind: Some(
            Note,
---
        line_num: 581,
        kind: Some(
            Warning,
        ),
        msg: "581:5: 581:29: attribute should be applied to a function or static [unused_attributes]",
    Error {
        line_num: 581,
        kind: Some(
            Note,
---
        line_num: 586,
        kind: Some(
            Warning,
        ),
        msg: "586:5: 586:29: attribute should be applied to a function or static [unused_attributes]",
    Error {
        line_num: 586,
        kind: Some(
            Note,
---
        line_num: 591,
        kind: Some(
            Warning,
        ),
        msg: "591:5: 591:29: attribute should be applied to a function or static [unused_attributes]",
    Error {
        line_num: 591,
        kind: Some(
            Note,
---
        line_num: 310,
        kind: Some(
            Warning,
        ),
        msg: "310:5: 310:21: unused attribute [unused_attributes]",
    Error {
        line_num: 314,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "314:1: 314:25: unused attribute [unused_attributes]",
    Error {
        line_num: 317,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "317:17: 317:42: unused attribute [unused_attributes]",
    Error {
        line_num: 320,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "320:5: 320:29: unused attribute [unused_attributes]",
    Error {
        line_num: 323,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "323:5: 323:29: unused attribute [unused_attributes]",
    Error {
        line_num: 326,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "326:5: 326:29: unused attribute [unused_attributes]",
    Error {
        line_num: 329,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "329:5: 329:29: unused attribute [unused_attributes]",
    Error {
        line_num: 361,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "361:1: 361:16: unused attribute [unused_attributes]",
    Error {
        line_num: 364,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "364:17: 364:33: unused attribute [unused_attributes]",
    Error {
        line_num: 367,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "367:5: 367:20: unused attribute [unused_attributes]",
    Error {
        line_num: 370,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "370:5: 370:20: unused attribute [unused_attributes]",
    Error {
        line_num: 373,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "373:5: 373:20: unused attribute [unused_attributes]",
    Error {
        line_num: 376,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "376:5: 376:20: unused attribute [unused_attributes]",
    Error {
        line_num: 380,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "380:1: 380:10: unused attribute [unused_attributes]",
    Error {
        line_num: 383,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "383:17: 383:27: unused attribute [unused_attributes]",
    Error {
        line_num: 386,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "386:5: 386:14: unused attribute [unused_attributes]",
    Error {
        line_num: 389,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "389:5: 389:14: unused attribute [unused_attributes]",
    Error {
        line_num: 392,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "392:5: 392:14: unused attribute [unused_attributes]",
    Error {
        line_num: 395,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "395:5: 395:14: unused attribute [unused_attributes]",
    Error {
        line_num: 399,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "399:1: 399:23: unused attribute [unused_attributes]",
    Error {
        line_num: 402,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "402:17: 402:40: unused attribute [unused_attributes]",
    Error {
        line_num: 405,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "405:5: 405:27: unused attribute [unused_attributes]",
    Error {
        line_num: 408,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "408:5: 408:27: unused attribute [unused_attributes]",
    Error {
        line_num: 411,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "411:5: 411:27: unused attribute [unused_attributes]",
    Error {
        line_num: 414,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "414:5: 414:27: unused attribute [unused_attributes]",
    Error {
        line_num: 418,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "418:1: 418:39: unused attribute [unused_attributes]",
    Error {
        line_num: 421,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "421:17: 421:54: unused attribute [unused_attributes]",
    Error {
        line_num: 424,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "424:5: 424:43: unused attribute [unused_attributes]",
    Error {
        line_num: 427,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "427:5: 427:43: unused attribute [unused_attributes]",
    Error {
        line_num: 430,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "430:5: 430:43: unused attribute [unused_attributes]",
    Error {
        line_num: 433,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "433:5: 433:43: unused attribute [unused_attributes]",
    Error {
        line_num: 445,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "445:5: 445:20: unused attribute [unused_attributes]",
    Error {
        line_num: 448,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "448:5: 448:20: unused attribute [unused_attributes]",
    Error {
        line_num: 451,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "451:5: 451:20: unused attribute [unused_attributes]",
    Error {
        line_num: 454,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "454:5: 454:20: unused attribute [unused_attributes]",
    Error {
        line_num: 458,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "458:1: 458:10: unused attribute [unused_attributes]",
    Error {
        line_num: 458,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "458:1: 458:10: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]` [unused_attributes]",
    Error {
        line_num: 462,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "462:17: 462:27: unused attribute [unused_attributes]",
    Error {
        line_num: 462,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "462:17: 462:27: crate-level attribute should be in the root module [unused_attributes]",
    Error {
        line_num: 466,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "466:5: 466:14: unused attribute [unused_attributes]",
    Error {
        line_num: 466,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "466:5: 466:14: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]` [unused_attributes]",
    Error {
        line_num: 470,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "470:5: 470:14: unused attribute [unused_attributes]",
    Error {
        line_num: 470,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "470:5: 470:14: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]` [unused_attributes]",
    Error {
        line_num: 474,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "474:5: 474:14: unused attribute [unused_attributes]",
    Error {
        line_num: 474,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "474:5: 474:14: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]` [unused_attributes]",
    Error {
        line_num: 478,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "478:5: 478:14: unused attribute [unused_attributes]",
    Error {
        line_num: 478,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "478:5: 478:14: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]` [unused_attributes]",
    Error {
        line_num: 659,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "659:1: 659:23: unused attribute [unused_attributes]",
    Error {
        line_num: 659,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "659:1: 659:23: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]` [unused_attributes]",
    Error {
        line_num: 663,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "663:17: 663:38: unused attribute [unused_attributes]",
    Error {
        line_num: 663,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "663:17: 663:38: crate-level attribute should be in the root module [unused_attributes]",
    Error {
        line_num: 667,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "667:5: 667:27: unused attribute [unused_attributes]",
    Error {
        line_num: 667,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "667:5: 667:27: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]` [unused_attributes]",
    Error {
        line_num: 671,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "671:5: 671:27: unused attribute [unused_attributes]",
    Error {
        line_num: 671,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "671:5: 671:27: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]` [unused_attributes]",
    Error {
        line_num: 675,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "675:5: 675:27: unused attribute [unused_attributes]",
    Error {
        line_num: 675,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "675:5: 675:27: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]` [unused_attributes]",
    Error {
        line_num: 679,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "679:5: 679:27: unused attribute [unused_attributes]",
    Error {
        line_num: 679,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "679:5: 679:27: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]` [unused_attributes]",
    Error {
        line_num: 684,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "684:1: 684:23: unused attribute [unused_attributes]",
    Error {
        line_num: 684,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "684:1: 684:23: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]` [unused_attributes]",
    Error {
        line_num: 688,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "688:17: 688:38: unused attribute [unused_attributes]",
    Error {
        line_num: 688,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "688:17: 688:38: crate-level attribute should be in the root module [unused_attributes]",
    Error {
        line_num: 692,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "692:5: 692:27: unused attribute [unused_attributes]",
    Error {
        line_num: 692,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "692:5: 692:27: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]` [unused_attributes]",
    Error {
        line_num: 696,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "696:5: 696:27: unused attribute [unused_attributes]",
    Error {
        line_num: 696,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "696:5: 696:27: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]` [unused_attributes]",
    Error {
        line_num: 700,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "700:5: 700:27: unused attribute [unused_attributes]",
    Error {
        line_num: 700,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "700:5: 700:27: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]` [unused_attributes]",
    Error {
        line_num: 704,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "704:5: 704:27: unused attribute [unused_attributes]",
    Error {
        line_num: 704,
        kind: Some(
            Warning,
            Warning,
        ),
        msg: "704:5: 704:27: crate-level attribute should be an inner attribute: add an exclamation mark: `#![foo]` [unused_attributes]",
    Error {
        line_num: 709,
        kind: Some(
            Warning,
            Warning,
---
test result: FAILED. 11565 passed; 19 failed; 96 ignored; 0 measured; 0 filtered out; finished in 137.04s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:15
