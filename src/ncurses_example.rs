use ncurses::*;
pub fn example1() {
    /* Start ncurses. */
    initscr();

    /* Print to the back buffer. */
    addstr("Hello, world!");

    /* Update the screen. */
    refresh();

    /* Wait for a key press. */
    getch();

    /* Terminate ncurses. */
    endwin();
}

pub fn example2() {
    initscr();
    raw();

    /* Allow for extended keyboard (like F1). */
    keypad(stdscr(), true);
    noecho();

    /* Prompt for a character. */
    addstr("Enter a character: ");

    /* Wait for input. */
    let ch = getch();
    if ch == KEY_F(1) {
        // Enable attributes (bold and blinking)
        attron(A_BOLD | A_BLINK);
        addstr("\nF1 key");
        attroff(A_BOLD | A_BLINK);
        addstr(" pressed");
    } else {
        /* Enable attributes and output message. */
        addstr("\nKey pressed: ");
        attron(A_BOLD | A_BLINK);
        addstr(format!("{}\n", char::from_u32(ch as u32).expect("Invalid char")).as_ref());
        attroff(A_BOLD | A_BLINK);
    }

    /* Refresh, showing the previous message. */
    refresh();

    /* Wait for one more character before exiting. */
    getch();
    endwin();
}



static WINDOW_HEIGHT: i32 = 3;
static WINDOW_WIDTH: i32 = 10;

pub fn example3() {
    /* Setup ncurses. */
    initscr();
    raw();

    /* Allow for extended keyboard (like F1). */
    keypad(stdscr(), true);
    noecho();

    /* Invisible cursor. */
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    /* Status/help info. */
    addstr("Use the arrow keys to move");
    mvprintw(LINES() - 1, 0, "Press F1 to exit");
    refresh();

    /* Get the screen bounds. */
    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);

    /* Start in the center. */
    let mut start_y = (max_y - WINDOW_HEIGHT) / 2;
    let mut start_x = (max_x - WINDOW_WIDTH) / 2;
    let mut win = create_win(start_y, start_x);

    let mut ch = getch();
    while ch != KEY_F(1) {
        match ch {
            KEY_LEFT => {
                start_x -= 1;
                destroy_win(win);
                win = create_win(start_y, start_x);
            }
            KEY_RIGHT => {
                start_x += 1;
                destroy_win(win);
                win = create_win(start_y, start_x);
            }
            KEY_UP => {
                start_y -= 1;
                destroy_win(win);
                win = create_win(start_y, start_x);
            }
            KEY_DOWN => {
                start_y += 1;
                destroy_win(win);
                win = create_win(start_y, start_x);
            }
            _ => {}
        }
        ch = getch();
    }

    endwin();
}

fn create_win(start_y: i32, start_x: i32) -> WINDOW {
    let win = newwin(WINDOW_HEIGHT, WINDOW_WIDTH, start_y, start_x);
    box_(win, 0, 0);
    wrefresh(win);
    win
}

fn destroy_win(win: WINDOW) {
    let ch = ' ' as chtype;
    wborder(win, ch, ch, ch, ch, ch, ch, ch, ch);
    wrefresh(win);
    delwin(win);
}
/*
 */
