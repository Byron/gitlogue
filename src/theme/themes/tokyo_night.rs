use super::super::Theme;
use ratatui::style::Color;

/// Tokyo Night inspired color scheme
pub fn tokyo_night() -> Theme {
    Theme {
        background_left: Color::Rgb(30, 34, 54),
        background_right: Color::Rgb(26, 27, 38),

        editor_line_number: Color::Rgb(86, 95, 137),
        editor_line_number_cursor: Color::Rgb(125, 207, 255),
        editor_separator: Color::Rgb(86, 95, 137),
        editor_cursor_char_bg: Color::Rgb(122, 162, 247),
        editor_cursor_char_fg: Color::Rgb(26, 27, 38),
        editor_cursor_line_bg: Color::Rgb(42, 47, 68),

        file_tree_added: Color::Rgb(158, 206, 106),
        file_tree_deleted: Color::Rgb(247, 118, 142),
        file_tree_modified: Color::Rgb(255, 158, 100),
        file_tree_renamed: Color::Rgb(122, 162, 247),
        file_tree_directory: Color::Rgb(122, 162, 247),
        file_tree_current_file_bg: Color::Rgb(42, 47, 68),
        file_tree_current_file_fg: Color::Rgb(192, 202, 245),
        file_tree_default: Color::Rgb(192, 202, 245),
        file_tree_stats_added: Color::Rgb(158, 206, 106),
        file_tree_stats_deleted: Color::Rgb(247, 118, 142),

        terminal_command: Color::Rgb(192, 202, 245),
        terminal_output: Color::Rgb(86, 95, 137),
        terminal_cursor_bg: Color::Rgb(122, 162, 247),
        terminal_cursor_fg: Color::Rgb(26, 27, 38),

        status_hash: Color::Rgb(255, 213, 128),
        status_author: Color::Rgb(158, 206, 106),
        status_date: Color::Rgb(122, 162, 247),
        status_message: Color::Rgb(192, 202, 245),
        status_no_commit: Color::Rgb(86, 95, 137),

        separator: Color::Rgb(86, 95, 137),

        syntax_keyword: Color::Rgb(187, 154, 247),
        syntax_type: Color::Rgb(125, 207, 255),
        syntax_function: Color::Rgb(122, 162, 247),
        syntax_variable: Color::Rgb(192, 202, 245),
        syntax_string: Color::Rgb(158, 206, 106),
        syntax_number: Color::Rgb(255, 158, 100),
        syntax_comment: Color::Rgb(86, 95, 137),
        syntax_operator: Color::Rgb(125, 207, 255),
        syntax_punctuation: Color::Rgb(140, 148, 184),
        syntax_constant: Color::Rgb(255, 158, 100),
        syntax_parameter: Color::Rgb(255, 213, 128),
        syntax_property: Color::Rgb(158, 206, 106),
        syntax_label: Color::Rgb(187, 154, 247),
    }
}
