plain
test init::bin_already_exists_implicit_namenosrc ... ok
test init::bin_already_exists_implicit_nosrc ... ok
test init::bin_already_exists_implicit_namesrc ... ok
test init::both_lib_and_bin ... ok
test init::cant_create_library_when_both_binlib_present ... ok
test init::cargo_lock_gitignored_if_lib2 ... ok
test init::confused_by_multiple_lib_files ... ok
test init::cargo_lock_not_gitignored_if_bin2 ... ok
test init::cargo_lock_not_gitignored_if_bin1 ... ok
test init::cargo_lock_not_gitignored_if_bin1 ... ok
test init::creates_binary_when_both_binlib_present ... ok
test init::creates_binary_when_instructed_and_has_lib_file_no_warning ... ok
test init::creates_library_when_instructed_and_has_bin_file ... ok
test glob_targets::test_example ... ok
test init::formats_source ... ok
test init::git_autodetect ... ok
test init::gitignore_added_newline_in_existing ... ok
---
test test::test_workspaces_cwd ... ok

failures:

---- metadata::package_default_run stdout ----
running `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo metadata`
thread 'metadata::package_default_run' panicked at '
Expected: execs
    but: JSON mismatch
{
{
  "metadata": null,
  "packages": [
      "authors": [
      "authors": [
        "wycats@example.com"
      ],
      "categories": [],
      "default_run": "a",
      "dependencies": [],
      "description": null,
      "documentation": null,
      "edition": "2018",
      "features": {},
      "homepage": null,
      "id": "foo 0.1.0 (path+file:[..])",
      "keywords": [],
      "license": null,
      "license_file": null,
      "links": null,
      "manifest_path": "[..]Cargo.toml",
      "metadata": null,
      "name": "foo",
      "publish": null,
      "readme": null,
      "repository": null,
      "source": null,
      "targets": [
        {
          "crate_types": [
            "lib"
          ],
          "doc": true,
          "doctest": true,
          "edition": "2018",
          "kind": [
            "lib"
          ],
          "name": "foo",
          "src_path": "[..]src/lib.rs",
          "test": true
        {
        {
          "crate_types": [
            "bin"
          ],
          "doc": true,
          "doctest": false,
          "edition": "2018",
          "kind": [
            "bin"
          ],
          "name": "a",
          "src_path": "[..]src/bin/a.rs",
          "test": true
        {
        {
          "crate_types": [
            "bin"
          ],
          "doc": true,
          "doctest": false,
          "edition": "2018",
          "kind": [
            "bin"
          ],
          "name": "b",
          "src_path": "[..]src/bin/b.rs",
          "test": true
      ],
      "version": "0.1.0"
    }
  ],
  ],
  "resolve": {
    "nodes": [
      {
        "dependencies": [],
        "deps": [],
        "features": [],
        "id": "foo 0.1.0 (path+file:[..])"
    ],
    ],
    "root": "foo 0.1.0 (path+file:[..])"
  },
  "target_directory": "[..]",
  "version": 1,
  "workspace_members": [
    "foo 0.1.0 (path+file:[..])"
  ],
  "workspace_root": "[..]"
Was:
{
{
  "metadata": null,
  "packages": [
      "authors": [
      "authors": [
        "wycats@example.com"
      ],
      "categories": [],
      "default_run": "a",
      "dependencies": [],
      "description": null,
      "documentation": null,
      "edition": "2018",
      "features": {},
      "homepage": null,
      "id": "foo 0.1.0 (path+file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/tmp/cit/t1315/foo)",
      "keywords": [],
      "license": null,
      "license_file": null,
      "links": null,
      "manifest_path": "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/tmp/cit/t1315/foo/Cargo.toml",
      "metadata": null,
      "name": "foo",
      "publish": null,
      "readme": null,
      "repository": null,
      "source": null,
      "targets": [
        {
          "crate_types": [
            "lib"
          ],
          "doc": true,
          "doctest": true,
          "edition": "2018",
          "kind": [
            "lib"
          ],
          "name": "foo",
          "src_path": "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/tmp/cit/t1315/foo/src/lib.rs",
          "test": true
        {
        {
          "crate_types": [
            "bin"
          ],
          "doc": true,
          "doctest": false,
          "edition": "2018",
          "kind": [
            "bin"
          ],
          "name": "b",
          "src_path": "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/tmp/cit/t1315/foo/src/bin/b.rs",
          "test": true
        {
        {
          "crate_types": [
            "bin"
          ],
          "doc": true,
          "doctest": false,
          "edition": "2018",
          "kind": [
            "bin"
          ],
          "name": "a",
          "src_path": "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/tmp/cit/t1315/foo/src/bin/a.rs",
          "test": true
      ],
      "version": "0.1.0"
    }
  ],
  ],
  "resolve": {
    "nodes": [
      {
        "dependencies": [],
        "deps": [],
        "features": [],
        "id": "foo 0.1.0 (path+file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/tmp/cit/t1315/foo)"
    ],
    ],
    "root": "foo 0.1.0 (path+file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/tmp/cit/t1315/foo)"
  },
  "target_directory": "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/tmp/cit/t1315/foo/target",
  "version": 1,
  "workspace_members": [
    "foo 0.1.0 (path+file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/tmp/cit/t1315/foo)"
  ],
  "workspace_root": "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/tmp/cit/t1315/foo"
Expected part:
"a"
Actual part:
"b"
---


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 src/tools/cargo src/tools/cargotest
Build completed unsuccessfully in 0:17:58
make: *** [check-aux] Error 1
Makefile:44: recipe for target 'check-aux' failed
