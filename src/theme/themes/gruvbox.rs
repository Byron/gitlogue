use super::super::Theme;
use ratatui::style::Color;

/// Gruvbox Dark inspired color scheme
pub fn gruvbox() -> Theme {
    Theme {
        background_left: Color::Rgb(29, 32, 33),
        background_right: Color::Rgb(40, 40, 40),

        editor_line_number: Color::Rgb(146, 131, 116),
        editor_line_number_cursor: Color::Rgb(131, 165, 152),
        editor_separator: Color::Rgb(146, 131, 116),
        editor_cursor_char_bg: Color::Rgb(254, 128, 25),
        editor_cursor_char_fg: Color::Rgb(40, 40, 40),
        editor_cursor_line_bg: Color::Rgb(60, 56, 54),

        file_tree_added: Color::Rgb(184, 187, 38),
        file_tree_deleted: Color::Rgb(251, 73, 52),
        file_tree_modified: Color::Rgb(254, 128, 25),
        file_tree_renamed: Color::Rgb(131, 165, 152),
        file_tree_directory: Color::Rgb(131, 165, 152),
        file_tree_current_file_bg: Color::Rgb(60, 56, 54),
        file_tree_current_file_fg: Color::Rgb(235, 219, 178),
        file_tree_default: Color::Rgb(213, 196, 161),
        file_tree_stats_added: Color::Rgb(184, 187, 38),
        file_tree_stats_deleted: Color::Rgb(251, 73, 52),

        terminal_command: Color::Rgb(235, 219, 178),
        terminal_output: Color::Rgb(146, 131, 116),
        terminal_cursor_bg: Color::Rgb(254, 128, 25),
        terminal_cursor_fg: Color::Rgb(40, 40, 40),

        status_hash: Color::Rgb(250, 189, 47),
        status_author: Color::Rgb(184, 187, 38),
        status_date: Color::Rgb(131, 165, 152),
        status_message: Color::Rgb(235, 219, 178),
        status_no_commit: Color::Rgb(146, 131, 116),

        separator: Color::Rgb(146, 131, 116),

        syntax_keyword: Color::Rgb(251, 73, 52),
        syntax_type: Color::Rgb(250, 189, 47),
        syntax_function: Color::Rgb(184, 187, 38),
        syntax_variable: Color::Rgb(235, 219, 178),
        syntax_string: Color::Rgb(184, 187, 38),
        syntax_number: Color::Rgb(211, 134, 155),
        syntax_comment: Color::Rgb(146, 131, 116),
        syntax_operator: Color::Rgb(251, 73, 52),
        syntax_punctuation: Color::Rgb(213, 196, 161),
        syntax_constant: Color::Rgb(211, 134, 155),
        syntax_parameter: Color::Rgb(254, 128, 25),
        syntax_property: Color::Rgb(184, 187, 38),
        syntax_label: Color::Rgb(251, 73, 52),
    }
}
