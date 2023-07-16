
pub fn bfs<N, FN, IN, FS>(start: &N, successors: FN, success: FS) -> Option<Vec<N>>
where
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
    FS: FnMut(&N) -> bool,
{
    None
}

struct Test;

fn main() {
    let success = |node| true;
    let successors = |node| {
        vec![]
    };
    let answer = bfs(&Test, successors, success).unwrap().len();
    println!("answer = {answer}");
}
