c
#include <locale.h>
#include <curses.h>

int main() {
    setlocale(LC_ALL, "");
    initscr();
    cbreak(); 
    noecho(); 
    keypad(stdscr, true);
    clear();
    move(10, 10);
    addnstr("x", 1);
    move(10, 10);
    insnstr("Привет", 2);
    getch();
    endwin();
    return 0;
}
