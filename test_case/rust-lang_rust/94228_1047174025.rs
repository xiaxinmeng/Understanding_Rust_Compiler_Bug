plain
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  IMAGE: mingw-check
##[endgroup]
fatal: unknown commit 53fd98ca776cb875bc9e5514f56b52eb74f9e7a9
All commits in `HEAD` are present in `master`
src/ci/scripts/verify-stable-version-number.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: mingw-check
---
    Checking structopt v0.3.25
error[E0433]: failed to resolve: use of undeclared type `Config`
  --> src/tools/rustfmt/src/ignore_path.rs:41:13
   |
41 |             Config::from_toml(r#"ignore = ["foo.rs", "bar_dir/*"]"#, Path::new("")).unwrap();
   |
help: consider importing this struct
   |
35 |     use crate::Config;
35 |     use crate::Config;
   |

error[E0433]: failed to resolve: use of undeclared type `Path`
  --> src/tools/rustfmt/src/ignore_path.rs:41:70
   |
41 |             Config::from_toml(r#"ignore = ["foo.rs", "bar_dir/*"]"#, Path::new("")).unwrap();
   |
help: consider importing one of these items
   |
35 |     use crate::ast::Path;
---

error[E0433]: failed to resolve: use of undeclared type `IgnorePathSet`
  --> src/tools/rustfmt/src/ignore_path.rs:42:31
   |
42 |         let ignore_path_set = IgnorePathSet::from_ignore_list(&config.ignore()).unwrap();
   |
help: consider importing this struct
   |
35 |     use crate::ignore_path::IgnorePathSet;
35 |     use crate::ignore_path::IgnorePathSet;
   |

error[E0433]: failed to resolve: use of undeclared type `FileName`
  --> src/tools/rustfmt/src/ignore_path.rs:44:43
   |
44 |         assert!(ignore_path_set.is_match(&FileName::Real(PathBuf::from("src/foo.rs"))));
   |
help: consider importing one of these items
   |
35 |     use crate::FileName;
35 |     use crate::FileName;
   |
35 |     use rustc_span::FileName;
   |

error[E0433]: failed to resolve: use of undeclared type `PathBuf`
  --> src/tools/rustfmt/src/ignore_path.rs:44:58
   |
44 |         assert!(ignore_path_set.is_match(&FileName::Real(PathBuf::from("src/foo.rs"))));
   |
help: consider importing one of these items
   |
35 |     use crate::PathBuf;
35 |     use crate::PathBuf;
   |
35 |     use std::path::PathBuf;
   |

error[E0433]: failed to resolve: use of undeclared type `FileName`
  --> src/tools/rustfmt/src/ignore_path.rs:45:43
   |
45 |         assert!(ignore_path_set.is_match(&FileName::Real(PathBuf::from("bar_dir/baz.rs"))));
   |
help: consider importing one of these items
   |
35 |     use crate::FileName;
35 |     use crate::FileName;
   |
35 |     use rustc_span::FileName;
   |

error[E0433]: failed to resolve: use of undeclared type `PathBuf`
  --> src/tools/rustfmt/src/ignore_path.rs:45:58
   |
45 |         assert!(ignore_path_set.is_match(&FileName::Real(PathBuf::from("bar_dir/baz.rs"))));
   |
help: consider importing one of these items
   |
35 |     use crate::PathBuf;
35 |     use crate::PathBuf;
   |
35 |     use std::path::PathBuf;
   |

error[E0433]: failed to resolve: use of undeclared type `FileName`
  --> src/tools/rustfmt/src/ignore_path.rs:46:44
   |
46 |         assert!(!ignore_path_set.is_match(&FileName::Real(PathBuf::from("src/bar.rs"))));
   |
help: consider importing one of these items
   |
35 |     use crate::FileName;
35 |     use crate::FileName;
   |
35 |     use rustc_span::FileName;
   |

error[E0433]: failed to resolve: use of undeclared type `PathBuf`
  --> src/tools/rustfmt/src/ignore_path.rs:46:59
   |
46 |         assert!(!ignore_path_set.is_match(&FileName::Real(PathBuf::from("src/bar.rs"))));
   |
help: consider importing one of these items
   |
35 |     use crate::PathBuf;
