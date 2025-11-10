use anyhow::{Context, Result};
use ratatui::style::Color;

#[derive(Debug, Clone)]
pub struct Theme {
    // Background colors
    pub background_left: Color,  // FileTree and StatusBar side (darker)
    pub background_right: Color, // Editor and Terminal side

    // Editor colors
    pub editor_line_number: Color,
    pub editor_line_number_cursor: Color,
    pub editor_separator: Color,
    pub editor_cursor_char_bg: Color,
    pub editor_cursor_char_fg: Color,
    pub editor_cursor_line_bg: Color,

    // File tree colors
    pub file_tree_added: Color,
    pub file_tree_deleted: Color,
    pub file_tree_modified: Color,
    pub file_tree_renamed: Color,
    pub file_tree_directory: Color,
    pub file_tree_current_file_bg: Color,
    pub file_tree_current_file_fg: Color,
    pub file_tree_default: Color,
    pub file_tree_stats_added: Color,
    pub file_tree_stats_deleted: Color,

    // Terminal colors
    pub terminal_command: Color,
    pub terminal_output: Color,
    pub terminal_cursor_bg: Color,
    pub terminal_cursor_fg: Color,

    // Status bar colors
    pub status_hash: Color,
    pub status_author: Color,
    pub status_date: Color,
    pub status_message: Color,
    pub status_no_commit: Color,

    // Separator colors
    pub separator: Color,

    // Syntax highlighting colors
    pub syntax_keyword: Color,
    pub syntax_type: Color,
    pub syntax_function: Color,
    pub syntax_variable: Color,
    pub syntax_string: Color,
    pub syntax_number: Color,
    pub syntax_comment: Color,
    pub syntax_operator: Color,
    pub syntax_punctuation: Color,
    pub syntax_constant: Color,
    pub syntax_parameter: Color,
    pub syntax_property: Color,
    pub syntax_label: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self::tokyo_night()
    }
}

impl Theme {
    /// Tokyo Night inspired color scheme
    pub fn tokyo_night() -> Self {
        Self {
            // Background colors
            background_left: Color::Rgb(30, 34, 54), // Darker for left side
            background_right: Color::Rgb(26, 27, 38), // Base background

            // Editor colors
            editor_line_number: Color::Rgb(86, 95, 137),
            editor_line_number_cursor: Color::Rgb(125, 207, 255), // Cyan
            editor_separator: Color::Rgb(86, 95, 137),
            editor_cursor_char_bg: Color::Rgb(122, 162, 247),
            editor_cursor_char_fg: Color::Rgb(26, 27, 38),
            editor_cursor_line_bg: Color::Rgb(42, 47, 68),

            // File tree colors
            file_tree_added: Color::Rgb(158, 206, 106), // Green
            file_tree_deleted: Color::Rgb(247, 118, 142), // Red
            file_tree_modified: Color::Rgb(255, 158, 100), // Orange
            file_tree_renamed: Color::Rgb(122, 162, 247), // Blue
            file_tree_directory: Color::Rgb(122, 162, 247), // Blue
            file_tree_current_file_bg: Color::Rgb(42, 47, 68),
            file_tree_current_file_fg: Color::Rgb(192, 202, 245),
            file_tree_default: Color::Rgb(192, 202, 245),
            file_tree_stats_added: Color::Rgb(158, 206, 106), // Green
            file_tree_stats_deleted: Color::Rgb(247, 118, 142), // Red

            // Terminal colors
            terminal_command: Color::Rgb(192, 202, 245), // Light foreground
            terminal_output: Color::Rgb(86, 95, 137),    // Muted gray (less prominent)
            terminal_cursor_bg: Color::Rgb(122, 162, 247),
            terminal_cursor_fg: Color::Rgb(26, 27, 38),

            // Status bar colors
            status_hash: Color::Rgb(255, 213, 128),   // Yellow
            status_author: Color::Rgb(158, 206, 106), // Green
            status_date: Color::Rgb(122, 162, 247),   // Blue
            status_message: Color::Rgb(192, 202, 245),
            status_no_commit: Color::Rgb(86, 95, 137), // Muted gray

            // Separator colors
            separator: Color::Rgb(86, 95, 137),

            // Syntax highlighting colors (Tokyo Night inspired)
            syntax_keyword: Color::Rgb(187, 154, 247), // Purple
            syntax_type: Color::Rgb(125, 207, 255),    // Cyan
            syntax_function: Color::Rgb(122, 162, 247), // Blue
            syntax_variable: Color::Rgb(192, 202, 245), // Light foreground
            syntax_string: Color::Rgb(158, 206, 106),  // Green
            syntax_number: Color::Rgb(255, 158, 100),  // Orange
            syntax_comment: Color::Rgb(86, 95, 137),   // Muted blue-gray
            syntax_operator: Color::Rgb(125, 207, 255), // Cyan
            syntax_punctuation: Color::Rgb(140, 148, 184), // Light gray-blue
            syntax_constant: Color::Rgb(255, 158, 100), // Orange
            syntax_parameter: Color::Rgb(255, 213, 128), // Yellow
            syntax_property: Color::Rgb(158, 206, 106), // Green
            syntax_label: Color::Rgb(187, 154, 247),   // Purple
        }
    }

