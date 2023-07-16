
$ callgrind_annotate --tree=caller results/clgout-9ef27bf7dc50a8b51435579b4f2e86f7ee3f7a94-inflate-Check-Full
2,960,080,594 (64.18%)  < ???:_ZN21rustc_trait_selection6traits7fulfill18FulfillmentContext6select17h80d1450895e486aeE.llvm.13260143977094445196 (34,057x)
2,220,443,052 (48.14%)  *  ???:rustc_data_structures::obligation_forest::ObligationForest<O>::process_obligations

  404,736,181 ( 8.78%)  < ???:rustc_data_structures::obligation_forest::ObligationForest<O>::process_obligations (8,826x)
  201,820,150 ( 4.38%)  *  ???:rustc_data_structures::obligation_forest::ObligationForest<O>::compress

$ callgrind_annotate --tree=caller results/clgout-c7dbe7a830100c70d59994fd940bf75bb6e39b39-inflate-Check-Full | less
3,056,791,553 (64.92%)  < ???:_ZN21rustc_trait_selection6traits7fulfill18FulfillmentContext6select17h80d1450895e486aeE.llvm.9518623118464054305 (34,057x)
2,293,350,331 (48.71%)  *  ???:rustc_data_structures::obligation_forest::ObligationForest<O>::process_obligations

  404,659,331 ( 8.59%)  < ???:rustc_data_structures::obligation_forest::ObligationForest<O>::process_obligations (8,826x)
  201,740,042 ( 4.28%)  *  ???:rustc_data_structures::obligation_forest::ObligationForest<O>::compress
