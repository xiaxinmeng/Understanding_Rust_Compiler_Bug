
*   15491d7b6be - (origin/master, origin/HEAD) Auto merge of #89343 - Mark-Simulacrum:no-args-queries, r=cjgillot (7 days ago) <bors>
|\  
| * 415a9a2ea69 - (origin/pr/89343) Add a test that -Zquery-dep-graph -Zdump-dep-graph works (7 days ago) <Mark Rousskov>
| * 6f78eed1c75 - Query the fingerprint style during key reconstruction (10 days ago) <Mark Rousskov>
* |   bb918d0a5bf - Auto merge of #89698 - matthiaskrgr:rollup-gna54x6, r=matthiaskrgr (8 days ago) <bors>
|\ \  
| * \   2e5a5e22b21 - (origin/pr/89698) Rollup merge of #89697 - alessandrod:issue-89689, r=nikic (8 days ago) <Matthias Krüger>
| |\ \  
| | * | 8683d36042d - (origin/pr/89697) Fix min LLVM version for bpf-types test (8 days ago) <Alessandro Decina>
| * | |   827b540424c - Rollup merge of #89694 - jkugelman:must-use-string-transforms, r=joshtriplett (8 days ago) <Matthias Krüger>
| |\ \ \  
| | * | | 2ec7588aa1a - (origin/pr/89694) Update library/core/src/num/mod.rs (8 days ago) <John Kugelman>
| | * | | 54d807cfc7f - Add #[must_use] to string/char transformation methods (8 days ago) <John Kugelman>
| | |/ /  
| * | |   ee804594c83 - Rollup merge of #89693 - jkugelman:must-use-stdin-stdout-stderr-locks, r=joshtriplett (8 days ago) <Matthias Krüger>
| |\ \ \  
| | * | | e27bfb6e237 - (origin/pr/89693) Add #[must_use] to stdin/stdout/stderr locks (8 days ago) <John Kugelman>
| | |/ /  
| * | |   a06c664328c - Rollup merge of #89687 - Nicholas-Baron:move_read2_abbreviated, r=Mark-Simulacrum (8 days ago) <Matthias Krüger>
| |\ \ \  
| | * | | 8a4085d370a - (origin/pr/89687) Move read2_abbreviated function into read2.rs (8 days ago) <Nicholas-Baron>
| * | | |   0481c67dd46 - Rollup merge of #89684 - asquared31415:asm-doc-fix, r=joshtriplett (8 days ago) <Matthias Krüger>
| |\ \ \ \  
| | * | | | 4a565e51100 - (origin/pr/89684) Fix asm docs typo (8 days ago) <asquared31415>
| * | | | |   346f833c3dd - Rollup merge of #89678 - marcelo-gonzalez:master, r=joshtriplett (8 days ago) <Matthias Krüger>
| |\ \ \ \ \  
| | * | | | | 82c974dab5c - (origin/pr/89678) Fix minor std::thread documentation typo (8 days ago) <Marcelo Diop-Gonzalez>
| | |/ / / /  
| * | | | |   5ebb6a8fd93 - Rollup merge of #89641 - asquared31415:asm-feature-attr-regs, r=oli-obk (8 days ago) <Matthias Krüger>
| |\ \ \ \ \  
| | * | | | | 271da7d8bc9 - (origin/pr/89641) make #[target_feature] work with `asm` register classes (9 days ago) <asquared31415>
| * | | | | |   9d14b6505b3 - Rollup merge of #89634 - hawkw:eliza/enable-err-warn, r=oli-obk (8 days ago) <Matthias Krüger>
| |\ \ \ \ \ \  
| | * | | | | | 84fc5db59b3 - (origin/pr/89634) bless warnings (8 days ago) <Eliza Weisman>
| | * | | | | | 0e79545c306 - lol i forgot the syntax for my own crate's macros (9 days ago) <Eliza Weisman>
| | * | | | | | b6f09a19b23 - comma-related changes (9 days ago) <Eliza Weisman>
| | * | | | | | e00eac8b9c4 - use structured fields in some existing warnings (9 days ago) <Eliza Weisman>
| | * | | | | | 928c787fcee - make them structured while i'm here (9 days ago) <Eliza Weisman>
| | * | | | | | 01803025d2a - demote `rustc_peek` traces look not user-facing (9 days ago) <Eliza Weisman>
| | * | | | | | eb67bf93682 - Update compiler/rustc_driver/src/lib.rs (9 days ago) <Eliza Weisman>
| | * | | | | | e7f04857efc - rustc_driver: Enable the `WARN` log level by default (9 days ago) <Eliza Weisman>
| * | | | | | |   03a34a291d8 - Rollup merge of #89605 - camelid:fix-version, r=nagisa (8 days ago) <Matthias Krüger>
| |\ \ \ \ \ \ \  
| | * | | | | | | 6189d0a116c - (origin/pr/89605) Fix stabilization version for `bindings_after_at` (10 days ago) <Noah Lev>
| | | |/ / / / /  
| | |/| | | | |   
| * | | | | | | 36db6587965 - Rollup merge of #88707 - sylvestre:split_example, r=yaahc (8 days ago) <Matthias Krüger>
|/| | | | | | | 
| * | | | | | | d4031d092d0 - (origin/pr/88707) String.split_terminator: Add an example when using a slice of chars (6 weeks ago) <Sylvestre Ledru>
* | | | | | | |   910692de742 - Auto merge of #89582 - jkugelman:optimize-file-read-to-end, r=joshtriplett (8 days ago) <bors>
