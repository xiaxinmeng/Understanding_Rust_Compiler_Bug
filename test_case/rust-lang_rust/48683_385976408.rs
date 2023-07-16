TOML
  [profile.build]
  # low hanging optimization fruits which shouldn't affect build time much
  opt-level = 1 
  # save build time and disk space by not emitting huge amount of debug-info
  debug = false 
  # speed up both compile time and runtime
  debug-assertions = false
  overflow-checks = false
  # build scripts are small and rarely modified
  incremental = false
  