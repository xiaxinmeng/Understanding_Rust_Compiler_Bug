 rust
fn foo<'a,'b>(names: &[&'b str],
              map: &'a Map<&'static str>)
              -> Option<&'a u32>
    where 'b: 'a
{
    map.get(&names[0])
}
