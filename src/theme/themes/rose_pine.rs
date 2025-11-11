use super::super::Theme;
use ratatui::style::Color;

/// Rose Pine inspired color scheme
pub fn rose_pine() -> Theme {
    Theme {
        background_left: Color::Rgb(25, 23, 36),
        background_right: Color::Rgb(35, 33, 54),

        editor_line_number: Color::Rgb(110, 106, 134),
        editor_line_number_cursor: Color::Rgb(156, 207, 216),
        editor_separator: Color::Rgb(110, 106, 134),
        editor_cursor_char_bg: Color::Rgb(235, 188, 186),
        editor_cursor_char_fg: Color::Rgb(35, 33, 54),
        editor_cursor_line_bg: Color::Rgb(42, 39, 63),

        file_tree_added: Color::Rgb(156, 207, 216),
        file_tree_deleted: Color::Rgb(235, 111, 146),
        file_tree_modified: Color::Rgb(246, 193, 119),
        file_tree_renamed: Color::Rgb(196, 167, 231),
        file_tree_directory: Color::Rgb(196, 167, 231),
        file_tree_current_file_bg: Color::Rgb(42, 39, 63),
        file_tree_current_file_fg: Color::Rgb(224, 222, 244),
        file_tree_default: Color::Rgb(224, 222, 244),
        file_tree_stats_added: Color::Rgb(156, 207, 216),
        file_tree_stats_deleted: Color::Rgb(235, 111, 146),

        terminal_command: Color::Rgb(224, 222, 244),
        terminal_output: Color::Rgb(110, 106, 134),
        terminal_cursor_bg: Color::Rgb(235, 188, 186),
        terminal_cursor_fg: Color::Rgb(35, 33, 54),

        status_hash: Color::Rgb(246, 193, 119),
        status_author: Color::Rgb(156, 207, 216),
        status_date: Color::Rgb(196, 167, 231),
        status_message: Color::Rgb(224, 222, 244),
        status_no_commit: Color::Rgb(110, 106, 134),

        separator: Color::Rgb(110, 106, 134),

        syntax_keyword: Color::Rgb(196, 167, 231),
        syntax_type: Color::Rgb(246, 193, 119),
        syntax_function: Color::Rgb(156, 207, 216),
        syntax_variable: Color::Rgb(224, 222, 244),
        syntax_string: Color::Rgb(246, 193, 119),
        syntax_number: Color::Rgb(234, 154, 151),
        syntax_comment: Color::Rgb(110, 106, 134),
        syntax_operator: Color::Rgb(235, 111, 146),
        syntax_punctuation: Color::Rgb(144, 140, 170),
        syntax_constant: Color::Rgb(235, 188, 186),
        syntax_parameter: Color::Rgb(246, 193, 119),
        syntax_property: Color::Rgb(156, 207, 216),
        syntax_label: Color::Rgb(196, 167, 231),
    }
}
