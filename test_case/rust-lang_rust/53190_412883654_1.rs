
fn licenseck(file: &Path, contents: &str) -> bool {
    if contents.contains("ignore-license") {
        return true
    }
    let exceptions = [
        "libstd/sync/mpsc/mpsc_queue.rs",
        "libstd/sync/mpsc/spsc_queue.rs",
    ];
    if exceptions.iter().any(|f| file.ends_with(f)) {
        return true
    }

    // Skip the BOM if it's there
    let bom = "\u{feff}";
    let contents = if contents.starts_with(bom) {&contents[3..]} else {contents};

    // See if the license shows up in the first 100 lines
    let lines = contents.lines().take(100).collect::<Vec<_>>();
    lines.windows(LICENSE.lines().count()).any(|window| {
        let offset = if window.iter().all(|w| w.starts_with("//")) {
            2
        } else if window.iter().all(|w| w.starts_with('#')) {
            1
        } else if window.iter().all(|w| w.starts_with(" *")) {
            2
        } else {
            return false
        };
        window.iter().map(|a| a[offset..].trim())
              .zip(LICENSE.lines()).all(|(a, b)| {
            a == b || match b.find("<year>") {
                Some(i) => a.starts_with(&b[..i]) && a.ends_with(&b[i+6..]),
                None => false,
            }
        })
    })

}
