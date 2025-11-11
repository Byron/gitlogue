use super::super::Theme;
use ratatui::style::Color;

/// Catppuccin Mocha inspired color scheme
pub fn catppuccin() -> Theme {
    Theme {
        background_left: Color::Rgb(24, 24, 37),
        background_right: Color::Rgb(30, 30, 46),

        editor_line_number: Color::Rgb(108, 112, 134),
        editor_line_number_cursor: Color::Rgb(137, 180, 250),
        editor_separator: Color::Rgb(108, 112, 134),
        editor_cursor_char_bg: Color::Rgb(245, 194, 231),
        editor_cursor_char_fg: Color::Rgb(30, 30, 46),
        editor_cursor_line_bg: Color::Rgb(49, 50, 68),

        file_tree_added: Color::Rgb(166, 227, 161),
        file_tree_deleted: Color::Rgb(243, 139, 168),
        file_tree_modified: Color::Rgb(250, 179, 135),
        file_tree_renamed: Color::Rgb(137, 180, 250),
        file_tree_directory: Color::Rgb(203, 166, 247),
        file_tree_current_file_bg: Color::Rgb(49, 50, 68),
        file_tree_current_file_fg: Color::Rgb(205, 214, 244),
        file_tree_default: Color::Rgb(205, 214, 244),
        file_tree_stats_added: Color::Rgb(166, 227, 161),
        file_tree_stats_deleted: Color::Rgb(243, 139, 168),

        terminal_command: Color::Rgb(205, 214, 244),
        terminal_output: Color::Rgb(108, 112, 134),
        terminal_cursor_bg: Color::Rgb(245, 194, 231),
        terminal_cursor_fg: Color::Rgb(30, 30, 46),

        status_hash: Color::Rgb(249, 226, 175),
        status_author: Color::Rgb(166, 227, 161),
        status_date: Color::Rgb(137, 180, 250),
        status_message: Color::Rgb(205, 214, 244),
        status_no_commit: Color::Rgb(108, 112, 134),

        separator: Color::Rgb(108, 112, 134),

        syntax_keyword: Color::Rgb(203, 166, 247),
        syntax_type: Color::Rgb(249, 226, 175),
        syntax_function: Color::Rgb(137, 180, 250),
        syntax_variable: Color::Rgb(205, 214, 244),
        syntax_string: Color::Rgb(166, 227, 161),
        syntax_number: Color::Rgb(250, 179, 135),
        syntax_comment: Color::Rgb(108, 112, 134),
        syntax_operator: Color::Rgb(148, 226, 213),
        syntax_punctuation: Color::Rgb(186, 194, 222),
        syntax_constant: Color::Rgb(250, 179, 135),
        syntax_parameter: Color::Rgb(245, 194, 231),
        syntax_property: Color::Rgb(166, 227, 161),
        syntax_label: Color::Rgb(203, 166, 247),
    }
}
