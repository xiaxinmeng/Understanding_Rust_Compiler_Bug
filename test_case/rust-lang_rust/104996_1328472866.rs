rust
const ESCAPE: usize = VK_ESCAPE.0 as _;

if let WPARAM(ESCAPE) = wparam {
    DestroyWindow(hwnd);
}
