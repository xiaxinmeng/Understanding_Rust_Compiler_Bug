
> error: unconstrained generic constant
>    --> lightsaber_util\src\libcore_ext\str\validations\amd64\mod.rs:136:38
>     |
> 136 |         Self::from(_mm_alignr_epi8::<{ 16 - IMM8 }>(
>     |                                      ^^^^^^^^^^^^^
>     |
>     = help: try adding a `where` bound using this expression: `where [(); { 16 - IMM8 }]:`
> 