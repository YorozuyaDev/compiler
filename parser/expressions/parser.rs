use crate::{Parser, Precedence};
use crate::tokens::*;

use super::*;

pub fn parse<'a>(parser: &'a mut Parser, precedence: Precedence) -> Option<Box<Expressions>> {
  let current_token: Token = parser.current_token.clone();
  let mut expression: Option<Box<Expressions>> = None;

  // Parse identifiers.
  if current_token.token.clone().is_identifier() {
    expression = Some(Identifier::new_box_from_token(current_token.clone()));
  }

  // Parse strings.
  if current_token.token.clone().is_string() {
    expression = Some(StringE::new_box_from_token(current_token.clone()));
  }

  // Parse numbers.
  if current_token.token.clone().is_number() {
    expression = Number::parse(parser);
  }

  // Parse booleans.
  if current_token.token.clone().is_keyword(Keywords::TRUE) || current_token.token.clone().is_keyword(Keywords::FALSE) {
    expression = Some(Boolean::parse(parser));
  }

  // Parse infix expression.
  while !parser.next_token_is(Signs::new(Signs::SEMICOLON)) &&
    !parser.next_token_is(Box::new(Tokens::EOL)) &&
    !parser.next_token_is(Box::new(Tokens::EOF)) &&
    precedence <= parser.next_precedence()
  {
    let next_token: Box<Tokens> = parser.next_token.token.clone();

    // Parse Infix
    if next_token.clone().is_sign(Signs::PLUS) ||
      next_token.clone().is_sign(Signs::MINUS) ||
      next_token.clone().is_sign(Signs::DIVIDE) ||
      next_token.clone().is_sign(Signs::MULTIPLY) ||
      next_token.clone().is_sign(Signs::EMPOWERMENT) ||
      next_token.clone().is_sign(Signs::MODULE) ||
      next_token.clone().is_sign(Signs::EQUAL) ||
      next_token.clone().is_sign(Signs::EQUALTYPE) ||
      next_token.clone().is_sign(Signs::NOTEQUAL) ||
      next_token.clone().is_sign(Signs::NOTEQUALTYPE) ||
      next_token.clone().is_sign(Signs::LESSTHAN) ||
      next_token.clone().is_sign(Signs::LESSOREQUALTHAN) ||
      next_token.clone().is_sign(Signs::GREATERTHAN) ||
      next_token.clone().is_sign(Signs::GREATEROREQUALTHAN) {
      // Get the next token.
      parser.next_token();

      // Set the new expression.
      expression = Some(Infix::parse(parser, expression));

      continue;
    }
  }

  // Return expression.
  expression
}