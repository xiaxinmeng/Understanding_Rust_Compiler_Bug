
enum task = uint;
fn defaults() -> opts;
fn spawn(opts: opts, body: fn~()) -> task;