    /// Dracula inspired color scheme
    pub fn dracula() -> Self {
        Self {
            background_left: Color::Rgb(33, 34, 44),
            background_right: Color::Rgb(40, 42, 54),

            editor_line_number: Color::Rgb(98, 114, 164),
            editor_line_number_cursor: Color::Rgb(139, 233, 253),
            editor_separator: Color::Rgb(98, 114, 164),
            editor_cursor_char_bg: Color::Rgb(255, 121, 198),
            editor_cursor_char_fg: Color::Rgb(40, 42, 54),
            editor_cursor_line_bg: Color::Rgb(68, 71, 90),

            file_tree_added: Color::Rgb(80, 250, 123),
            file_tree_deleted: Color::Rgb(255, 85, 85),
            file_tree_modified: Color::Rgb(255, 184, 108),
            file_tree_renamed: Color::Rgb(139, 233, 253),
            file_tree_directory: Color::Rgb(189, 147, 249),
            file_tree_current_file_bg: Color::Rgb(68, 71, 90),
            file_tree_current_file_fg: Color::Rgb(248, 248, 242),
            file_tree_default: Color::Rgb(248, 248, 242),
            file_tree_stats_added: Color::Rgb(80, 250, 123),
            file_tree_stats_deleted: Color::Rgb(255, 85, 85),

            terminal_command: Color::Rgb(248, 248, 242),
            terminal_output: Color::Rgb(98, 114, 164),
            terminal_cursor_bg: Color::Rgb(255, 121, 198),
            terminal_cursor_fg: Color::Rgb(40, 42, 54),

            status_hash: Color::Rgb(241, 250, 140),
            status_author: Color::Rgb(80, 250, 123),
            status_date: Color::Rgb(139, 233, 253),
            status_message: Color::Rgb(248, 248, 242),
            status_no_commit: Color::Rgb(98, 114, 164),

            separator: Color::Rgb(98, 114, 164),

            syntax_keyword: Color::Rgb(255, 121, 198),
            syntax_type: Color::Rgb(139, 233, 253),
            syntax_function: Color::Rgb(80, 250, 123),
            syntax_variable: Color::Rgb(248, 248, 242),
            syntax_string: Color::Rgb(241, 250, 140),
            syntax_number: Color::Rgb(189, 147, 249),
            syntax_comment: Color::Rgb(98, 114, 164),
            syntax_operator: Color::Rgb(255, 121, 198),
            syntax_punctuation: Color::Rgb(248, 248, 242),
            syntax_constant: Color::Rgb(189, 147, 249),
            syntax_parameter: Color::Rgb(255, 184, 108),
            syntax_property: Color::Rgb(80, 250, 123),
            syntax_label: Color::Rgb(255, 121, 198),
        }
    }

