#[cfg(test)]
mod tests {
    use crate::editor_model::*;
    use crate::cursor::Cursor;

    fn setup_empty_model() -> EditorModel {
        let text = "
".to_string();
        let mut model = EditorModel::new();
        model.set_text(text);
        return model;
    }

    fn setup_simple_model() -> EditorModel {
        let text = "Hello world
another sentence
"
        .to_string();
        let mut model = EditorModel::new();
        model.set_text(text);
        return model;
    }

    fn setup_model() -> EditorModel {
        let text = "Hello world
another sentence

And a last sentence
"
        .to_string();
        let mut model = EditorModel::new();
        model.set_text(text);
        return model;
    }

    #[test]
    fn test_undo_redo() {
        let mut model = setup_empty_model();
        model.force_insert_mode();
        model.key_tapped('a' as u32);
        model.key_tapped('b' as u32);
        model.key_tapped('c' as u32);
        assert_eq!(model.get_lines()[0], "abc");
        model.force_normal_mode();
        model.key_tapped('u' as u32);
        assert_eq!(model.get_lines()[0], "ab");
        model.key_tapped('u' as u32);
        assert_eq!(model.get_lines()[0], "a");

    }

    #[test]
    fn test_visual_yanking_pasting() {
        let mut model = setup_model();
        model.force_normal_mode();
        model.key_tapped('j' as u32);
        model.key_tapped('v' as u32);
        model.key_tapped('l' as u32);
        model.key_tapped('l' as u32);
        model.key_tapped('l' as u32);
        model.key_tapped('l' as u32);
        model.key_tapped('y' as u32);
        model.key_tapped('j' as u32);
        model.key_tapped('p' as u32);
        assert_eq!(model.get_lines()[2], "anoth");
    }


    #[test]
    fn test_insertion_at_end_of_line() {
        let mut model = setup_simple_model();
        model.force_insert_mode();
        // Make sure we spam the right key
        for _ in 0..20 {
            model.key_tapped(RIGHT.unwrap() as u32);
        }
        // Tap a key and validate that it inserted the key after the line
        model.key_tapped('f' as u32);
        assert_eq!(model.get_lines()[0], "Hello worldf");
    }

    #[test]
    fn test_normal_forward_motion() {
        let mut model = setup_simple_model();
        model.reset_cursor();
        model.force_normal_mode();
        model.key_tapped('f' as u32);
        model.key_tapped('o' as u32);
        assert_cursor_at(model.get_cursor(), 4, 0);
        // 
        model.key_tapped('f' as u32);
        model.key_tapped('Z' as u32);
        assert_cursor_at(model.get_cursor(), 4, 0);
    }

    #[test]
    fn test_normal_replace() {
        let mut model = setup_simple_model();
        model.reset_cursor();
        model.force_normal_mode();
        model.key_tapped('r' as u32);
        model.key_tapped('a' as u32);
        assert_eq!(model.get_lines()[0], "aello world");
        model.key_tapped('r' as u32);
        model.key_tapped('H' as u32);
        assert_eq!(model.get_lines()[0], "Hello world");
    }

    #[test]
    fn test_cursor_position() {
        let mut model = setup_simple_model();

        model.reset_cursor();
        model.key_tapped(DOWN.unwrap() as u32);
        model.key_tapped(DOWN.unwrap() as u32);

        // Make sure we spam the right key
        for i in 0..20 {
            model.key_tapped(RIGHT.unwrap() as u32);
        }
        assert_cursor_at(model.get_cursor(), 16, 1);

        // Pres the up key, the cursor must go to the left
        model.key_tapped(UP.unwrap() as u32);
        assert_cursor_at(model.get_cursor(), 10, 0);
    }

