
trait FromRedisValue {}
trait ConnectionLike {}
struct Cmd;

pub struct Iter<'a, 'b, T: FromRedisValue, CL: ConnectionLike> {
    batch: Vec<T>,
    cursor: u64,
    con: &'b CL,
    cmd: &'a Cmd,
}

fn main() {}
