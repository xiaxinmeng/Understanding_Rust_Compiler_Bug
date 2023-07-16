
$ ./target/release/collector diff_local cachegrind +9ef27bf7dc50a8b51435579b4f2e86f7ee3f7a94 +c7dbe7a830100c70d59994fd940bf75bb6e39b39 --include inflate --builds Check --runs Full
$ less results/cgann-9ef27bf7dc50a8b51435579b4f2e86f7ee3f7a94-c7dbe7a830100c70d59994fd940bf75bb6e39b39-inflate-Check-Full
72,907,279  ???:rustc_data_structures::obligation_forest::ObligationForest<O>::process_obligations
23,931,248  ???:ena::unify::UnificationTable<S>::uninlined_get_root_key
