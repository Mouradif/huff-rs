use huff_lexer::*;
use huff_parser::*;
use huff_utils::{
    ast::{FunctionDefinition, FunctionType},
    prelude::*,
};
use std::collections::HashMap;

#[test]
fn parses_valid_function_tuple_arguments() {
    let source = "#define function callFunction(address, (address, uint256), bytes) nonpayable returns ()";
    let expected_fn = FunctionDefinition {
        name: "test".to_string(),
        inputs: vec![
            Argument {
                name: None,
                arg_type: Some(String::from("uint256")),
                indexed: false,
                arg_location: None,
                span: AstSpan(vec![Span { start: 22, end: 28, file: None }]),
            },
            Argument {
                name: Some(String::from("b")),
                arg_type: Some(String::from("bool")),
                indexed: false,
                arg_location: None,
                span: AstSpan(vec![
                    Span { start: 30, end: 33, file: None },
                    Span { start: 35, end: 35, file: None },
                ]),
            },
        ],
        fn_type: FunctionType::View,
        outputs: vec![Argument {
            name: None,
            arg_type: Some(String::from("uint256")),
            indexed: false,
            arg_location: None,
            span: AstSpan(vec![Span { start: 51, end: 57, file: None }]),
        }],
        signature: [84, 204, 215, 119],
        span: AstSpan(vec![
            Span { start: 0, end: 6, file: None },
            Span { start: 8, end: 15, file: None },
            Span { start: 17, end: 20, file: None },
            Span { start: 21, end: 21, file: None },
            Span { start: 22, end: 28, file: None },
            Span { start: 29, end: 29, file: None },
            Span { start: 30, end: 33, file: None },
            Span { start: 35, end: 35, file: None },
            Span { start: 36, end: 36, file: None },
            Span { start: 38, end: 41, file: None },
            Span { start: 43, end: 49, file: None },
            Span { start: 50, end: 50, file: None },
            Span { start: 51, end: 57, file: None },
            Span { start: 58, end: 58, file: None },
        ]),
    };

    let flattened_source = FullFileSource { source, file: None, spans: vec![] };
    let lexer = Lexer::new(flattened_source.source);
    let tokens = lexer
        .into_iter()
        .map(|x| x.unwrap())
        .filter(|x| !matches!(x.kind, TokenKind::Whitespace))
        .collect::<Vec<Token>>();
    println!("{:#?}", tokens);
    let mut parser = Parser::new(tokens, None);
    let _ = parser.match_kind(TokenKind::Define);
    let function = parser.parse_function().unwrap();

    // Ensure that the parser constructed the `Function` node correctly.
    assert_eq!(function, expected_fn);
}
