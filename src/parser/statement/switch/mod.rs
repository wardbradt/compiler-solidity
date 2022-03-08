//!
//! The switch statement.
//!

pub mod case;

use crate::error::Error;
use crate::lexer::lexeme::keyword::Keyword;
use crate::lexer::lexeme::Lexeme;
use crate::lexer::Lexer;
use crate::parser::error::Error as ParserError;
use crate::parser::statement::block::Block;
use crate::parser::statement::expression::Expression;

use self::case::Case;

///
/// The switch statement.
///
#[derive(Debug, PartialEq, Clone)]
pub struct Switch {
    /// The expression being matched.
    pub expression: Expression,
    /// The non-default cases.
    pub cases: Vec<Case>,
    /// The optional default case, if `cases` do not cover all possible values.
    pub default: Option<Block>,
}

///
/// The parsing state.
///
pub enum State {
    /// After match expression.
    CaseOrDefaultKeyword,
    /// After `case`.
    CaseBlock,
    /// After `default`.
    DefaultBlock,
}

impl Switch {
    ///
    /// The element parser, which acts like a constructor.
    ///
    pub fn parse(lexer: &mut Lexer, initial: Option<Lexeme>) -> Result<Self, Error> {
        let lexeme = crate::parser::take_or_next(initial, lexer)?;
        let mut state = State::CaseOrDefaultKeyword;

        let expression = Expression::parse(lexer, Some(lexeme.clone()))?;
        let mut cases = Vec::new();
        let mut default = None;

        loop {
            match state {
                State::CaseOrDefaultKeyword => match lexer.peek()? {
                    Lexeme::Keyword(Keyword::Case) => state = State::CaseBlock,
                    Lexeme::Keyword(Keyword::Default) => state = State::DefaultBlock,
                    _ => break,
                },
                State::CaseBlock => {
                    lexer.next()?;
                    cases.push(Case::parse(lexer, None)?);
                    state = State::CaseOrDefaultKeyword;
                }
                State::DefaultBlock => {
                    lexer.next()?;
                    default = Some(Block::parse(lexer, None)?);
                    break;
                }
            }
        }

        if cases.is_empty() && default.is_none() {
            return Err(ParserError::expected_one_of(vec!["case", "default"], lexeme, None).into());
        }

        Ok(Self {
            expression,
            cases,
            default,
        })
    }
}

impl<D> compiler_llvm_context::WriteLLVM<D> for Switch
where
    D: compiler_llvm_context::Dependency,
{
    fn into_llvm(self, context: &mut compiler_llvm_context::Context<D>) -> anyhow::Result<()> {
        if self.cases.is_empty() {
            if let Some(block) = self.default {
                block.into_llvm(context)?;
            }
            return Ok(());
        }

        let join_block = context.append_basic_block("switch_join");

        let mut current_block = context.append_basic_block("switch_case_constant_1");
        context.build_unconditional_branch(current_block);

        for (index, case) in self.cases.into_iter().enumerate() {
            let expression_block =
                context.append_basic_block(format!("switch_case_branch_{}", index + 1).as_str());
            context.set_basic_block(expression_block);
            case.block.into_llvm(context)?;
            context.build_unconditional_branch(join_block);

            context.set_basic_block(current_block);
            let scrutinee = self
                .expression
                .clone()
                .into_llvm(context)?
                .expect("Always exists");
            let constant = case.literal.into_llvm(context);
            let comparison = context.builder().build_int_compare(
                inkwell::IntPredicate::EQ,
                constant.to_llvm().into_int_value(),
                scrutinee.to_llvm().into_int_value(),
                format!("switch_case_condition_{}", index).as_str(),
            );

            let next_block =
                context.append_basic_block(format!("switch_case_constant_{}", index + 2).as_str());
            context.build_conditional_branch(comparison, expression_block, next_block);
            current_block = next_block;
        }

        context.set_basic_block(current_block);
        if let Some(block) = self.default {
            block.into_llvm(context)?;
        }
        context.build_unconditional_branch(join_block);

        context.set_basic_block(join_block);

        Ok(())
    }
}
