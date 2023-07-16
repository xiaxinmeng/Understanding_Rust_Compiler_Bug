
query stack during panic:
#0 [const_eval_raw] const-evaluating `main`
#1 [const_eval_validated] const-evaluating + checking `main`
#2 [const_eval_validated] const-evaluating + checking `main`
#3 [normalize_generic_arg_after_erasing_regions] normalizing `main::promoted[2]`
#4 [optimized_mir] optimizing MIR for `main`
#5 [collect_and_partition_mono_items] collect_and_partition_mono_items