    /// Nord inspired color scheme
    pub fn nord() -> Self {
        Self {
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

    /// Solarized Dark color scheme
    pub fn solarized_dark() -> Self {
        Self {
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

    /// Solarized Light color scheme
    pub fn solarized_light() -> Self {
        Self {
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

    /// Monokai inspired color scheme
    pub fn monokai() -> Self {
        Self {
            background_left: Color::Rgb(30, 30, 30),
            background_right: Color::Rgb(39, 40, 34),

            editor_line_number: Color::Rgb(117, 113, 94),
            editor_line_number_cursor: Color::Rgb(102, 217, 239),
            editor_separator: Color::Rgb(117, 113, 94),
            editor_cursor_char_bg: Color::Rgb(253, 151, 31),
            editor_cursor_char_fg: Color::Rgb(39, 40, 34),
            editor_cursor_line_bg: Color::Rgb(51, 51, 45),

            file_tree_added: Color::Rgb(166, 226, 46),
            file_tree_deleted: Color::Rgb(249, 38, 114),
            file_tree_modified: Color::Rgb(253, 151, 31),
            file_tree_renamed: Color::Rgb(102, 217, 239),
            file_tree_directory: Color::Rgb(174, 129, 255),
            file_tree_current_file_bg: Color::Rgb(51, 51, 45),
            file_tree_current_file_fg: Color::Rgb(248, 248, 242),
            file_tree_default: Color::Rgb(248, 248, 242),
            file_tree_stats_added: Color::Rgb(166, 226, 46),
            file_tree_stats_deleted: Color::Rgb(249, 38, 114),

            terminal_command: Color::Rgb(248, 248, 242),
            terminal_output: Color::Rgb(117, 113, 94),
            terminal_cursor_bg: Color::Rgb(253, 151, 31),
            terminal_cursor_fg: Color::Rgb(39, 40, 34),

            status_hash: Color::Rgb(230, 219, 116),
            status_author: Color::Rgb(166, 226, 46),
            status_date: Color::Rgb(102, 217, 239),
            status_message: Color::Rgb(248, 248, 242),
            status_no_commit: Color::Rgb(117, 113, 94),

            separator: Color::Rgb(117, 113, 94),

            syntax_keyword: Color::Rgb(249, 38, 114),
            syntax_type: Color::Rgb(102, 217, 239),
            syntax_function: Color::Rgb(166, 226, 46),
            syntax_variable: Color::Rgb(248, 248, 242),
            syntax_string: Color::Rgb(230, 219, 116),
            syntax_number: Color::Rgb(174, 129, 255),
            syntax_comment: Color::Rgb(117, 113, 94),
            syntax_operator: Color::Rgb(249, 38, 114),
            syntax_punctuation: Color::Rgb(248, 248, 242),
            syntax_constant: Color::Rgb(174, 129, 255),
            syntax_parameter: Color::Rgb(253, 151, 31),
            syntax_property: Color::Rgb(166, 226, 46),
            syntax_label: Color::Rgb(249, 38, 114),
        }
    }

    /// One Dark inspired color scheme
    pub fn one_dark() -> Self {
        Self {
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

    /// Gruvbox Dark inspired color scheme
    pub fn gruvbox() -> Self {
        Self {
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

    /// Catppuccin Mocha inspired color scheme
    pub fn catppuccin() -> Self {
        Self {
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

    /// Load theme by name
    pub fn load(name: &str) -> Result<Self> {
        match name {
            "tokyo-night" => Ok(Self::tokyo_night()),
            "dracula" => Ok(Self::dracula()),
            "nord" => Ok(Self::nord()),
            "solarized-dark" => Ok(Self::solarized_dark()),
            "solarized-light" => Ok(Self::solarized_light()),
            "monokai" => Ok(Self::monokai()),
            "one-dark" => Ok(Self::one_dark()),
            "gruvbox" => Ok(Self::gruvbox()),
            "catppuccin" => Ok(Self::catppuccin()),
            _ => Err(anyhow::anyhow!("Unknown theme: {}", name))
                .context("Available themes: tokyo-night, dracula, nord, solarized-dark, solarized-light, monokai, one-dark, gruvbox, catppuccin"),
        }
    }

    /// List all available built-in themes
    pub fn available_themes() -> Vec<&'static str> {
        vec![
            "tokyo-night",
            "dracula",
            "nord",
            "solarized-dark",
            "solarized-light",
            "monokai",
            "one-dark",
            "gruvbox",
            "catppuccin",
        ]
    }
}
