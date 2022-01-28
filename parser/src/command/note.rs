use crate::error::Error;
use crate::token::{Token, Tokenizer};
use std::fmt;

#[derive(PartialEq, Eq, Debug)]
pub enum NoteCommand {
    Summary { title: String },
}

#[derive(PartialEq, Eq, Debug)]
pub enum ParseError {
    MissingTitle,
}
impl std::error::Error for ParseError {}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::MissingTitle => write!(f, "missing required summary title"),
        }
    }
}

impl NoteCommand {
    pub fn parse<'a>(input: &mut Tokenizer<'a>) -> Result<Option<Self>, Error<'a>> {
        let mut toks = input.clone();
        if let Some(Token::Word("note")) = toks.peek_token()? {
            toks.next_token()?;
            if let Some(Token::Word(title)) = toks.next_token()? {
                Ok(Some(NoteCommand::Summary {
                    title: title.to_string(),
                }))
            } else {
                Err(toks.error(ParseError::MissingTitle))
            }
        } else {
            Ok(None)
        }
    }
}
