use super::super::Theme;
use ratatui::style::Color;

/// Everforest Dark inspired color scheme
pub fn everforest() -> Theme {
    Theme {
        background_left: Color::Rgb(41, 48, 42),
        background_right: Color::Rgb(45, 52, 46),

        editor_line_number: Color::Rgb(125, 135, 116),
        editor_line_number_cursor: Color::Rgb(131, 192, 146),
        editor_separator: Color::Rgb(125, 135, 116),
        editor_cursor_char_bg: Color::Rgb(131, 192, 146),
        editor_cursor_char_fg: Color::Rgb(45, 52, 46),
        editor_cursor_line_bg: Color::Rgb(57, 64, 58),

        file_tree_added: Color::Rgb(131, 192, 146),
        file_tree_deleted: Color::Rgb(230, 126, 128),
        file_tree_modified: Color::Rgb(219, 188, 127),
        file_tree_renamed: Color::Rgb(125, 192, 192),
        file_tree_directory: Color::Rgb(125, 192, 192),
        file_tree_current_file_bg: Color::Rgb(57, 64, 58),
        file_tree_current_file_fg: Color::Rgb(211, 198, 170),
        file_tree_default: Color::Rgb(211, 198, 170),
        file_tree_stats_added: Color::Rgb(131, 192, 146),
        file_tree_stats_deleted: Color::Rgb(230, 126, 128),

        terminal_command: Color::Rgb(211, 198, 170),
        terminal_output: Color::Rgb(125, 135, 116),
        terminal_cursor_bg: Color::Rgb(131, 192, 146),
        terminal_cursor_fg: Color::Rgb(45, 52, 46),

        status_hash: Color::Rgb(219, 188, 127),
        status_author: Color::Rgb(131, 192, 146),
        status_date: Color::Rgb(125, 192, 192),
        status_message: Color::Rgb(211, 198, 170),
        status_no_commit: Color::Rgb(125, 135, 116),

        separator: Color::Rgb(125, 135, 116),

        syntax_keyword: Color::Rgb(230, 126, 128),
        syntax_type: Color::Rgb(219, 188, 127),
        syntax_function: Color::Rgb(131, 192, 146),
        syntax_variable: Color::Rgb(211, 198, 170),
        syntax_string: Color::Rgb(219, 188, 127),
        syntax_number: Color::Rgb(211, 134, 155),
        syntax_comment: Color::Rgb(125, 135, 116),
        syntax_operator: Color::Rgb(230, 126, 128),
        syntax_punctuation: Color::Rgb(180, 166, 137),
        syntax_constant: Color::Rgb(211, 134, 155),
        syntax_parameter: Color::Rgb(219, 188, 127),
        syntax_property: Color::Rgb(125, 192, 192),
        syntax_label: Color::Rgb(230, 126, 128),
    }
}
