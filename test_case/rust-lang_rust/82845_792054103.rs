
C:\Users\Alice\Documents\GitHub\bevy [ecs-docs ↓1 +0 ~1 -0 !]> cargo rustdoc -- -Z time-passes
time:   0.000; rss:   17MB ->   17MB (   +0MB)  parse_crate
time:   0.000; rss:   17MB ->   17MB (   +0MB)  attributes_injection
time:   0.000; rss:   17MB ->   17MB (   +0MB)  recursion_limit
time:   0.000; rss:   18MB ->   18MB (   +0MB)  plugin_loading
time:   0.000; rss:   18MB ->   18MB (   +0MB)  plugin_registration
time:   0.000; rss:   18MB ->   18MB (   +0MB)  crate_injection
time:   0.076; rss:   18MB ->   26MB (   +8MB)  expand_crate
time:   0.000; rss:   26MB ->   26MB (   +0MB)  check_unused_macros
time:   0.076; rss:   18MB ->   26MB (   +8MB)  macro_expand_crate
time:   0.000; rss:   26MB ->   26MB (   +0MB)  maybe_building_test_harness
time:   0.000; rss:   26MB ->   26MB (   +0MB)  AST_validation
time:   0.000; rss:   26MB ->   26MB (   +0MB)  maybe_create_a_macro_crate
time:   0.001; rss:   26MB ->   27MB (   +0MB)  finalize_imports
time:   0.000; rss:   27MB ->   27MB (   +0MB)  finalize_macro_resolutions
time:   0.000; rss:   27MB ->   27MB (   +0MB)  late_resolve_crate
time:   0.000; rss:   27MB ->   27MB (   +0MB)  resolve_check_unused
time:   0.000; rss:   27MB ->   27MB (   +0MB)  resolve_report_errors
time:   0.000; rss:   27MB ->   27MB (   +0MB)  resolve_postprocess
time:   0.001; rss:   26MB ->   27MB (   +0MB)  resolve_crate
time:   0.000; rss:   27MB ->   27MB (   +0MB)  complete_gated_feature_checking
time:   0.077; rss:   18MB ->   27MB (   +9MB)  configure_and_expand
time:   0.000; rss:   27MB ->   27MB (   +0MB)  load_extern_crates
time:   0.000; rss:   27MB ->   27MB (   +0MB)  prepare_outputs
time:   0.000; rss:   27MB ->   27MB (   +0MB)  hir_lowering
time:   0.000; rss:   27MB ->   27MB (   +0MB)  early_lint_checks
time:   0.000; rss:   28MB ->   28MB (   +0MB)  setup_global_ctxt
time:   0.000; rss:   27MB ->   28MB (   +1MB)  create_global_ctxt
time:   0.000; rss:   28MB ->   29MB (   +1MB)  item_types_checking
time:   0.000; rss:   29MB ->   30MB (   +1MB)  crate_lints
time:   0.000; rss:   30MB ->   30MB (   +0MB)  module_lints
time:   0.000; rss:   29MB ->   30MB (   +1MB)  missing_docs
time:   0.000; rss:   30MB ->   30MB (   +0MB)  check_mod_attrs
time:   0.300; rss:  134MB ->  199MB (  +64MB)  clean_crate
time:   6.034; rss:  199MB ->  345MB ( +147MB)  collect_synthetic_impls
time:   0.003; rss:  345MB ->  346MB (   +0MB)  collect_items_for_trait_impls
time:   7.886; rss:  199MB ->  664MB ( +466MB)  collect-trait-impls
time:   0.031; rss:  664MB ->  643MB (  -21MB)  unindent-comments
time:   0.038; rss:  643MB ->  644MB (   +0MB)  check-private-items-doc-tests
time:   0.088; rss:  644MB ->  645MB (   +1MB)  strip-hidden
time:   0.056; rss:  645MB ->  645MB (   +0MB)  strip-private
time:   0.152; rss:  645MB ->  648MB (   +3MB)  collect-intra-doc-links
time:   0.077; rss:  648MB ->  648MB (   +0MB)  check-code-block-syntax
time:   0.018; rss:  648MB ->  648MB (   +0MB)  check-invalid-html-tags
time:   0.024; rss:  648MB ->  648MB (   +0MB)  propagate-doc-cfg
time:   0.019; rss:  648MB ->  649MB (   +1MB)  check-non-autolinks
time:   0.335; rss:  649MB ->  706MB (  +57MB)  create_format_cache
time:   9.162; rss:   28MB ->  706MB ( +678MB)  run_global_ctxt
time:   0.203; rss:  706MB ->  713MB (   +7MB)  create_renderer(html)
time:   0.002; rss:  714MB ->  714MB (   +0MB)  renderer_after_krate(html)
time:   2.793; rss:  706MB ->  504MB ( -203MB)  render_html
time:   0.075; rss:  504MB ->   87MB ( -417MB)  free_global_ctxt
    Finished dev [unoptimized + debuginfo] target(s) in 1.05s
