
let c1 = catch { a: 10 };
let c2 = catch { ..c1 }; // <--

struct catch {}
let c3 = catch {}; // <--
