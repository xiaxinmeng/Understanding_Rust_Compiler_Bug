rust
#[macro_export]
macro_rules! slider {
    ( $name:expr, $min:expr, $max:expr, $value:expr ) => {
        html!(<Slider name=$name min=$min max=$max value=&mut $value as *mut Value value_changed=value_changed.clone() />)
    };
}
