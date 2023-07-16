
process(infcx, P); // processes P, adding constraints to infcx
let constraints = infcx.take_constraints(); // takes all registered constriants
nll_context.record_constraints(constriants, P); // record them in the new format