    #[test]
    fn test_basic_functionalities_insert_mode() {
        // Setup model
        let mut model = setup_simple_model();
        assert_eq!(model.get_lines().len(), 2);

        // Make sure we are in insert mode
        model.force_insert_mode();

        // Type some letters and press enter
        model.reset_cursor();
        model.key_tapped('a' as u32);
        model.key_tapped('a' as u32);
        model.key_tapped('a' as u32);
        model.key_tapped(ENTER.unwrap() as u32);
        println!("Model = {:?}", model.get_lines());
        assert_eq!(model.get_lines().len(), 3);
        assert_eq!(model.get_lines()[0], "aaa");

        // Move the cursor and press enter
        model.key_tapped(RIGHT.unwrap() as u32);
        model.key_tapped(RIGHT.unwrap() as u32);
        model.key_tapped(RIGHT.unwrap() as u32);
        model.key_tapped(RIGHT.unwrap() as u32);
        model.key_tapped(RIGHT.unwrap() as u32);
        model.key_tapped(ENTER.unwrap() as u32);
        assert_eq!(model.get_lines().len(), 4);
        assert_eq!(model.get_lines()[1], "Hello");
        assert_eq!(model.get_lines()[2], " world");

        // Press enter several times
        model.key_tapped(ENTER.unwrap() as u32);
        model.key_tapped(ENTER.unwrap() as u32);
        model.key_tapped(ENTER.unwrap() as u32);
        assert_eq!(model.get_lines().len(), 7);
        assert_eq!(model.get_lines()[5], " world");
        assert_eq!(model.get_lines()[6], "another sentence");

        // Move line, cursor, and delete words
        model.key_tapped(DOWN.unwrap() as u32);
        model.key_tapped(RIGHT.unwrap() as u32);
        model.key_tapped(RIGHT.unwrap() as u32);
        model.key_tapped(RIGHT.unwrap() as u32);
        model.key_tapped(RIGHT.unwrap() as u32);
        model.key_tapped(RIGHT.unwrap() as u32);
        model.key_tapped(BACKSPACE.unwrap() as u32);
        model.key_tapped(BACKSPACE.unwrap() as u32);
        assert_eq!(model.get_lines()[6], "anoer sentence");
    }

    fn assert_cursor_at(cursor: &Cursor, x: usize, y: usize) {
        assert_eq!(cursor.x, x);
        assert_eq!(cursor.y, y);
    }

    #[test]
    fn insert_mode_simple_motion() {
        // 1. Setup model
        let text = "Hello world".to_string();
        let mut model = EditorModel::new();
        model.set_text(text);

        assert_cursor_at(model.get_cursor(), 0, 0);
        for i in 0..10 {
            model.key_tapped(RIGHT.unwrap() as u32);
            assert_cursor_at(model.get_cursor(), i + 1, 0);
        }

        // Validate that if you keep spamming right arrow, the cursor does not move
        model.key_tapped(RIGHT.unwrap() as u32);
        model.key_tapped(RIGHT.unwrap() as u32);
        model.key_tapped(RIGHT.unwrap() as u32);
        model.key_tapped(RIGHT.unwrap() as u32);
        model.key_tapped(RIGHT.unwrap() as u32);
        assert_cursor_at(model.get_cursor(), 11, 0);
    }

    #[test]
    fn test_basic_functionalities_normal_mode() {
        // 1. Setup model
        let text = "Hello world
another sentence"
            .to_string();
        let mut model = EditorModel::new();
        model.set_text(text);

        // Initially the model is in insert
        model.key_tapped(ESCAPE.unwrap() as u32);
        assert_cursor_at(model.get_cursor(), 0, 0);

        // Tap on word
        model.key_tapped('w' as u32);
        assert_cursor_at(model.get_cursor(), 6, 0);
        model.key_tapped('w' as u32);
        model.key_tapped('w' as u32);
    }

    #[test]
    fn normal_mode_move_words() {
        // 1. Setup model
        let text = "Hello world".to_string();
        let mut model = EditorModel::new();
        model.set_text(text);

        // Initially the model is in insert
        model.key_tapped(ESCAPE.unwrap() as u32);
        assert_cursor_at(model.get_cursor(), 0, 0);

        // Tap on word
        model.key_tapped('w' as u32);
        assert_cursor_at(model.get_cursor(), 6, 0);
        model.key_tapped('w' as u32);
        assert_cursor_at(model.get_cursor(), 10, 0);

        // Assert that the position of the cursor does not move
        model.key_tapped('w' as u32);
        model.key_tapped('w' as u32);
        model.key_tapped('w' as u32);
        model.key_tapped('w' as u32);
        model.key_tapped('w' as u32);
        assert_cursor_at(model.get_cursor(), 10, 0);
    }
}
