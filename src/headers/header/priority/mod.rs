use macros::{DefaultTokenizer, HeaderExtImpl, HeaderNewType};

#[derive(HeaderNewType, DefaultTokenizer, HeaderExtImpl, Debug, PartialEq, Eq, Clone)]
pub struct Priority(String);
