
macro_rules! stmt_list(
    ( while $cond:expr { $($body:tt)+ } ) => ( while $cond { $($body:tt)+ } );
    ( $head:stmt ; ) => ( { $head; } );
    ( $head:stmt ; $($rest:tt)+ ) => ( { $head ; stmt_list!( $($rest)+ ) } );
)

fn main()
{
    trace_macros!(true);

    stmt_list!(
        while x < 100 {
            let y = x + 1;
            x = y;
        }
    )
}
