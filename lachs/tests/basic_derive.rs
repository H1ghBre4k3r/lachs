use lachs::token;

#[token]
enum Token {
    #[terminal("=")]
    Equals,
    #[literal("[0-9]+")]
    Number,
}

fn main() {}
