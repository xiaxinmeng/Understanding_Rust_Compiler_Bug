
RUSTC_LOG_COLOR=never RUSTC_LOG="rustc_trait_selection::traits::select[evaluate_predicate_recursively]=debug" rustc +stage1 src/main.rs --emit=metadata --cfg 'depth = "4"' 2>&1 | tee /shared/depth4.txt
