use super::super::Theme;
use ratatui::style::Color;

/// Solarized Dark color scheme
pub fn solarized_dark() -> Theme {
    Theme {
        background_left: Color::Rgb(0, 36, 41),
        background_right: Color::Rgb(0, 43, 54),

        editor_line_number: Color::Rgb(88, 110, 117),
        editor_line_number_cursor: Color::Rgb(38, 139, 210),
        editor_separator: Color::Rgb(88, 110, 117),
        editor_cursor_char_bg: Color::Rgb(38, 139, 210),
        editor_cursor_char_fg: Color::Rgb(0, 43, 54),
        editor_cursor_line_bg: Color::Rgb(7, 54, 66),

        file_tree_added: Color::Rgb(133, 153, 0),
        file_tree_deleted: Color::Rgb(220, 50, 47),
        file_tree_modified: Color::Rgb(181, 137, 0),
        file_tree_renamed: Color::Rgb(38, 139, 210),
        file_tree_directory: Color::Rgb(42, 161, 152),
        file_tree_current_file_bg: Color::Rgb(7, 54, 66),
        file_tree_current_file_fg: Color::Rgb(238, 232, 213),
        file_tree_default: Color::Rgb(131, 148, 150),
        file_tree_stats_added: Color::Rgb(133, 153, 0),
        file_tree_stats_deleted: Color::Rgb(220, 50, 47),

        terminal_command: Color::Rgb(238, 232, 213),
        terminal_output: Color::Rgb(88, 110, 117),
        terminal_cursor_bg: Color::Rgb(38, 139, 210),
        terminal_cursor_fg: Color::Rgb(0, 43, 54),

        status_hash: Color::Rgb(181, 137, 0),
        status_author: Color::Rgb(133, 153, 0),
        status_date: Color::Rgb(38, 139, 210),
        status_message: Color::Rgb(238, 232, 213),
        status_no_commit: Color::Rgb(88, 110, 117),

        separator: Color::Rgb(88, 110, 117),

        syntax_keyword: Color::Rgb(203, 75, 22),
        syntax_type: Color::Rgb(181, 137, 0),
        syntax_function: Color::Rgb(38, 139, 210),
        syntax_variable: Color::Rgb(131, 148, 150),
        syntax_string: Color::Rgb(42, 161, 152),
        syntax_number: Color::Rgb(108, 113, 196),
        syntax_comment: Color::Rgb(88, 110, 117),
        syntax_operator: Color::Rgb(203, 75, 22),
        syntax_punctuation: Color::Rgb(131, 148, 150),
        syntax_constant: Color::Rgb(108, 113, 196),
        syntax_parameter: Color::Rgb(181, 137, 0),
        syntax_property: Color::Rgb(42, 161, 152),
        syntax_label: Color::Rgb(211, 54, 130),
    }
}
