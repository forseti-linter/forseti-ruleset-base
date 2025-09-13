mod no_trailing_whitespace;
mod max_line_length;
mod no_empty_files;
mod require_final_newline;

pub use no_trailing_whitespace::NoTrailingWhitespaceRule;
pub use max_line_length::MaxLineLengthRule;
pub use no_empty_files::NoEmptyFilesRule;
pub use require_final_newline::RequireFinalNewlineRule;