 rust
trait Resources {
    type Buffer;
}

#[derive(Copy)]
struct ConstantBufferSet<R: Resources>(
    pub R::Buffer
);

enum It {}
impl Resources for It {}

#[derive(Copy)]
enum Command {
    BindConstantBuffers(ConstantBufferSet<It>)
}
