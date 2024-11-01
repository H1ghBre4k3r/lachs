use std::error::Error;

use lachs::{token, Span};

#[token]
enum Token {
    #[terminal("+")]
    Plus,
    #[literal("[0-9]+")]
    Number,
}

#[test]
fn main() -> Result<(), Box<dyn Error>> {
    let result = Token::lex("1 + 2")?;

    let expected = vec![
        Token::Number {
            position: Span {
                start: (0, 0),
                end: (0, 1),
                source: "1 + 2".into(),
            },
            value: "1".into(),
        },
        Token::Plus {
            position: Span {
                start: (0, 2),
                end: (0, 3),
                source: "1 + 2".into(),
            },
        },
        Token::Number {
            position: Span {
                start: (0, 4),
                end: (0, 5),
                source: "1 + 2".into(),
            },
            value: "2".into(),
        },
    ];

    assert_eq!(result, expected);

    Ok(())
}