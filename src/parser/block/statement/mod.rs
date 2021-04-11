//!
//! The block statement.
//!

pub mod assignment;
pub mod expression;
pub mod for_loop;
pub mod function_definition;
pub mod if_conditional;
pub mod switch;
pub mod variable_declaration;

use crate::error::Error;
use crate::lexer::lexeme::keyword::Keyword;
use crate::lexer::lexeme::Lexeme;
use crate::lexer::Lexer;
use crate::parser::block::Block;
use crate::parser::error::Error as ParserError;

use self::assignment::Assignment;
use self::expression::Expression;
use self::for_loop::ForLoop;
use self::function_definition::FunctionDefinition;
use self::if_conditional::IfConditional;
use self::switch::Switch;
use self::variable_declaration::VariableDeclaration;

///
/// The block statement.
///
#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    /// The code block.
    Block(Block),
    /// The expression.
    Expression(Expression),
    /// The `function` statement.
    FunctionDefinition(FunctionDefinition),
    /// The `let` statement.
    VariableDeclaration(VariableDeclaration),
    /// The `:=` existing variables reassignment statement.
    Assignment(Assignment),
    /// The `if` statement.
    IfConditional(IfConditional),
    /// The `switch` statement.
    Switch(Switch),
    /// The `for` statement.
    ForLoop(ForLoop),
    /// The `continue` statement.
    Continue,
    /// The `break` statement.
    Break,
    /// The `leave` statement.
    Leave,
}

impl Statement {
    ///
    /// The element parser, which acts like a constructor.
    ///
    pub fn parse(lexer: &mut Lexer, initial: Option<Lexeme>) -> Result<Self, Error> {
        let lexeme = crate::parser::take_or_next(initial, lexer)?;

        match lexeme {
            Lexeme::Keyword(Keyword::Function) => Ok(Statement::FunctionDefinition(
                FunctionDefinition::parse(lexer, None)?,
            )),
            Lexeme::Keyword(Keyword::Let) => Ok(Statement::VariableDeclaration(
                VariableDeclaration::parse(lexer, None)?,
            )),
            Lexeme::Keyword(Keyword::If) => {
                Ok(Statement::IfConditional(IfConditional::parse(lexer, None)?))
            }
            Lexeme::Keyword(Keyword::Switch) => Ok(Statement::Switch(Switch::parse(lexer, None)?)),
            Lexeme::Keyword(Keyword::For) => Ok(Statement::ForLoop(ForLoop::parse(lexer, None)?)),
            Lexeme::Keyword(Keyword::Continue) => Ok(Statement::Continue),
            Lexeme::Keyword(Keyword::Break) => Ok(Statement::Break),
            Lexeme::Keyword(Keyword::Leave) => Ok(Statement::Leave),
            lexeme => Err(ParserError::expected_one_of(
                vec![
                    "function", "let", "if", "switch", "for", "break", "continue", "leave",
                ],
                lexeme,
                None,
            )
            .into()),
        }
    }

    ///
    /// Converts the statement into a block.
    ///
    /// # Panics
    /// If there statement is not a block.
    ///
    pub fn into_block(self) -> Block {
        match self {
            Self::Block(block) => block,
            _ => panic!("Expected block"),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn leave_should_compile() {
        let input = r#"{
            function foo() -> x {
                x := 42
                if lt(x, 55) {
                    leave
                }
                x := 43
            }
        }"#;

        assert!(crate::parse(input).is_ok());
    }

    #[test]
    fn continue_should_compile() {
        let input = r#"{
            function foo() -> x {
                x := 0
                for { let i := 0 } lt(i, 10) { i := add(i, 1) } {
                    if mod(i, 2) {
                        continue
                    }
                    x := add(i, x)
                }
            }
        }"#;

        assert!(crate::parse(input).is_ok());
    }

    #[test]
    fn break_should_compile() {
        let input = r#"{
            function foo() -> x {
                x:= 0
                for { let i := 0 } lt(i, 10) { i := add(i, 1) } {
                    if gt(x, 18) {
                        break
                    }
                    x := add(i, x)
                }
            }
        }"#;

        assert!(crate::parse(input).is_ok());
    }
}
