rust
pub fn negotiate<'stream, S>(link: S)
where
    S: 'stream,
    for<'a> &'a S: 'a,
{
    fn lives_as_long<'a, T: 'a>(_: T) {}
    lives_as_long::<'stream, _>(|| {
        let _l = link;
    })
}

fn main() {}
