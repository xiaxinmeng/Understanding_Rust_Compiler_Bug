rust
#![allow(dead_code)]
#![allow(unused_variables)]

use rkyv::{Archive, Archived, Deserialize, Fallible};

const ARRAY_SIZE: usize = 32;
struct SomeStruct1 {
    buffer_one: Option<[u8; ARRAY_SIZE]>,
    buffer_two: Option<[u8; ARRAY_SIZE]>,
}

struct ArchivedSomeStruct1
where
    Option<[u8; ARRAY_SIZE]>: ::rkyv::Archive,
    Option<[u8; ARRAY_SIZE]>: ::rkyv::Archive,
{
    buffer_one: ::rkyv::Archived<Option<[u8; ARRAY_SIZE]>>,
    buffer_two: ::rkyv::Archived<Option<[u8; ARRAY_SIZE]>>,
}

struct SomeStruct1Resolver
where
    Option<[u8; ARRAY_SIZE]>: ::rkyv::Archive,
    Option<[u8; ARRAY_SIZE]>: ::rkyv::Archive,
{
    buffer_one: ::rkyv::Resolver<Option<[u8; ARRAY_SIZE]>>,
    buffer_two: ::rkyv::Resolver<Option<[u8; ARRAY_SIZE]>>,
}

impl Archive for SomeStruct1
where
    Option<[u8; ARRAY_SIZE]>: ::rkyv::Archive,
    Option<[u8; ARRAY_SIZE]>: ::rkyv::Archive,
{
    type Archived = ArchivedSomeStruct1;
    type Resolver = SomeStruct1Resolver;
    #[allow(clippy::unit_arg)]
    #[inline]
    unsafe fn resolve(&self, pos: usize, resolver: Self::Resolver, out: *mut Self::Archived) {
        unreachable!();
    }
}

impl<__D: Fallible + ?Sized> Deserialize<SomeStruct1, __D> for Archived<SomeStruct1>
where
    Option<[u8; ARRAY_SIZE]>: Archive,
    Archived<Option<[u8; ARRAY_SIZE]>>: Deserialize<Option<[u8; ARRAY_SIZE]>, __D>,
    Option<[u8; ARRAY_SIZE]>: Archive,
    Archived<Option<[u8; ARRAY_SIZE]>>: Deserialize<Option<[u8; ARRAY_SIZE]>, __D>,
{
    fn deserialize(
        &self,
        deserializer: &mut __D,
    ) -> ::core::result::Result<SomeStruct1, __D::Error> {
        unreachable!();
    }
}

fn main() {}
