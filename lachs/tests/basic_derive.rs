use lachs::{token, Span};

#[token]
enum Token {
    #[terminal("=")]
    Equals,
    #[literal("[0-9]+")]
    Number,
}

#[test]
fn main() {
    let eq = Token::Equals(Equals {
        position: Span::default(),
    });

    assert_eq!(eq.get_name(), String::from("Equals"));

    let eq = Equals {
        position: Span::default(),
    };
    assert_eq!(eq.get_name(), String::from("Equals"));
}
