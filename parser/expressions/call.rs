use crate::{
  Error,
  Parser,
  Precedence,
  tokens::{
    Signs,
    Token,
  },
};

use super::{
  Expression,
  Expressions,
  parse_expression,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Call {
  pub token: Token,
  pub arguments: Vec<Box<Expressions>>,
  pub semicolon: Option<Token>,
}

impl Expression for Call {
  fn new() -> Call {
    Call {
      token: Token::new_empty(),
      arguments: Vec::new(),
      semicolon: None,
    }
  }

  fn from_token(token: Token) -> Call {
    let mut call: Call = Expression::new();

    call.token = token;

    call
  }

  fn string(&self) -> String {
    let mut arguments: Vec<String> = Vec::new();

    for argument in self.arguments.iter() {
      arguments.push(argument.clone().string());
    }

    format!(
      "{}({}){}",
      self.token.value,
      arguments.join(", "),
      match self.semicolon.clone() {
        Some(semicolon) => semicolon.value,
        None => String::new(),
      },
    )
  }
}

impl Call {
  pub fn parse<'a>(
    parser: &'a mut Parser,
    standard_library: bool,
    with_this: bool,
  ) -> Result<Box<Expressions>, Error> {
    let mut call: Call = Expression::from_token(parser.get_current_token());

    // Check if the next token is a left parentheses.
    if !parser.expect_token(Signs::new(Signs::LEFTPARENTHESES)) {
      return Err(Error::from_token(
        format!("expect `(`, got `{}` instead.", parser.get_next_token().value),
        parser.get_next_token(),
      ));
    }

    // Get the next token.
    parser.next_token();

    // Get all arguments.
    while !parser.current_token_is(Signs::new(Signs::RIGHTPARENTHESES)) {
      // Check if the current token is a comma.
      if parser.current_token_is(Signs::new(Signs::COMMA)) {
        // Get the next token.
        parser.next_token();
      }

      // Parse expression.
      match parse_expression(parser, Precedence::LOWEST, standard_library, with_this) {
        Ok(argument) => {
          call.arguments.push(argument);
        },
        Err(error) => {
          return Err(error);
        }
      }

      // Get the next token.
      parser.next_token();
    }

    // Check if the current token is a semicolon.
    if parser.next_token_is(Signs::new(Signs::SEMICOLON)) {
      // Get the next token.
      parser.next_token();

      call.semicolon = Some(parser.get_current_token());
    }

    // Return the call expression.
    Ok(Box::new(Expressions::CALL(call)))
  }
}
