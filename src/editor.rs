use ncurses::*;

use crate::editor_model::EditorModel;

pub struct TextEditor {
    model: EditorModel,
    background_color: u32,
}

impl TextEditor {
    const X_BASELINE: i32 = 4;
    const Y_BASELINE: i32 = 2;

    pub fn new(filename: String) -> Self {
        Self {
            model: EditorModel::from_file(filename),
            background_color: 0,
        }
    }

    pub fn display(&mut self) {
        //Start NCurses
        initscr();
        cbreak();
        keypad(stdscr(), true);
        noecho();

        use_default_colors();
        start_color();
        init_pair(1, COLOR_BLACK, -1);
        init_pair(2, COLOR_MAGENTA, -1);
        init_pair(3, COLOR_BLACK, COLOR_CYAN);
        attron(COLOR_PAIR(1));

        // Get the default color pairs
        self.background_color = getbkgd(stdscr());
        eprintln!("Background: {}", self.background_color);

        // Get the screen bounds
        let mut max_x = 0;
        let mut max_y = 0;
        getmaxyx(stdscr(), &mut max_y, &mut max_x);
        self.model.set_screen_h(max_y - TextEditor::Y_BASELINE);
        self.model.set_screen_w(max_x - TextEditor::X_BASELINE);
        eprintln!("Screen bounds: {}, {}", max_x, max_y);

        // First drawing
        self.draw_screen();

        // Handle user inputs
        let mut ch = getch();
        while ch != KEY_F(1) {
            // Handle keys via the model
            self.model.key_tapped(ch as u32);

            // Draw based on the model
            self.draw_screen();

            // Get the next key
            ch = getch();
        }

        /* Terminate ncurses. */
        endwin();
    }
}

impl TextEditor {
    /// Function in charge of drawing the entire visible screen
    /// It is called after every key is tapped, once the model has been updated.
    fn draw_screen(&mut self) {
        let lines = self.model.get_lines();
        let screen = self.model.get_screen_info();
        let cursor = self.model.get_cursor();
        let selection = self.model.get_selection();

        // Number of time that a breakline happened while drawing the provided range
        let mut breakline_count = 0;
        let mut cursor_y_pos: i32 = -1;
        let mut cursor_x_pos: i32 = cursor.x as i32;

        // Start by clearing the entire screen
        wmove(stdscr(), 0, 0);
        clrtobot();

        for line_number in 0..lines.len() {
            let line_number = line_number as i32;
            // The line in the screen reference
            let line_in_screen = line_number + breakline_count - screen.top;

            // Only the index inside the visible screen are printed
            if screen.is_line_visible(line_number) {
                // Since we have to draw the cursor after having finished all the lines (easier
                // with ncurses), we save the cursor display line if we pass through it.
                if cursor.y as i32 == line_number {
                    cursor_y_pos = line_in_screen;
                }

                // Print the line number
                wmove(stdscr(), line_in_screen, 0);
                clrtoeol();
                attron(COLOR_PAIR(2));
                addstr(format!("{}", line_number).as_str());
                attron(COLOR_PAIR(1));

                // Move to the text baseline
                wmove(stdscr(), line_in_screen, TextEditor::X_BASELINE);

                // Print the line (line number followed by )
                // If the text goes beyond the screen, we split the line in sublines
                // and print each one of them accordingly.
                let line = &lines[line_number as usize];
                let ranges = screen.split_line(line);
                let has_selection =
                    selection.is_some() && selection.unwrap().contains_line(line_number as usize);

                if ranges.len() <= 1 {
                    if has_selection {
                        // The line is partially selected
                        // There are three parts to draw:
                        // - the beginning
                        // - the selected
                        // - the end

                        let mut start = 0;
                        let mut end = line.len();

                        if selection.unwrap().start().y == line_number as usize {
                            start = selection.unwrap().start().x;
                        }

                        if selection.unwrap().end().y == line_number as usize {
                            end = selection.unwrap().end().x;
                        }

                        let part1 = &line[0..start];
                        let part2 = &line[start..end];
                        let part3 = &line[end..line.len()];

                        addstr(part1);
                        attron(COLOR_PAIR(3));
                        addstr(part2);
                        attron(COLOR_PAIR(1));
                        addstr(part3);
                    } else {
                        addstr(line);
                    }
                } else {
                    // If the cursor is placed on this line, we must correct its position
                    // Correct the cursor position to be placed correctly
                    if cursor.y as i32 == line_number {
                        cursor_x_pos = cursor.x as i32 % screen.w;
                        cursor_y_pos += cursor.x as i32 / screen.w;
                    }

                    // Print all the sublines
                    let initial_breakline_count = breakline_count;
                    for range in ranges {
                        let range_start = range.start;
                        let range_end = range.end;
                        let subline = &line[range];
                        wmove(
                            stdscr(),
                            line_in_screen + breakline_count - initial_breakline_count,
                            TextEditor::X_BASELINE,
                        );

                        if has_selection {
                            // by default, the entire subline is printed as as selected
                            // These values are changed depending on the selection
                            let mut start = 0;
                            let mut end = subline.len();

                            // If the selection starts in this line, change the start 
                            if selection.unwrap().start().y == line_number as usize {
                                // If the start in the range, then change it
                                if selection.unwrap().start().x <= range_end && selection.unwrap().start().x >= range_start {
                                    start = selection.unwrap().start().x - range_start;
                                }
                                // If the starts is after the range, then do not highlight anything
                                else if selection.unwrap().start().x > range_end {
                                    // It means the selection does not start now
                                    start = subline.len();
                                }
                            }

                            // If the selection ends in this line, change the end
                            if selection.unwrap().end().y == line_number as usize {
                                // if the end is before the range, then do not select anything
                                if selection.unwrap().end().x < range_start {
                                    start = subline.len();
                                }
                                // if the end is in the range, the change the end of the visual
                                else if selection.unwrap().end().x <= range_end && selection.unwrap().end().x >= range_start {
                                    end = selection.unwrap().end().x - range_start;
                                }
                            }

                            let part1 = &subline[0..start];
                            let part2 = &subline[start..end];
                            let part3 = &subline[end..subline.len()];
                            addstr(part1);
                            attron(COLOR_PAIR(3));
                            addstr(part2);
                            attron(COLOR_PAIR(1));
                            addstr(part3);

                        } else {
                            addstr(subline);
                        }

                        breakline_count += 1;
                    }
                    // We have to remove 1 breakline
                    breakline_count -= 1;
                }
                /*
                 */
                clrtoeol();
            }
        }

        // If we have drawn the last line, we clear everything after
        clrtobot();

        // Status bar (at the bottom)
        wmove(stdscr(), LINES() - 2, 0);
        hline('-' as u32, 1000);
        mvprintw(LINES() - 1, 0, self.model.get_status_message().as_str());

        // Finally, we draw the cursor
        if cursor_y_pos >= 0 {
            wmove(
                stdscr(),
                cursor_y_pos,
                cursor_x_pos + TextEditor::X_BASELINE,
            );
            wrefresh(stdscr());
        }
    }
}
