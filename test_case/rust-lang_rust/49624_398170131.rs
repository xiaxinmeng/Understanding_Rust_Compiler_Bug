rust
let param_env1 = tcx.param_env(impl1);
                let param_env2 = tcx.param_env(impl2);

                let mut combined_param_envs_vec =
                    param_env1
                        .caller_bounds
                        .iter()
                        .chain(param_env2.caller_bounds.iter())
                        .map(|p| *p)
.collect::<Vec<_>>();
