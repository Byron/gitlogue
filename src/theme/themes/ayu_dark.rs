use super::super::Theme;
use ratatui::style::Color;

/// Ayu Dark inspired color scheme
pub fn ayu_dark() -> Theme {
    Theme {
        background_left: Color::Rgb(10, 14, 20),
        background_right: Color::Rgb(15, 20, 25),

        editor_line_number: Color::Rgb(62, 68, 82),
        editor_line_number_cursor: Color::Rgb(89, 182, 215),
        editor_separator: Color::Rgb(62, 68, 82),
        editor_cursor_char_bg: Color::Rgb(255, 180, 84),
        editor_cursor_char_fg: Color::Rgb(15, 20, 25),
        editor_cursor_line_bg: Color::Rgb(22, 29, 37),

        file_tree_added: Color::Rgb(186, 230, 126),
        file_tree_deleted: Color::Rgb(242, 97, 103),
        file_tree_modified: Color::Rgb(255, 180, 84),
        file_tree_renamed: Color::Rgb(89, 182, 215),
        file_tree_directory: Color::Rgb(89, 182, 215),
        file_tree_current_file_bg: Color::Rgb(22, 29, 37),
        file_tree_current_file_fg: Color::Rgb(230, 237, 243),
        file_tree_default: Color::Rgb(230, 237, 243),
        file_tree_stats_added: Color::Rgb(186, 230, 126),
        file_tree_stats_deleted: Color::Rgb(242, 97, 103),

        terminal_command: Color::Rgb(230, 237, 243),
        terminal_output: Color::Rgb(62, 68, 82),
        terminal_cursor_bg: Color::Rgb(255, 180, 84),
        terminal_cursor_fg: Color::Rgb(15, 20, 25),

        status_hash: Color::Rgb(229, 181, 103),
        status_author: Color::Rgb(186, 230, 126),
        status_date: Color::Rgb(89, 182, 215),
        status_message: Color::Rgb(230, 237, 243),
        status_no_commit: Color::Rgb(62, 68, 82),

        separator: Color::Rgb(62, 68, 82),

        syntax_keyword: Color::Rgb(255, 140, 99),
        syntax_type: Color::Rgb(229, 181, 103),
        syntax_function: Color::Rgb(255, 214, 111),
        syntax_variable: Color::Rgb(230, 237, 243),
        syntax_string: Color::Rgb(186, 230, 126),
        syntax_number: Color::Rgb(239, 158, 222),
        syntax_comment: Color::Rgb(92, 99, 112),
        syntax_operator: Color::Rgb(242, 151, 24),
        syntax_punctuation: Color::Rgb(230, 237, 243),
        syntax_constant: Color::Rgb(89, 182, 215),
        syntax_parameter: Color::Rgb(255, 214, 111),
        syntax_property: Color::Rgb(115, 184, 205),
        syntax_label: Color::Rgb(255, 140, 99),
    }
}
