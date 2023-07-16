rust
trait Fetch<'world> {
    type Item;
}

struct Query<Q>(Q);

impl<Q: WorldQuery> Query<Q> {
    fn get_mut(&mut self) -> <Q::Fetch as Fetch>::Item {
        todo!()
    }
}

trait WorldQuery {
    type Fetch: for<'world> Fetch<'world>;
}
impl WorldQuery for () {
    type Fetch = WorldFetch;
}
struct WorldFetch;
impl<'w, 's> Fetch<'w> for WorldFetch {
    type Item = ();
}

fn get_children_mut<Q, C>(q: &mut Query<Q>, callback: C)
where
    Q: WorldQuery,
    C: Fn(<<Q as WorldQuery>::Fetch as Fetch>::Item),
{
    callback(q.get_mut());
}

fn main() {
    get_children_mut(&mut Query(()), |_| {});
}
