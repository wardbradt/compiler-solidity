//!
//! The compiler test tools.
//!

use crate::lexer::lexeme::Lexeme;
use crate::lexer::Lexer;
use crate::parser::block::statement::expression::literal::Literal;
use crate::parser::block::statement::expression::Expression;
use crate::parser::block::statement::Statement;
use crate::parser::block::Block;
use crate::parser::Parser;

pub(crate) fn tokenize(input: &str) -> Vec<Lexeme> {
    let input = input.to_string();
    let mut lexer = Lexer::new(input);
    lexer.get_lexemes()
}

pub(crate) fn parse(input: &str) -> Vec<Statement> {
    Parser::parse(tokenize(input).into_iter())
}

pub(crate) fn compile(input: &str, run: &Option<&str>) -> u64 {
    let statements = parse(input);
    if statements.len() != 1 {
        panic!("Unparsed parts exist");
    }
    crate::generator::Compiler::compile(&statements[0], run)
}

#[test]
fn comment_should_not_be_parsed() {
    assert_eq!(
        parse("/*123 comment ***/{}"),
        [Statement::Block(Block { statements: vec![] })]
    );
}

#[test]
#[should_panic]
fn ill_formed_comment_should_panic() {
    parse("/* xxx yyy");
}

#[test]
#[should_panic]
fn badly_named_function_should_panic() {
    parse("{ function 42(){}}");
}

#[test]
#[should_panic]
fn function_with_bad_parameter_list_should_panic() {
    parse("{ function 42){}}");
}

#[test]
fn well_formed_void_function_should_be_parsed() {
    parse("{function foo(a : A, b){}}");
}

#[test]
fn well_formed_non_void_function_should_be_parsed() {
    parse("{function foo(a : A, b) -> x: T, z: Y {}}");
}

#[test]
fn vardecl_true_should_be_parsed() {
    parse("{let x := true}");
}

#[test]
fn vardecl_false_should_be_parsed() {
    parse("{let x := false}");
}

#[test]
fn vardecl_string_should_be_parsed() {
    parse("{let x := \"abc\"}");
}

#[test]
fn vardecl_dec_number_should_be_parsed() {
    parse("{let x := 42}");
}

#[test]
fn vardecl_hex_number_should_be_parsed() {
    parse("{let x := 0x42}");
}

#[test]
fn vardecl_identifier_should_be_parsed() {
    parse("{let x := y}");
}

#[test]
fn vardecl_function_call_should_be_parsed() {
    parse("{let x := foo()}");
    parse("{let x := foo(x, y)}");
    parse("{let x := foo(bar(x, baz()))}");
}

#[test]
fn if_statement_should_be_parsed() {
    parse("{if expr {}}");
}

#[test]
#[should_panic]
fn ill_formed_switch_statement_should_panic() {
    parse("{switch {}}");
    parse("{switch expr default {} case 3 {}}");
}

#[test]
fn for_loop_should_be_parsed() {
    parse("{for {} expr {}{}}");
}

#[test]
fn keywords_should_not_be_parsed_as_identifiers() {
    let kw_break = parse("{break}");
    let kw_continue = parse("{continue}");
    let kw_leave = parse("{leave}");
    assert_eq!(
        kw_break,
        [Statement::Block(Block {
            statements: vec![Statement::Break]
        })]
    );
    assert_eq!(
        kw_continue,
        [Statement::Block(Block {
            statements: vec![Statement::Continue]
        })]
    );
    assert_eq!(
        kw_leave,
        [Statement::Block(Block {
            statements: vec![Statement::Leave]
        })]
    );
}

#[test]
fn true_false_should_be_parsed_as_literals() {
    let kw_true = parse("{true}");
    let kw_false = parse("{false}");
    assert_eq!(
        kw_true,
        [Statement::Block(Block {
            statements: vec![Statement::Expression(Expression::Literal(Literal {
                value: "true".to_string()
            }))]
        })]
    );
    assert_eq!(
        kw_false,
        [Statement::Block(Block {
            statements: vec![Statement::Expression(Expression::Literal(Literal {
                value: "false".to_string()
            }))]
        })]
    );
}
#[test]
fn expressions_should_be_parsed() {
    parse("{id 3 foo(x, y)}");
}

#[test]
fn assignments_should_be_parsed() {
    parse("{x := foo(x)}");
    parse("{x,y := foo(x)}");
}

#[test]
#[should_panic]
fn ill_formed_assignment_should_panic() {
    parse("{x := }");
}

#[test]
#[should_panic]
fn id_list_wo_assignment_should_panic() {
    parse("{x,y}");
}

#[test]
fn void_function_should_compile() {
    compile("{function foo() {}}", &None);
}

#[test]
fn i256_function_should_compile() {
    compile("{function foo() -> x {}}", &None);
}

