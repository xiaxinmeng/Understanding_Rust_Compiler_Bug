rust
fn heart() { }

#[doc(hidden)]
#[allow(non_camel_case_types)]
#[doc = r" Rocket code generated proxy structure."]
struct heart {}
#[doc = r" Rocket code generated proxy static conversion implementation."]
impl From<heart> for rocket::StaticRouteInfo {
    #[allow(non_snake_case, unreachable_patterns, unreachable_code)]
    fn from(_: heart) -> rocket::StaticRouteInfo {
        fn monomorphized_function<'_b>(
            __req: &'_b rocket::request::Request,
            __data: rocket::data::Data,
        ) -> rocket::handler::HandlerFuture<'_b> {
            ::std::boxed::Box::pin(async move {
                let ___responder = heart();
                rocket::handler::Outcome::from(__req, ___responder)
            })
        }
        rocket::StaticRouteInfo {
            name: "heart",
            method: ::rocket::http::Method::Get,
            path: "/♥",
            handler: monomorphized_function,
            format: ::std::option::Option::None,
            rank: ::std::option::Option::None,
        }
    }
}
#[doc = r" Rocket code generated proxy conversion implementation."]
impl From<heart> for rocket::Route {
    #[inline]
    fn from(_: heart) -> rocket::Route {
        rocket::StaticRouteInfo::from(heart {}).into()
    }
}
#[doc = r" Rocket code generated wrapping URI macro."]
#[doc(hidden)]
#[macro_export]
macro_rules! rocket_uri_macro_heart2471868972214400932 {
    ($ ($ token : tt) *) =>
    {
        {
            extern crate std ; extern crate rocket ; rocket ::
            rocket_internal_uri ! ("/♥", (), $ ($ token) *)
        }
    } ;
}

#[doc(hidden)]
pub use rocket_uri_macro_heart2471868972214400932 as rocket_uri_macro_heart;

fn main() { }
