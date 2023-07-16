
#[rustc_pass_by_value]
trait CustomTrait {
    fn test<T: CustomTrait>(
        value: T,
        reference: &T, //~ ERROR passing `CustomTrait` by reference
    );
}

#[rustc_pass_by_value]
trait CustomTraitAlias: CustomTrait {
    fn test<T: CustomTraitAlias>(
        value: T,
        reference: &T, //~ ERROR passing `CustomTraitAlias` by reference
    );
}

#[rustc_pass_by_value]
enum CustomEnum {}

impl CustomEnum {
    fn test(
        value: CustomEnum,
        reference: &CustomEnum, //~ ERROR passing `CustomEnum` by reference
    ) {
    }
}

#[rustc_pass_by_value]
struct CustomStruct {}

#[rustc_pass_by_value]
type CustomAlias<'a> = &'a CustomStruct;

impl CustomStruct {
    fn test(
        value: CustomStruct,
        reference: &CustomStruct, //~ ERROR passing `CustomStruct` by reference
    ) {
    }

    fn test_alias(
        value: CustomAlias,
        reference: &CustomAlias, //~ ERROR passing `CustomAlias` by reference
    ) {
    }
}



————

not found errors (from test file): [
    Error {
        line_num: 71,
        kind: Some(
            Error,
        ),
        msg: "passing `CustomTrait` by reference",
    },
    Error {
        line_num: 79,
        kind: Some(
            Error,
        ),
        msg: "passing `CustomTraitAlias` by reference",
    },
    Error {
        line_num: 89,
        kind: Some(
            Error,
        ),
        msg: "passing `CustomEnum` by reference",
    },
    Error {
        line_num: 103,
        kind: Some(
            Error,
        ),
        msg: "passing `CustomStruct` by reference",
    },
    Error {
        line_num: 109,
        kind: Some(
            Error,
        ),
        msg: "passing `CustomAlias` by reference",
    },
]
