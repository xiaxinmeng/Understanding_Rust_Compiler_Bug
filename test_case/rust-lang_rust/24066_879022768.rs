rs
fn ok_with_another_bound_generic<V, W>() -> String
where
    String: From<V> + From<W>,
{
    String::from("✅")
}

fn nok_with_another_bound_concrete<V>() -> String
where
    String: From<V> + From<char>,
{
    String::from("❌")
}

fn ok_with_another_bound_concrete_hrtb<V>() -> String
where
    String: From<V> + for<'a> From<&'a String>,
{
    String::from("✅")
}

fn nok_with_another_bound_concrete_expected_type<V>() -> String
where
    String: From<V> + From<&'static str>,
{
    String::from("❌")
}

fn ok_with_another_bound_concrete_expected_type_local_trait<V>() -> String
where
    String: MyFrom<V> + MyFrom<&'static str>,
{
    String::my_from("✅")
}

fn ok_with_another_bound_concrete_expected_type_local_type<V>() -> MyString
where
    MyString: From<V> + From<&'static str>,
{
    MyString::from("✅")
}

trait MyFrom<T> { fn my_from(_: T) -> Self; }
impl<T> MyFrom<T> for T { fn my_from(x: T) -> Self { x } }
impl MyFrom<&str> for String { fn my_from(c: &str) -> Self { c.into() } }

struct MyString;
impl From<&str> for MyString { fn from(_: &str) -> Self { MyString } }
