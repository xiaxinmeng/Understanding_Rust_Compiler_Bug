
src/libsyntax_pos/lib.rs
1332:    match lines.binary_search(&pos) {
1333:        Ok(line) => line as isize,
1334:        Err(line) => line as isize - 1

src/librustc_data_structures/sorted_map.rs
225:                match self.lookup_index_for(k) {
226:                    Ok(index) => index + 1,
227:                    Err(index) => index,
235:                match self.lookup_index_for(k) {
236:                    Ok(index) => index + 1,
237:                    Err(index) => index,

src/libcore/sync/atomic.rs
456:        match self.compare_exchange(current, new, order, strongest_failure_ordering(order)) {
457:            Ok(x) => x,
458:            Err(x) => x,
943:        match self.compare_exchange(current, new, order, strongest_failure_ordering(order)) {
944:            Ok(x) => x,
945:            Err(x) => x,

src/tools/clippy/clippy_lints/src/doc.rs
218:                    let index = match spans.binary_search_by(|c| c.0.cmp(&offset)) {
219:                        Ok(o) => o,
220:                        Err(e) => e - 1,
