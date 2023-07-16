diff
             $( $( $path:ident )::+ ($it:pat) => $res:expr,   ##)*
             _ => $catch_all:expr $(,)?
         }) => {{
    -        $( if let Some($it) = $($path::)+cast($node.clone()) { $res } else )*
    +        macro_rules! braceless_expr {
    +            ({$inner:expr}) => ($inner);
    +            ($braceless:expr) => ($braceless);
    +        }
    +
    +        $( if let Some($it) = $($path::)+cast($node.clone()) { defile!(braceless_expr!(@ $res)) } else )*
             { $catch_all }
         }};
     }
    