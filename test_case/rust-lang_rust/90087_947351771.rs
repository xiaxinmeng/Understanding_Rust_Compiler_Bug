plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 42983a28ab3c70728da7a9b932b667c978dd898d and e26609663b8339471d0eb244951cd9de0a9ccae3
Clippy or rustfmt subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
test cargo_fmt_tests::targets::all_targets::workspaces::includes_all_packages_from_workspace_subdir ... FAILED

failures:

---- cargo_fmt_tests::targets::all_targets::workspaces::includes_workspace_from_dep_above stdout ----
thread 'cargo_fmt_tests::targets::all_targets::workspaces::includes_workspace_from_dep_above' panicked at 'Targets should have been loaded: Custom { kind: Other, error: "`cargo metadata` exited with an error: error: current package believes it's in a workspace when it's not:\ncurrent:   /checkout/src/tools/rustfmt/tests/cargo-fmt/source/workspaces/path-dep-above/e/Cargo.toml\nworkspace: /checkout/Cargo.toml\n\nthis may be fixable by adding `src/tools/rustfmt/tests/cargo-fmt/source/workspaces/path-dep-above/e` to the `workspace.members` array of the manifest located at: /checkout/Cargo.toml\nAlternatively, to keep it out of the workspace, add the package to the `workspace.exclude` array, or add an empty `[workspace]` table to the package's manifest." }', src/tools/rustfmt/src/cargo-fmt/test/targets.rs:22:14

---- cargo_fmt_tests::targets::all_targets::workspaces::includes_outside_workspace_deps stdout ----
---- cargo_fmt_tests::targets::all_targets::workspaces::includes_outside_workspace_deps stdout ----
thread 'cargo_fmt_tests::targets::all_targets::workspaces::includes_outside_workspace_deps' panicked at 'Targets should have been loaded: Custom { kind: Other, error: "`cargo metadata` exited with an error: error: current package believes it's in a workspace when it's not:\ncurrent:   /checkout/src/tools/rustfmt/tests/cargo-fmt/source/workspaces/path-dep-above/e/Cargo.toml\nworkspace: /checkout/Cargo.toml\n\nthis may be fixable by adding `src/tools/rustfmt/tests/cargo-fmt/source/workspaces/path-dep-above/e` to the `workspace.members` array of the manifest located at: /checkout/Cargo.toml\nAlternatively, to keep it out of the workspace, add the package to the `workspace.exclude` array, or add an empty `[workspace]` table to the package's manifest." }', src/tools/rustfmt/src/cargo-fmt/test/targets.rs:22:14
---- cargo_fmt_tests::targets::all_targets::workspaces::includes_all_packages_from_workspace_subdir stdout ----
---- cargo_fmt_tests::targets::all_targets::workspaces::includes_all_packages_from_workspace_subdir stdout ----
thread 'cargo_fmt_tests::targets::all_targets::workspaces::includes_all_packages_from_workspace_subdir' panicked at 'Targets should have been loaded: Custom { kind: Other, error: "`cargo metadata` exited with an error: error: current package believes it's in a workspace when it's not:\ncurrent:   /checkout/src/tools/rustfmt/tests/cargo-fmt/source/workspaces/path-dep-above/e/Cargo.toml\nworkspace: /checkout/Cargo.toml\n\nthis may be fixable by adding `src/tools/rustfmt/tests/cargo-fmt/source/workspaces/path-dep-above/e` to the `workspace.members` array of the manifest located at: /checkout/Cargo.toml\nAlternatively, to keep it out of the workspace, add the package to the `workspace.exclude` array, or add an empty `[workspace]` table to the package's manifest." }', src/tools/rustfmt/src/cargo-fmt/test/targets.rs:22:14

failures:
    cargo_fmt_tests::targets::all_targets::workspaces::includes_all_packages_from_workspace_subdir
    cargo_fmt_tests::targets::all_targets::workspaces::includes_outside_workspace_deps
