//!
//! The for-loop statement.
//!

use crate::error::Error;
use crate::generator::llvm::Context as LLVMContext;
use crate::generator::ILLVMWritable;
use crate::lexer::lexeme::Lexeme;
use crate::lexer::Lexer;
use crate::parser::statement::block::Block;
use crate::parser::statement::expression::Expression;

///
/// The for-loop statement.
///
#[derive(Debug, PartialEq, Clone)]
pub struct ForLoop {
    /// The index variables initialization block.
    pub initializer: Block,
    /// The continue condition block.
    pub condition: Expression,
    /// The index variables mutating block.
    pub finalizer: Block,
    /// The loop body.
    pub body: Block,
}

impl ForLoop {
    ///
    /// The element parser, which acts like a constructor.
    ///
    pub fn parse(lexer: &mut Lexer, initial: Option<Lexeme>) -> Result<Self, Error> {
        let lexeme = crate::parser::take_or_next(initial, lexer)?;

        let initializer = Block::parse(lexer, Some(lexeme))?;

        let condition = Expression::parse(lexer, None)?;

        let finalizer = Block::parse(lexer, None)?;

        let body = Block::parse(lexer, None)?;

        Ok(Self {
            initializer,
            condition,
            finalizer,
            body,
        })
    }
}

impl ILLVMWritable for ForLoop {
    fn into_llvm(self, context: &mut LLVMContext) {
        self.initializer.into_llvm_local(context);

        let condition_block = context.append_basic_block("for.condition");
        let body_block = context.append_basic_block("for.body");
        let increment_block = context.append_basic_block("for.increment");
        let join_block = context.append_basic_block("for.join");

        context.build_unconditional_branch(condition_block);
        context.set_basic_block(condition_block);
        let condition_expression = self
            .condition
            .into_llvm(context)
            .expect("Always exists")
            .into_int_value();
        let condition = context.builder.build_int_truncate(
            condition_expression,
            context.integer_type(compiler_const::bitlength::BOOLEAN),
            "",
        );
        context
            .builder
            .build_conditional_branch(condition, body_block, join_block);

        context.push_loop(body_block, increment_block, join_block);

        context.set_basic_block(body_block);
        self.body.into_llvm_local(context);
        context.build_unconditional_branch(increment_block);

        context.set_basic_block(increment_block);
        self.finalizer.into_llvm_local(context);
        context.build_unconditional_branch(condition_block);

        context.pop_loop();
        context.set_basic_block(join_block);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn ok_empty() {
        let input = r#"object "Test" { code {
            for {} expr {} {}
        }}"#;

        assert!(crate::parse(input).is_ok());
    }

    #[test]
    fn ok_complex() {
        let input = r#"object "Test" { code {
            function foo() -> x {
                x := 0
                for { let i := 0 } lt(i, 10) { i := add(i, 1) } {
                    x := add(i, x)
                }
            }
        }}"#;

        assert!(crate::parse(input).is_ok());
    }
}
