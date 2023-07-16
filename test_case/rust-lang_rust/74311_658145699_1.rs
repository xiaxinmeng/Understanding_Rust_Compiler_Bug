rust
fn main() {
    unsafe {
        Some(()).unwrap();
        setlocale(LC_ALL, b"\0".as_ptr() as _);
        initscr();
        cbreak(); 
        noecho(); 
        keypad(stdscr, true);
        clear();
        let s = "x";
        let t = "Привет\0";
        move_(10, 10);
        addnstr(s.as_ptr() as _, 1);
        move_(10, 10);
        insnstr(t.as_ptr() as _, 2);
        getch();
        endwin();
    }
}
