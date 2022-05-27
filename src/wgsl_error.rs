

use naga::{front::wgsl::ParseError, valid::ValidationError, WithSpan, SourceLocation};

#[derive(Debug)]
pub enum WgslError {
    ValidationErr(WithSpan<ValidationError>),
    ParserErr {
        error: String,
        line: usize,
        pos: usize,
    },
    IoErr(std::io::Error),
}

impl From<std::io::Error> for WgslError {
    fn from(err: std::io::Error) -> Self {
        Self::IoErr(err)
    }
}

impl WgslError {
    pub fn from_parse_err(err: ParseError, src: &str) -> Self {
        let SourceLocation{line_number, line_position, offset: _, length: _ } = err.location(src).unwrap();
        let error = err.emit_to_string(src);
        Self::ParserErr { error, line:line_number as usize, pos:line_position as  usize}
        // Self::ParserErr { error, line:line_number as usize, pos:line_position as  usize}
    }
}
