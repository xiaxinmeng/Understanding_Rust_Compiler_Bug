
#16 0x00007ffff2f44dbb in rustc::traits::select::SelectionContext::select (self=0x7fffecabfbc0, obligation=0x7fffeca9e488) at librustc/traits/select.rs:526
#17 0x00007ffff3607861 in rustc::traits::project::assemble_candidates_from_impls::{{closure}} () at librustc/traits/project.rs:1013
#18 rustc::infer::InferCtxt::probe (self=<optimized out>, f=...) at librustc/infer/mod.rs:905
#19 0x00007ffff2f8b99f in rustc::traits::project::assemble_candidates_from_impls (selcx=0x7fffecabfbc0, obligation=<optimized out>, obligation_trait_ref=<optimized out>, candidate_set=0x7fffeca9f3d0) at librustc/traits/project.rs:1012
#20 rustc::traits::project::project_type (selcx=<optimized out>, obligation=<optimized out>) at librustc/traits/project.rs:817
--
#49 0x00007ffff2f44dbb in rustc::traits::select::SelectionContext::select (self=0x7fffecabfbc0, obligation=0x7fffeca9e488) at librustc/traits/select.rs:526
#50 0x00007ffff3607861 in rustc::traits::project::assemble_candidates_from_impls::{{closure}} () at librustc/traits/project.rs:1013
#51 rustc::infer::InferCtxt::probe (self=<optimized out>, f=...) at librustc/infer/mod.rs:905
#52 0x00007ffff2f8b99f in rustc::traits::project::assemble_candidates_from_impls (selcx=0x7fffecabfbc0, obligation=<optimized out>, obligation_trait_ref=<optimized out>, candidate_set=0x7fffecaa1550) at librustc/traits/project.rs:1012
#53 rustc::traits::project::project_type (selcx=<optimized out>, obligation=<optimized out>) at librustc/traits/project.rs:817
--
#82 0x00007ffff2f44dbb in rustc::traits::select::SelectionContext::select (self=0x7fffecabfbc0, obligation=0x7fffeca9e488) at librustc/traits/select.rs:526
#83 0x00007ffff2f8c276 in rustc::traits::project::confirm_select_candidate (selcx=0x7fffecabfbc0, obligation=0x7fffecaa3650, obligation_trait_ref=<optimized out>) at librustc/traits/project.rs:1174
#84 rustc::traits::project::confirm_candidate (obligation=0x7fffecaa3650, obligation_trait_ref=<optimized out>, selcx=<optimized out>, candidate=...) at librustc/traits/project.rs:1161
#85 rustc::traits::project::project_type (selcx=<optimized out>, obligation=<optimized out>) at librustc/traits/project.rs:878
#86 0x00007ffff2f88038 in rustc::traits::project::opt_normalize_projection_type (selcx=0x7fffecabfbc0, param_env=..., projection_ty=..., cause=..., depth=<optimized out>) at librustc/traits/project.rs:541
--
#114 0x00007ffff2f44dbb in rustc::traits::select::SelectionContext::select (self=0x7fffecabfbc0, obligation=0x7fffeca9e488) at librustc/traits/select.rs:526
#115 0x00007ffff3607861 in rustc::traits::project::assemble_candidates_from_impls::{{closure}} () at librustc/traits/project.rs:1013
#116 rustc::infer::InferCtxt::probe (self=<optimized out>, f=...) at librustc/infer/mod.rs:905
#117 0x00007ffff2f8b99f in rustc::traits::project::assemble_candidates_from_impls (selcx=0x7fffecabfbc0, obligation=<optimized out>, obligation_trait_ref=<optimized out>, candidate_set=0x7fffecaa55f0) at librustc/traits/project.rs:1012
#118 rustc::traits::project::project_type (selcx=<optimized out>, obligation=<optimized out>) at librustc/traits/project.rs:817
--
#147 0x00007ffff2f44dbb in rustc::traits::select::SelectionContext::select (self=0x7fffecabfbc0, obligation=0x7fffeca9e488) at librustc/traits/select.rs:526
#148 0x00007ffff3607861 in rustc::traits::project::assemble_candidates_from_impls::{{closure}} () at librustc/traits/project.rs:1013
#149 rustc::infer::InferCtxt::probe (self=<optimized out>, f=...) at librustc/infer/mod.rs:905
#150 0x00007ffff2f8b99f in rustc::traits::project::assemble_candidates_from_impls (selcx=0x7fffecabfbc0, obligation=<optimized out>, obligation_trait_ref=<optimized out>, candidate_set=0x7fffecaa7770) at librustc/traits/project.rs:1012
#151 rustc::traits::project::project_type (selcx=<optimized out>, obligation=<optimized out>) at librustc/traits/project.rs:817
--
#180 0x00007ffff2f44dbb in rustc::traits::select::SelectionContext::select (self=0x7fffecabfbc0, obligation=0x7fffeca9e488) at librustc/traits/select.rs:526
#181 0x00007ffff3607861 in rustc::traits::project::assemble_candidates_from_impls::{{closure}} () at librustc/traits/project.rs:1013
#182 rustc::infer::InferCtxt::probe (self=<optimized out>, f=...) at librustc/infer/mod.rs:905
#183 0x00007ffff2f8b99f in rustc::traits::project::assemble_candidates_from_impls (selcx=0x7fffecabfbc0, obligation=<optimized out>, obligation_trait_ref=<optimized out>, candidate_set=0x7fffecaa98f0) at librustc/traits/project.rs:1012
#184 rustc::traits::project::project_type (selcx=<optimized out>, obligation=<optimized out>) at librustc/traits/project.rs:817
--
#213 0x00007ffff2f44dbb in rustc::traits::select::SelectionContext::select (self=0x7fffecabfbc0, obligation=0x7fffeca9e488) at librustc/traits/select.rs:526
#214 0x00007ffff2f8c276 in rustc::traits::project::confirm_select_candidate (selcx=0x7fffecabfbc0, obligation=0x7fffecaab9f0, obligation_trait_ref=<optimized out>) at librustc/traits/project.rs:1174
#215 rustc::traits::project::confirm_candidate (obligation=0x7fffecaab9f0, obligation_trait_ref=<optimized out>, selcx=<optimized out>, candidate=...) at librustc/traits/project.rs:1161
#216 rustc::traits::project::project_type (selcx=<optimized out>, obligation=<optimized out>) at librustc/traits/project.rs:878
#217 0x00007ffff2f88038 in rustc::traits::project::opt_normalize_projection_type (selcx=0x7fffecabfbc0, param_env=..., projection_ty=..., cause=..., depth=<optimized out>) at librustc/traits/project.rs:541
--
#245 0x00007ffff2f44dbb in rustc::traits::select::SelectionContext::select (self=0x7fffecabfbc0, obligation=0x7fffeca9e488) at librustc/traits/select.rs:526
#246 0x00007ffff2f8c276 in rustc::traits::project::confirm_select_candidate (selcx=0x7fffecabfbc0, obligation=0x7fffecaad910, obligation_trait_ref=<optimized out>) at librustc/traits/project.rs:1174
#247 rustc::traits::project::confirm_candidate (obligation=0x7fffecaad910, obligation_trait_ref=<optimized out>, selcx=<optimized out>, candidate=...) at librustc/traits/project.rs:1161
#248 rustc::traits::project::project_type (selcx=<optimized out>, obligation=<optimized out>) at librustc/traits/project.rs:878
#249 0x00007ffff2f88038 in rustc::traits::project::opt_normalize_projection_type (selcx=0x7fffecabfbc0, param_env=..., projection_ty=..., cause=..., depth=<optimized out>) at librustc/traits/project.rs:541
--
#277 0x00007ffff2f44dbb in rustc::traits::select::SelectionContext::select (self=0x7fffecabfbc0, obligation=0x7fffeca9e488) at librustc/traits/select.rs:526
#278 0x00007ffff2f8c276 in rustc::traits::project::confirm_select_candidate (selcx=0x7fffecabfbc0, obligation=0x7fffecaaf830, obligation_trait_ref=<optimized out>) at librustc/traits/project.rs:1174
#279 rustc::traits::project::confirm_candidate (obligation=0x7fffecaaf830, obligation_trait_ref=<optimized out>, selcx=<optimized out>, candidate=...) at librustc/traits/project.rs:1161
#280 rustc::traits::project::project_type (selcx=<optimized out>, obligation=<optimized out>) at librustc/traits/project.rs:878
#281 0x00007ffff2f88038 in rustc::traits::project::opt_normalize_projection_type (selcx=0x7fffecabfbc0, param_env=..., projection_ty=..., cause=..., depth=<optimized out>) at librustc/traits/project.rs:541
--
#309 0x00007ffff2f44dbb in rustc::traits::select::SelectionContext::select (self=0x7fffecabfbc0, obligation=0x7fffeca9e488) at librustc/traits/select.rs:526
#310 0x00007ffff3607861 in rustc::traits::project::assemble_candidates_from_impls::{{closure}} () at librustc/traits/project.rs:1013
#311 rustc::infer::InferCtxt::probe (self=<optimized out>, f=...) at librustc/infer/mod.rs:905
#312 0x00007ffff2f8b99f in rustc::traits::project::assemble_candidates_from_impls (selcx=0x7fffecabfbc0, obligation=<optimized out>, obligation_trait_ref=<optimized out>, candidate_set=0x7fffecab17d0) at librustc/traits/project.rs:1012
#313 rustc::traits::project::project_type (selcx=<optimized out>, obligation=<optimized out>) at librustc/traits/project.rs:817
--
#342 0x00007ffff2f44dbb in rustc::traits::select::SelectionContext::select (self=0x7fffecabfbc0, obligation=0x7fffeca9e488) at librustc/traits/select.rs:526
#343 0x00007ffff2f8c276 in rustc::traits::project::confirm_select_candidate (selcx=0x7fffecabfbc0, obligation=0x7fffecab38d0, obligation_trait_ref=<optimized out>) at librustc/traits/project.rs:1174
#344 rustc::traits::project::confirm_candidate (obligation=0x7fffecab38d0, obligation_trait_ref=<optimized out>, selcx=<optimized out>, candidate=...) at librustc/traits/project.rs:1161
#345 rustc::traits::project::project_type (selcx=<optimized out>, obligation=<optimized out>) at librustc/traits/project.rs:878
#346 0x00007ffff2f88038 in rustc::traits::project::opt_normalize_projection_type (selcx=0x7fffecabfbc0, param_env=..., projection_ty=..., cause=..., depth=<optimized out>) at librustc/traits/project.rs:541
--
#374 0x00007ffff2f44dbb in rustc::traits::select::SelectionContext::select (self=0x7fffecabfbc0, obligation=0x7fffeca9e488) at librustc/traits/select.rs:526
#375 0x00007ffff2f8c276 in rustc::traits::project::confirm_select_candidate (selcx=0x7fffecabfbc0, obligation=0x7fffecab57f0, obligation_trait_ref=<optimized out>) at librustc/traits/project.rs:1174
#376 rustc::traits::project::confirm_candidate (obligation=0x7fffecab57f0, obligation_trait_ref=<optimized out>, selcx=<optimized out>, candidate=...) at librustc/traits/project.rs:1161
#377 rustc::traits::project::project_type (selcx=<optimized out>, obligation=<optimized out>) at librustc/traits/project.rs:878
#378 0x00007ffff2f88038 in rustc::traits::project::opt_normalize_projection_type (selcx=0x7fffecabfbc0, param_env=..., projection_ty=..., cause=..., depth=<optimized out>) at librustc/traits/project.rs:541
--
#406 0x00007ffff2f44dbb in rustc::traits::select::SelectionContext::select (self=0x7fffecabfbc0, obligation=0x7fffeca9e488) at librustc/traits/select.rs:526
#407 0x00007ffff2f8c276 in rustc::traits::project::confirm_select_candidate (selcx=0x7fffecabfbc0, obligation=0x7fffecab7710, obligation_trait_ref=<optimized out>) at librustc/traits/project.rs:1174
#408 rustc::traits::project::confirm_candidate (obligation=0x7fffecab7710, obligation_trait_ref=<optimized out>, selcx=<optimized out>, candidate=...) at librustc/traits/project.rs:1161
#409 rustc::traits::project::project_type (selcx=<optimized out>, obligation=<optimized out>) at librustc/traits/project.rs:878
#410 0x00007ffff2f88038 in rustc::traits::project::opt_normalize_projection_type (selcx=0x7fffecabfbc0, param_env=..., projection_ty=..., cause=..., depth=<optimized out>) at librustc/traits/project.rs:541
--
#438 0x00007ffff2f44dbb in rustc::traits::select::SelectionContext::select (self=0x7fffecabfbc0, obligation=0x7fffeca9e488) at librustc/traits/select.rs:526
#439 0x00007ffff2f8c276 in rustc::traits::project::confirm_select_candidate (selcx=0x7fffecabfbc0, obligation=0x7fffecab9630, obligation_trait_ref=<optimized out>) at librustc/traits/project.rs:1174
#440 rustc::traits::project::confirm_candidate (obligation=0x7fffecab9630, obligation_trait_ref=<optimized out>, selcx=<optimized out>, candidate=...) at librustc/traits/project.rs:1161
#441 rustc::traits::project::project_type (selcx=<optimized out>, obligation=<optimized out>) at librustc/traits/project.rs:878
#442 0x00007ffff2f88038 in rustc::traits::project::opt_normalize_projection_type (selcx=0x7fffecabfbc0, param_env=..., projection_ty=..., cause=..., depth=<optimized out>) at librustc/traits/project.rs:541
--
#470 0x00007ffff2f44dbb in rustc::traits::select::SelectionContext::select (self=0x7fffecabfbc0, obligation=0x7fffeca9e488) at librustc/traits/select.rs:526
#471 0x00007ffff3607861 in rustc::traits::project::assemble_candidates_from_impls::{{closure}} () at librustc/traits/project.rs:1013
#472 rustc::infer::InferCtxt::probe (self=<optimized out>, f=...) at librustc/infer/mod.rs:905
#473 0x00007ffff2f8b99f in rustc::traits::project::assemble_candidates_from_impls (selcx=0x7fffecabfbc0, obligation=<optimized out>, obligation_trait_ref=<optimized out>, candidate_set=0x7fffecabb5d0) at librustc/traits/project.rs:1012
#474 rustc::traits::project::project_type (selcx=<optimized out>, obligation=<optimized out>) at librustc/traits/project.rs:817
--
#503 0x00007ffff2f44dbb in rustc::traits::select::SelectionContext::select (self=0x7fffecabfbc0, obligation=0x7fffeca9e488) at librustc/traits/select.rs:526
#504 0x00007ffff3607861 in rustc::traits::project::assemble_candidates_from_impls::{{closure}} () at librustc/traits/project.rs:1013
#505 rustc::infer::InferCtxt::probe (self=<optimized out>, f=...) at librustc/infer/mod.rs:905
#506 0x00007ffff2f8b99f in rustc::traits::project::assemble_candidates_from_impls (selcx=0x7fffecabfbc0, obligation=<optimized out>, obligation_trait_ref=<optimized out>, candidate_set=0x7fffecabd750) at librustc/traits/project.rs:1012
#507 rustc::traits::project::project_type (selcx=<optimized out>, obligation=<optimized out>) at librustc/traits/project.rs:817
