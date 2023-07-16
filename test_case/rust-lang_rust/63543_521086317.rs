plain
2019-08-14T02:00:51.6427676Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-14T02:00:51.6621875Z ##[command]git config gc.auto 0
2019-08-14T02:00:51.6692485Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-14T02:00:51.6744631Z ##[command]git config --get-all http.proxy
2019-08-14T02:00:51.6915611Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63543/merge:refs/remotes/pull/63543/merge
---
2019-08-14T02:01:27.9339723Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-14T02:01:27.9340067Z 
2019-08-14T02:01:27.9340656Z   git checkout -b <new-branch-name>
2019-08-14T02:01:27.9341034Z 
2019-08-14T02:01:27.9341339Z HEAD is now at cc47b00c2 Merge 3672869691e9944d61559b697146f9d16651adab into 60960a260f7b5c695fd0717311d72ce62dd4eb43
2019-08-14T02:01:27.9505133Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-14T02:01:27.9508459Z ==============================================================================
2019-08-14T02:01:27.9508519Z Task         : Bash
2019-08-14T02:01:27.9508583Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-14T02:10:43.6579033Z    Compiling rand_chacha v0.1.0
2019-08-14T02:10:43.8280949Z    Compiling rand v0.6.1
2019-08-14T02:10:46.5837418Z     Checking tempfile v3.0.5
2019-08-14T02:10:47.1291831Z     Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
2019-08-14T02:10:47.4798260Z error[E0252]: the name `Variant` is defined multiple times
2019-08-14T02:10:47.4798769Z     --> src/librustdoc/html/render.rs:3756:41
2019-08-14T02:10:47.4799047Z      |
2019-08-14T02:10:47.4804516Z 3756 |             use crate::clean::{Variant, Variant};
2019-08-14T02:10:47.4805179Z      |                                -------  ^^^^^^^ `Variant` reimported here
2019-08-14T02:10:47.4805555Z      |                                |
2019-08-14T02:10:47.4805932Z      |                                previous import of the type `Variant` here
2019-08-14T02:10:47.4806194Z      |
2019-08-14T02:10:47.4806568Z      = note: `Variant` must be defined only once in the type namespace of this block
2019-08-14T02:10:47.4806624Z 
2019-08-14T02:10:47.7096873Z error: unused import: `Variant`
2019-08-14T02:10:47.7097310Z     --> src/librustdoc/html/render.rs:3756:41
2019-08-14T02:10:47.7097616Z      |
2019-08-14T02:10:47.7098200Z 3756 |             use crate::clean::{Variant, Variant};
2019-08-14T02:10:47.7098881Z      |
2019-08-14T02:10:47.7099199Z      = note: `-D unused-imports` implied by `-D warnings`
2019-08-14T02:10:47.7099243Z 
2019-08-14T02:10:47.7099243Z 
2019-08-14T02:10:47.7099548Z error: the item `Variant` is imported redundantly
2019-08-14T02:10:47.7100107Z     --> src/librustdoc/html/render.rs:3756:41
2019-08-14T02:10:47.7100355Z      |
2019-08-14T02:10:47.7100703Z 3756 |             use crate::clean::{Variant, Variant};
2019-08-14T02:10:47.7101080Z      |                                -------  ^^^^^^^
2019-08-14T02:10:47.7101403Z      |                                |
2019-08-14T02:10:47.7101786Z      |                                the item `Variant` is already imported here
2019-08-14T02:10:47.7101831Z 
2019-08-14T02:10:49.2074195Z error[E0599]: no associated item named `Struct` found for type `clean::Variant` in the current scope
2019-08-14T02:10:49.2075530Z     --> src/librustdoc/fold.rs:61:30
2019-08-14T02:10:49.2076200Z      |
2019-08-14T02:10:49.2076879Z 61   |                     Variant::Struct(mut j) => {
2019-08-14T02:10:49.2077664Z      |                              ^^^^^^ associated item not found in `clean::Variant`
2019-08-14T02:10:49.2078918Z     ::: src/librustdoc/clean/mod.rs:3328:1
2019-08-14T02:10:49.2079502Z      |
2019-08-14T02:10:49.2080812Z 3328 | pub struct Variant {
2019-08-14T02:10:49.2080812Z 3328 | pub struct Variant {
2019-08-14T02:10:49.2081637Z      | ------------------ associated item `Struct` not found for this
2019-08-14T02:10:49.2081971Z 
2019-08-14T02:10:49.2137385Z error[E0599]: no associated item named `Struct` found for type `clean::Variant` in the current scope
2019-08-14T02:10:49.2138184Z     --> src/librustdoc/fold.rs:66:61
2019-08-14T02:10:49.2138842Z      |
2019-08-14T02:10:49.2139575Z 66   |                         VariantItem(Variant {kind: Variant::Struct(j), ..i2})
2019-08-14T02:10:49.2140828Z      |                                                             ^^^^^^ associated item not found in `clean::Variant`
2019-08-14T02:10:49.2142140Z     ::: src/librustdoc/clean/mod.rs:3328:1
2019-08-14T02:10:49.2142759Z      |
2019-08-14T02:10:49.2143464Z 3328 | pub struct Variant {
2019-08-14T02:10:49.2143464Z 3328 | pub struct Variant {
2019-08-14T02:10:49.2144191Z      | ------------------ associated item `Struct` not found for this
2019-08-14T02:10:49.2144499Z 
2019-08-14T02:10:50.0621403Z error[E0599]: no associated item named `CLike` found for type `clean::Variant` in the current scope
2019-08-14T02:10:50.0622821Z     --> src/librustdoc/html/render.rs:3686:45
2019-08-14T02:10:50.0623516Z      |
2019-08-14T02:10:50.0624296Z 3686 |                             clean::Variant::CLike => write!(w, "{}", name)?,
2019-08-14T02:10:50.0625092Z      |                                             ^^^^^ associated item not found in `clean::Variant`
2019-08-14T02:10:50.0626600Z     ::: src/librustdoc/clean/mod.rs:3328:1
2019-08-14T02:10:50.0627193Z      |
2019-08-14T02:10:50.0627858Z 3328 | pub struct Variant {
2019-08-14T02:10:50.0627858Z 3328 | pub struct Variant {
2019-08-14T02:10:50.0628577Z      | ------------------ associated item `CLike` not found for this
2019-08-14T02:10:50.0628894Z 
2019-08-14T02:10:50.0678757Z error[E0599]: no associated item named `Tuple` found for type `clean::Variant` in the current scope
2019-08-14T02:10:50.0679578Z     --> src/librustdoc/html/render.rs:3687:45
2019-08-14T02:10:50.0680282Z      |
2019-08-14T02:10:50.0681171Z 3687 |                             clean::Variant::Tuple(ref tys) => {
2019-08-14T02:10:50.0681966Z      |                                             ^^^^^ associated item not found in `clean::Variant`
2019-08-14T02:10:50.0683241Z     ::: src/librustdoc/clean/mod.rs:3328:1
2019-08-14T02:10:50.0683858Z      |
2019-08-14T02:10:50.0684503Z 3328 | pub struct Variant {
2019-08-14T02:10:50.0684503Z 3328 | pub struct Variant {
2019-08-14T02:10:50.0685198Z      | ------------------ associated item `Tuple` not found for this
2019-08-14T02:10:50.0685538Z 
2019-08-14T02:10:50.0741284Z error[E0599]: no associated item named `Struct` found for type `clean::Variant` in the current scope
2019-08-14T02:10:50.0742876Z     --> src/librustdoc/html/render.rs:3697:45
2019-08-14T02:10:50.0743463Z      |
2019-08-14T02:10:50.0744078Z 3697 |                             clean::Variant::Struct(ref s) => {
2019-08-14T02:10:50.0744725Z      |                                             ^^^^^^ associated item not found in `clean::Variant`
2019-08-14T02:10:50.0745719Z     ::: src/librustdoc/clean/mod.rs:3328:1
2019-08-14T02:10:50.0746206Z      |
2019-08-14T02:10:50.0746706Z 3328 | pub struct Variant {
2019-08-14T02:10:50.0746706Z 3328 | pub struct Variant {
2019-08-14T02:10:50.0747274Z      | ------------------ associated item `Struct` not found for this
2019-08-14T02:10:50.0747500Z 
2019-08-14T02:10:50.0971892Z error[E0599]: no associated item named `Tuple` found for type `clean::Variant` in the current scope
2019-08-14T02:10:50.0972553Z     --> src/librustdoc/html/render.rs:3741:40
2019-08-14T02:10:50.0973259Z      |
2019-08-14T02:10:50.0974143Z 3741 |                 if let clean::Variant::Tuple(ref tys) = var.kind {
2019-08-14T02:10:50.0974843Z      |                                        ^^^^^ associated item not found in `clean::Variant`
2019-08-14T02:10:50.0975807Z     ::: src/librustdoc/clean/mod.rs:3328:1
2019-08-14T02:10:50.0976484Z      |
2019-08-14T02:10:50.0976978Z 3328 | pub struct Variant {
2019-08-14T02:10:50.0976978Z 3328 | pub struct Variant {
2019-08-14T02:10:50.0977570Z      | ------------------ associated item `Tuple` not found for this
2019-08-14T02:10:50.0977761Z 
2019-08-14T02:10:50.1097339Z error[E0599]: no associated item named `Struct` found for type `clean::Variant` in the current scope
2019-08-14T02:10:50.1098051Z     --> src/librustdoc/html/render.rs:3758:32
2019-08-14T02:10:50.1098550Z      |
2019-08-14T02:10:50.1099093Z 3758 |                 kind: Variant::Struct(ref s)
2019-08-14T02:10:50.1099742Z      |                                ^^^^^^ associated item not found in `clean::Variant`
2019-08-14T02:10:50.1100937Z     ::: src/librustdoc/clean/mod.rs:3328:1
2019-08-14T02:10:50.1101404Z      |
2019-08-14T02:10:50.1101892Z 3328 | pub struct Variant {
2019-08-14T02:10:50.1101892Z 3328 | pub struct Variant {
2019-08-14T02:10:50.1102500Z      | ------------------ associated item `Struct` not found for this
2019-08-14T02:10:50.1102695Z 
2019-08-14T02:10:50.3971492Z error[E0599]: no associated item named `Struct` found for type `clean::Variant` in the current scope
2019-08-14T02:10:50.3972756Z     --> src/librustdoc/passes/mod.rs:233:39
2019-08-14T02:10:50.3973342Z      |
2019-08-14T02:10:50.3974538Z 233  |                 kind: clean::Variant::Struct(..),
2019-08-14T02:10:50.3975390Z      |                                       ^^^^^^ associated item not found in `clean::Variant`
2019-08-14T02:10:50.3977107Z     ::: src/librustdoc/clean/mod.rs:3328:1
2019-08-14T02:10:50.3977394Z      |
2019-08-14T02:10:50.3977725Z 3328 | pub struct Variant {
2019-08-14T02:10:50.3977725Z 3328 | pub struct Variant {
2019-08-14T02:10:50.3978156Z      | ------------------ associated item `Struct` not found for this
2019-08-14T02:10:50.3978209Z 
2019-08-14T02:10:50.5136685Z error[E0560]: struct `doctree::Variant<'_>` has no field named `stab`
2019-08-14T02:10:50.5137177Z    --> src/librustdoc/visit_ast.rs:136:17
2019-08-14T02:10:50.5137483Z     |
2019-08-14T02:10:50.5137840Z 136 |                 stab: self.stability(v.id),
2019-08-14T02:10:50.5138465Z     |                 ^^^^ `doctree::Variant<'_>` does not have this field
2019-08-14T02:10:50.5138828Z     |
2019-08-14T02:10:50.5139233Z     = note: available fields are: `name`, `id`, `attrs`, `def`, `whence`
2019-08-14T02:10:50.5139287Z 
2019-08-14T02:10:50.5205663Z error[E0599]: no method named `stability` found for type `&mut visit_ast::RustdocVisitor<'a, 'tcx>` in the current scope
2019-08-14T02:10:50.5206092Z    --> src/librustdoc/visit_ast.rs:136:28
2019-08-14T02:10:50.5206642Z     |
2019-08-14T02:10:50.5207009Z 136 |                 stab: self.stability(v.id),
2019-08-14T02:10:50.5207440Z 
2019-08-14T02:10:50.5207440Z 
2019-08-14T02:10:50.5207801Z error[E0560]: struct `doctree::Variant<'_>` has no field named `depr`
2019-08-14T02:10:50.5208134Z    --> src/librustdoc/visit_ast.rs:137:17
2019-08-14T02:10:50.5208447Z     |
2019-08-14T02:10:50.5208808Z 137 |                 depr: self.deprecation(v.id),
2019-08-14T02:10:50.5209233Z     |                 ^^^^ `doctree::Variant<'_>` does not have this field
2019-08-14T02:10:50.5209564Z     |
2019-08-14T02:10:50.5209930Z     = note: available fields are: `name`, `id`, `attrs`, `def`, `whence`
2019-08-14T02:10:50.5209976Z 
2019-08-14T02:10:50.5265700Z error[E0599]: no method named `deprecation` found for type `&mut visit_ast::RustdocVisitor<'a, 'tcx>` in the current scope
2019-08-14T02:10:50.5266116Z    --> src/librustdoc/visit_ast.rs:137:28
2019-08-14T02:10:50.5266417Z     |
2019-08-14T02:10:50.5266817Z 137 |                 depr: self.deprecation(v.id),
2019-08-14T02:10:50.5267226Z 
2019-08-14T02:10:50.6594536Z error: aborting due to 15 previous errors
2019-08-14T02:10:50.6594697Z 
2019-08-14T02:10:50.6595063Z Some errors have detailed explanations: E0252, E0560, E0599.
---
2019-08-14T02:10:50.7036640Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "--message-format" "json"
2019-08-14T02:10:50.7036792Z expected success, got: exit code: 101
2019-08-14T02:10:50.7050004Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-08-14T02:10:50.7050124Z Build completed unsuccessfully in 0:06:44
2019-08-14T02:10:52.8175674Z ##[error]Bash exited with code '1'.
2019-08-14T02:10:52.8219541Z ##[section]Starting: Checkout
2019-08-14T02:10:52.8222049Z ==============================================================================
2019-08-14T02:10:52.8222141Z Task         : Get sources
2019-08-14T02:10:52.8222198Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
