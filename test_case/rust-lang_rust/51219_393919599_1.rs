rust
#![feature(proc_macro, generators)]

extern crate futures_await as futures;
extern crate ldap3;
extern crate tokio_core;

#[macro_use]
extern crate failure;

use tokio_core::reactor::Handle;
use ldap3::{LdapConnAsync, Scope};
use futures::prelude::*;
use failure::Error;

use std::io::Write;

#[derive(Fail)]
enum LdapError {
    #[fail(display = "InvalidCreds: {}", name)]
    InvalidCreds { name: String },

    #[fail(display = "NotInGroup: {}", group_name)]
    NotInGroup { group_name: String },
}

#[async]
fn test_user_has_access(handle: Handle) -> Result<(), Error> {
    let r = LdapConnAsync::new("ldap://localhost:2389", &handle)?
        .map_err(|e| {
            let e2: Error = e.into();
            e2
        })
        .and_then(move |ldap| {
            ldap.simple_bind("bind_user", "bind_pass")
                .map_err(Into::into)
                .and_then(move |ldap2| {
                    let j = if ldap2.rc == 49 {
                        Err(LdapError::InvalidCreds { name: "some_user".to_string() }.into())
                    } else {
                        Ok(())
                    };

                    j.into_future().and_then(|()| {
                        ldap.search("something", Scope::Subtree, "something", vec![])
                            .map_err(Into::into)
                    })
                })
        });

    return Err(LdapError::NotInGroup {
        name: "test".to_string(),
    }.into());
}

fn main() {}
