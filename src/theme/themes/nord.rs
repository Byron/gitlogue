use super::super::Theme;
use ratatui::style::Color;

/// Nord inspired color scheme
pub fn nord() -> Theme {
    Theme {
        background_left: Color::Rgb(36, 42, 56),
        background_right: Color::Rgb(46, 52, 64),

        editor_line_number: Color::Rgb(76, 86, 106),
        editor_line_number_cursor: Color::Rgb(136, 192, 208),
        editor_separator: Color::Rgb(76, 86, 106),
        editor_cursor_char_bg: Color::Rgb(136, 192, 208),
        editor_cursor_char_fg: Color::Rgb(46, 52, 64),
        editor_cursor_line_bg: Color::Rgb(59, 66, 82),

        file_tree_added: Color::Rgb(163, 190, 140),
        file_tree_deleted: Color::Rgb(191, 97, 106),
        file_tree_modified: Color::Rgb(235, 203, 139),
        file_tree_renamed: Color::Rgb(129, 161, 193),
        file_tree_directory: Color::Rgb(136, 192, 208),
        file_tree_current_file_bg: Color::Rgb(59, 66, 82),
        file_tree_current_file_fg: Color::Rgb(236, 239, 244),
        file_tree_default: Color::Rgb(216, 222, 233),
        file_tree_stats_added: Color::Rgb(163, 190, 140),
        file_tree_stats_deleted: Color::Rgb(191, 97, 106),

        terminal_command: Color::Rgb(236, 239, 244),
        terminal_output: Color::Rgb(76, 86, 106),
        terminal_cursor_bg: Color::Rgb(136, 192, 208),
        terminal_cursor_fg: Color::Rgb(46, 52, 64),

        status_hash: Color::Rgb(235, 203, 139),
        status_author: Color::Rgb(163, 190, 140),
        status_date: Color::Rgb(129, 161, 193),
        status_message: Color::Rgb(236, 239, 244),
        status_no_commit: Color::Rgb(76, 86, 106),

        separator: Color::Rgb(76, 86, 106),

        syntax_keyword: Color::Rgb(180, 142, 173),
        syntax_type: Color::Rgb(136, 192, 208),
        syntax_function: Color::Rgb(136, 192, 208),
        syntax_variable: Color::Rgb(236, 239, 244),
        syntax_string: Color::Rgb(163, 190, 140),
        syntax_number: Color::Rgb(180, 142, 173),
        syntax_comment: Color::Rgb(76, 86, 106),
        syntax_operator: Color::Rgb(136, 192, 208),
        syntax_punctuation: Color::Rgb(216, 222, 233),
        syntax_constant: Color::Rgb(180, 142, 173),
        syntax_parameter: Color::Rgb(235, 203, 139),
        syntax_property: Color::Rgb(163, 190, 140),
        syntax_label: Color::Rgb(180, 142, 173),
    }
}
