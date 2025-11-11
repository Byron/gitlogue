use super::super::Theme;
use ratatui::style::Color;

/// Solarized Light color scheme
pub fn solarized_light() -> Theme {
    Theme {
        background_left: Color::Rgb(250, 245, 225),
        background_right: Color::Rgb(253, 246, 227),

        editor_line_number: Color::Rgb(147, 161, 161),
        editor_line_number_cursor: Color::Rgb(38, 139, 210),
        editor_separator: Color::Rgb(147, 161, 161),
        editor_cursor_char_bg: Color::Rgb(38, 139, 210),
        editor_cursor_char_fg: Color::Rgb(253, 246, 227),
        editor_cursor_line_bg: Color::Rgb(238, 232, 213),

        file_tree_added: Color::Rgb(133, 153, 0),
        file_tree_deleted: Color::Rgb(220, 50, 47),
        file_tree_modified: Color::Rgb(181, 137, 0),
        file_tree_renamed: Color::Rgb(38, 139, 210),
        file_tree_directory: Color::Rgb(42, 161, 152),
        file_tree_current_file_bg: Color::Rgb(238, 232, 213),
        file_tree_current_file_fg: Color::Rgb(7, 54, 66),
        file_tree_default: Color::Rgb(101, 123, 131),
        file_tree_stats_added: Color::Rgb(133, 153, 0),
        file_tree_stats_deleted: Color::Rgb(220, 50, 47),

        terminal_command: Color::Rgb(7, 54, 66),
        terminal_output: Color::Rgb(147, 161, 161),
        terminal_cursor_bg: Color::Rgb(38, 139, 210),
        terminal_cursor_fg: Color::Rgb(253, 246, 227),

        status_hash: Color::Rgb(181, 137, 0),
        status_author: Color::Rgb(133, 153, 0),
        status_date: Color::Rgb(38, 139, 210),
        status_message: Color::Rgb(7, 54, 66),
        status_no_commit: Color::Rgb(147, 161, 161),

        separator: Color::Rgb(147, 161, 161),

        syntax_keyword: Color::Rgb(203, 75, 22),
        syntax_type: Color::Rgb(181, 137, 0),
        syntax_function: Color::Rgb(38, 139, 210),
        syntax_variable: Color::Rgb(101, 123, 131),
        syntax_string: Color::Rgb(42, 161, 152),
        syntax_number: Color::Rgb(108, 113, 196),
        syntax_comment: Color::Rgb(147, 161, 161),
        syntax_operator: Color::Rgb(203, 75, 22),
        syntax_punctuation: Color::Rgb(101, 123, 131),
        syntax_constant: Color::Rgb(108, 113, 196),
        syntax_parameter: Color::Rgb(181, 137, 0),
        syntax_property: Color::Rgb(42, 161, 152),
        syntax_label: Color::Rgb(211, 54, 130),
    }
}
