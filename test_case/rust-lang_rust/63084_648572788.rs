rust
#[macro_export]
macro_rules! profile_method {
    ($($t:tt)*) => {
        let _firestorm_method_guard = {
            const FIRESTORM_STRUCT_NAME: &'static str = ::std::any::type_name::<Self>();
            let event_data: &'static _ = &$crate::internal::EventData::Start(
                $crate::internal::Start::Method {
                    signature: stringify!($($t)*),
                    typ: FIRESTORM_STRUCT_NAME,
                }
            );
            $crate::internal::start(event_data);
            $crate::internal::SpanGuard
        };
    };
}
