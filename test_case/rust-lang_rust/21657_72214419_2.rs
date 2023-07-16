 rust
let tmp = Box::new(2);
{
    let mut map: HashMap<u8, &u8> = HashMap::new();
    {
        map.insert(43, &*tmp);
    }
}
// no fails this time
