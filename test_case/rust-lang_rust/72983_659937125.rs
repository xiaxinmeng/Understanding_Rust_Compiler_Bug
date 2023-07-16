
2020-07-17T07:20:52.3570346Z [0m[1m[38;5;9merror[E0609][0m[0m[1m: no field `tables` on type `&mem_categorization::MemCategorizationContext<'a, 'tcx>`[0m
2020-07-17T07:20:52.3571107Z [0m   [0m[0m[1m[38;5;12m--> [0m[0msrc/librustc_typeck/mem_categorization.rs:429:22[0m
2020-07-17T07:20:52.3571457Z [0m    [0m[0m[1m[38;5;12m|[0m
2020-07-17T07:20:52.3571837Z [0m[1m[38;5;12m429[0m[0m [0m[0m[1m[38;5;12m| [0m[0m                    .tables[0m
2020-07-17T07:20:52.3572284Z [0m    [0m[0m[1m[38;5;12m| [0m[0m                     [0m[0m[1m[38;5;9m^^^^^^[0m[0m [0m[0m[1m[38;5;9munknown field[0m
2020-07-17T07:20:52.3572613Z [0m    [0m[0m[1m[38;5;12m|[0m
2020-07-17T07:20:52.3573495Z [0m    [0m[0m[1m[38;5;12m= [0m[0m[1mnote[0m[0m: available fields are: `typeck_results`, `infcx`, `param_env`, `body_owner`, `upvars`[0m
2020-07-17T07:20:52.3573627Z 
2020-07-17T07:20:52.3732591Z [0m[1m[38;5;9merror[E0609][0m[0m[1m: no field `tables` on type `&mem_categorization::MemCategorizationContext<'a, 'tcx>`[0m
2020-07-17T07:20:52.3733108Z [0m   [0m[0m[1m[38;5;12m--> [0m[0msrc/librustc_typeck/mem_categorization.rs:652:24[0m
2020-07-17T07:20:52.3733457Z [0m    [0m[0m[1m[38;5;12m|[0m
2020-07-17T07:20:52.3733881Z [0m[1m[38;5;12m652[0m[0m [0m[0m[1m[38;5;12m| [0m[0m        let res = self.tables.qpath_res(qpath, pat_hir_id);[0m
2020-07-17T07:20:52.3734323Z [0m    [0m[0m[1m[38;5;12m| [0m[0m                       [0m[0m[1m[38;5;9m^^^^^^[0m[0m [0m[0m[1m[38;5;9munknown field[0m
2020-07-17T07:20:52.3734649Z [0m    [0m[0m[1m[38;5;12m|[0m
2020-07-17T07:20:52.3735088Z [0m    [0m[0m[1m[38;5;12m= [0m[0m[1mnote[0m[0m: available fields are: `typeck_results`, `infcx`, `param_env`, `body_owner`, `upvars`[0m
2020-07-17T07:20:52.3735241Z 
2020-07-17T07:20:52.3735931Z [0m[1m[38;5;9merror[E0609][0m[0m[1m: no field `tables` on type `&mem_categorization::MemCategorizationContext<'a, 'tcx>`[0m
2020-07-17T07:20:52.3736342Z [0m   [0m[0m[1m[38;5;12m--> [0m[0msrc/librustc_typeck/mem_categorization.rs:653:23[0m
2020-07-17T07:20:52.3736657Z [0m    [0m[0m[1m[38;5;12m|[0m
2020-07-17T07:20:52.3737307Z [0m[1m[38;5;12m653[0m[0m [0m[0m[1m[38;5;12m| [0m[0m        let ty = self.tables.node_type(pat_hir_id);[0m
2020-07-17T07:20:52.3737812Z [0m    [0m[0m[1m[38;5;12m| [0m[0m                      [0m[0m[1m[38;5;9m^^^^^^[0m[0m [0m[0m[1m[38;5;9munknown field[0m
2020-07-17T07:20:52.3738130Z [0m    [0m[0m[1m[38;5;12m|[0m
2020-07-17T07:20:52.3738572Z [0m    [0m[0m[1m[38;5;12m= [0m[0m[1mnote[0m[0m: available fields are: `typeck_results`, `infcx`, `param_env`, `body_owner`, `upvars`[0m
2020-07-17T07:20:52.3738677Z 
2020-07-17T07:20:52.3762286Z [0m[1m[38;5;9merror[E0609][0m[0m[1m: no field `tables` on type `&mem_categorization::MemCategorizationContext<'a, 'tcx>`[0m
2020-07-17T07:20:52.3762999Z [0m   [0m[0m[1m[38;5;12m--> [0m[0msrc/librustc_typeck/mem_categorization.rs:688:23[0m
2020-07-17T07:20:52.3763323Z [0m    [0m[0m[1m[38;5;12m|[0m
2020-07-17T07:20:52.3763740Z [0m[1m[38;5;12m688[0m[0m [0m[0m[1m[38;5;12m| [0m[0m        let ty = self.tables.node_type(pat_hir_id);[0m
2020-07-17T07:20:52.3764190Z [0m    [0m[0m[1m[38;5;12m| [0m[0m                      [0m[0m[1m[38;5;9m^^^^^^[0m[0m [0m[0m[1m[38;5;9munknown field[0m
2020-07-17T07:20:52.3764523Z [0m    [0m[0m[1m[38;5;12m|[0m
2020-07-17T07:20:52.3764966Z [0m    [0m[0m[1m[38;5;12m= [0m[0m[1mnote[0m[0m: available fields are: `typeck_results`, `infcx`, `param_env`, `body_owner`, `upvars`[0m
2020-07-17T07:20:52.3765092Z 
2020-07-17T07:20:52.3776763Z [0m[1m[38;5;9merror[E0609][0m[0m[1m: no field `tables` on type `&mem_categorization::MemCategorizationContext<'a, 'tcx>`[0m
2020-07-17T07:20:52.3777492Z [0m   [0m[0m[1m[38;5;12m--> [0m[0msrc/librustc_typeck/mem_categorization.rs:703:23[0m
2020-07-17T07:20:52.3778100Z [0m    [0m[0m[1m[38;5;12m|[0m
2020-07-17T07:20:52.3778504Z [0m[1m[38;5;12m703[0m[0m [0m[0m[1m[38;5;12m| [0m[0m        let ty = self.tables.node_type(pat_hir_id);[0m
2020-07-17T07:20:52.3779101Z [0m    [0m[0m[1m[38;5;12m| [0m[0m                      [0m[0m[1m[38;5;9m^^^^^^[0m[0m [0m[0m[1m[38;5;9munknown field[0m
2020-07-17T07:20:52.3779415Z [0m    [0m[0m[1m[38;5;12m|[0m
2020-07-17T07:20:52.3779986Z [0m    [0m[0m[1m[38;5;12m= [0m[0m[1mnote[0m[0m: available fields are: `typeck_results`, `infcx`, `param_env`, `body_owner`, `upvars`[0m
2020-07-17T07:20:52.3780285Z 
2020-07-17T07:20:52.3900645Z [0m[1m[38;5;9merror[E0609][0m[0m[1m: no field `tables` on type `&mem_categorization::MemCategorizationContext<'a, 'tcx>`[0m
2020-07-17T07:20:52.3901147Z [0m   [0m[0m[1m[38;5;12m--> [0m[0msrc/librustc_typeck/mem_categorization.rs:821:26[0m
2020-07-17T07:20:52.3901471Z [0m    [0m[0m[1m[38;5;12m|[0m
2020-07-17T07:20:52.3901890Z [0m[1m[38;5;12m821[0m[0m [0m[0m[1m[38;5;12m| [0m[0m                        .tables[0m
2020-07-17T07:20:52.3902327Z [0m    [0m[0m[1m[38;5;12m| [0m[0m                         [0m[0m[1m[38;5;9m^^^^^^[0m[0m [0m[0m[1m[38;5;9munknown field[0m
2020-07-17T07:20:52.3902647Z [0m    [0m[0m[1m[38;5;12m|[0m
2020-07-17T07:20:52.3903083Z [0m    [0m[0m[1m[38;5;12m= [0m[0m[1mnote[0m[0m: available fields are: `typeck_results`, `infcx`, `param_env`, `body_owner`, `upvars`[0m
2020-07-17T07:20:52.3903212Z 
2020-07-17T07:20:52.5078843Z [0m[1m[38;5;9merror[0m[0m[1m: aborting due to 6 previous errors[0m
2020-07-17T07:20:52.5079025Z 
2020-07-17T07:20:52.5079372Z [0m[1mFor more information about this error, try `rustc --explain E0609`.[0m
2020-07-17T07:20:52.5135800Z [0m[0m[1m[31merror[0m[1m:[0m could not compile `rustc_typeck`.