#[test]
fn literal_initialization_should_compile() {
    let result = compile("{function foo() -> x {let y := 5 x :=y }}", &Some("foo"));
    assert_eq!(result, 5);
    let result = compile("{function foo() -> x {let y := 1234567890123456789012345678 let z := 1234567890123456789012345679 x := sub(z, y) }}", &Some("foo"));
    assert_eq!(result, 1);
    let result = compile(
        "{function foo() -> x {let y := 0x2a x := y }}",
        &Some("foo"),
    );
    assert_eq!(result, 42);
    let result = compile("{function foo() -> x {let y := 0xffffffffffffffff let z := 0xfffffffffffffffe x := sub(y, z) }}", &Some("foo"));
    assert_eq!(result, 1);
}

#[test]
fn variable_initialization_should_compile() {
    compile("{function foo() -> x {let y := x}}", &None);
}

#[test]
fn builtin_call_should_compile() {
    let result = compile(
        "{function foo() -> x {let y := 3 x := add(3, y)}}",
        &Some("foo"),
    );
    assert_eq!(result, 6);
    let result = compile(
        "{function foo() -> x {let y := 3 x := sub(3, y)}}",
        &Some("foo"),
    );
    assert_eq!(result, 0);
    let result = compile(
        "{function foo() -> x {let y := 3 x := mul(3, y)}}",
        &Some("foo"),
    );
    assert_eq!(result, 9);
    let result = compile(
        "{function foo() -> x {let y := 3 x := div(3, y)}}",
        &Some("foo"),
    );
    assert_eq!(result, 1);
    let result = compile(
        "{function foo() -> x {let y := 3 x := sdiv(3, y)}}",
        &Some("foo"),
    );
    assert_eq!(result, 1);
    let result = compile(
        "{function foo() -> x {let y := 3 x := mod(3, y)}}",
        &Some("foo"),
    );
    assert_eq!(result, 0);
    let result = compile(
        "{function foo() -> x {let y := 3 x := smod(3, y)}}",
        &Some("foo"),
    );
    assert_eq!(result, 0);
}

#[test]
fn function_parameter_should_compile() {
    compile("{function foo(z) -> x {let y := 3 x := add(3, y)}}", &None);
}

#[test]
fn if_statement_should_compile() {
    let result = compile(
        "{function foo() -> x {x := 42 let y := 1 if lt(x, y) {x := add(y, 1)}}}",
        &Some("foo"),
    );
    assert_eq!(result, 42);
    let result = compile(
        "{function foo() -> x {x := 42 let y := 1 if gt(x, y) {x := add(y, 1)}}}",
        &Some("foo"),
    );
    assert_eq!(result, 2);
    let result = compile(
        "{function foo() -> x {x := 42 let y := 1 if eq(x, y) {x := add(y, 1)}}}",
        &Some("foo"),
    );
    assert_eq!(result, 42);
}

#[test]
fn switch_statement_should_compile() {
    let result = compile(
        "{function foo() -> x {x := 42 switch x case 1 {x := 22} default {x := 17}}}",
        &Some("foo"),
    );
    assert_eq!(result, 17);
}

#[test]
fn leave_should_compile() {
    let result = compile(
        "{function foo() -> x {x := 42 if lt(x, 55) {leave} x := 43}}",
        &Some("foo"),
    );
    assert_eq!(result, 42);
}

#[test]
fn for_statement_should_compile() {
    let result = compile("{function foo() -> x { x:= 0 for { let i := 0} lt(i, 10) { i := add(i, 1) } { x := add(i, x)}}}", &Some("foo"));
    assert_eq!(result, 45);
}

#[test]
fn continue_should_compile() {
    let result = compile("{function foo() -> x { x:= 0 for { let i := 0} lt(i, 10) { i := add(i, 1) } { if mod(i, 2) { continue } x := add(i, x) }}}", &Some("foo"));
    assert_eq!(result, 20);
}

#[test]
fn break_should_compile() {
    let result = compile("{function foo() -> x { x:= 0 for { let i := 0} lt(i, 10) { i := add(i, 1) } { if gt(x, 18) { break } x := add(i, x) }}}", &Some("foo"));
    assert_eq!(result, 21);
}

#[test]
fn call_should_compile() {
    let result = compile(
        "{function bar() -> x { x:= 42 } function foo() -> x { x := bar()}}",
        &Some("foo"),
    );
    assert_eq!(result, 42);
}

#[test]
fn call_void_should_compile() {
    let result = compile(
        "{function bar() {} function foo() -> x { x := 42 bar()}}",
        &Some("foo"),
    );
    assert_eq!(result, 42);
}

#[test]
fn tuples_should_compile() {
    compile("{ function foo() -> x, y { }}", &None);
}