
#[stream(item = (Cow<'a, ()>, Cow<'a, ()>))]
async fn next2<'a, S>(s1: S, s2: S)
where
    S: Stream<Item = (Cow<'a, ()>, Cow<'a, ()>)> + 'static,
{
    pin_mut!(s1);
    pin_mut!(s2);
    s1.next().await.unwrap();
    s2.next().await.unwrap();
}
