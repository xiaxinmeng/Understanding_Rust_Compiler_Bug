rust
/**
    Return controls from the Ui. Panics if one of the controls could not be retrieved.  
    
    
    To avoid the panic, use `ui.get` directly.

    Usage:  
    `let control = nwg_get!(ui; (control ID, control type))`  
    `let (control1, control2) = nwg_get!(ui; [(control1_ID, control1 type), (control2_ID, control2_type)])`  
*/
#[macro_export]
macro_rules! nwg_get {
    ( $ui:ident; ($n:expr, $t:ty) ) => {
        $ui.get::<$t>(&$n).expect("Failed to find a control")
    };

    ( $ui:ident; [ $( ($n:expr, $t:ty) ),* ] ) => {
        (
            $( $ui.get::<$t>(&$n).expect("Failed to find a control") ),*
        )
    }
}
