use editor::TextEditor;
use gag::Redirect;
use std::fs::OpenOptions;
use std::env;

/// Define the ncurses_example module
//mod ncurses_example;
mod cursor;
mod editor;
mod editor_action;
mod undo_redo;
mod editor_model;
mod modes;
mod motion;
mod selection;
mod screen;
mod yanker;

mod test_model;


fn main() {
    // Open a log file for 'stderr', since stdout is used by ncurses
    let log = OpenOptions::new()
        .truncate(true)
        .read(true)
        .create(true)
        .write(true)
        .open("tmp.log")
        .unwrap();
    let _print_redirect = Redirect::stderr(log).unwrap();

    // Load a file
    let args: Vec<String> = env::args().collect();
    let file = if args.len() > 1 {
        args[1].clone()
    } else {
        "".to_string()
    };

    eprintln!("Reading file: {file}");

    // Open the viewer
    let mut viewer = TextEditor::new(file);
    viewer.display();
}
