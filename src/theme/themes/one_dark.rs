use super::super::Theme;
use ratatui::style::Color;

/// One Dark inspired color scheme
pub fn one_dark() -> Theme {
    Theme {
        background_left: Color::Rgb(33, 37, 43),
        background_right: Color::Rgb(40, 44, 52),

        editor_line_number: Color::Rgb(92, 99, 112),
        editor_line_number_cursor: Color::Rgb(97, 175, 239),
        editor_separator: Color::Rgb(92, 99, 112),
        editor_cursor_char_bg: Color::Rgb(97, 175, 239),
        editor_cursor_char_fg: Color::Rgb(40, 44, 52),
        editor_cursor_line_bg: Color::Rgb(47, 52, 61),

        file_tree_added: Color::Rgb(152, 195, 121),
        file_tree_deleted: Color::Rgb(224, 108, 117),
        file_tree_modified: Color::Rgb(209, 154, 102),
        file_tree_renamed: Color::Rgb(97, 175, 239),
        file_tree_directory: Color::Rgb(97, 175, 239),
        file_tree_current_file_bg: Color::Rgb(47, 52, 61),
        file_tree_current_file_fg: Color::Rgb(220, 223, 228),
        file_tree_default: Color::Rgb(171, 178, 191),
        file_tree_stats_added: Color::Rgb(152, 195, 121),
        file_tree_stats_deleted: Color::Rgb(224, 108, 117),

        terminal_command: Color::Rgb(220, 223, 228),
        terminal_output: Color::Rgb(92, 99, 112),
        terminal_cursor_bg: Color::Rgb(97, 175, 239),
        terminal_cursor_fg: Color::Rgb(40, 44, 52),

        status_hash: Color::Rgb(229, 192, 123),
        status_author: Color::Rgb(152, 195, 121),
        status_date: Color::Rgb(97, 175, 239),
        status_message: Color::Rgb(220, 223, 228),
        status_no_commit: Color::Rgb(92, 99, 112),

        separator: Color::Rgb(92, 99, 112),

        syntax_keyword: Color::Rgb(198, 120, 221),
        syntax_type: Color::Rgb(229, 192, 123),
        syntax_function: Color::Rgb(97, 175, 239),
        syntax_variable: Color::Rgb(220, 223, 228),
        syntax_string: Color::Rgb(152, 195, 121),
        syntax_number: Color::Rgb(209, 154, 102),
        syntax_comment: Color::Rgb(92, 99, 112),
        syntax_operator: Color::Rgb(198, 120, 221),
        syntax_punctuation: Color::Rgb(171, 178, 191),
        syntax_constant: Color::Rgb(209, 154, 102),
        syntax_parameter: Color::Rgb(229, 192, 123),
        syntax_property: Color::Rgb(152, 195, 121),
        syntax_label: Color::Rgb(198, 120, 221),
    }
}
