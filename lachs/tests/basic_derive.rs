use lachs::token;

#[token]
enum Token {
    #[terminal("=")]
    Eq,
    #[literal("[0-9]+")]
    Number,
}

fn main() {}
