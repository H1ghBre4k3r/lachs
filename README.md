# Lachs

A tool to automatically generate a lexer based on a given enum.

## Usage

To generate a lexer from a given struct, just annotate it with [`token`]:

```rust
use lachs::token;

#[token]
pub enum Token {
    #[terminal("+")]
    Plus,
    #[literal("[0-9]+")]
    Integer
}
```

As you can see, we also annotated the variants `Token::Plus` and `Token::Integer` with `#[terminal("+")]` and `#[literal("[0-9]+")]`, respectively.

The helper `#[terminal(...)]` takes a string literal which has to match exactly to be lexed as the decorated token, while `#[literal(...)]` takes
a regular expression to extract a matched sequence from the text.

These helper macros get evaluated by `#[token]` and describe the two different kinds of tokens the lexer can understand:

- terminals (without an own value)
- literals (with an own value)

Under the hood, the proc macro expands the struct to roughly the following:

```rust
pub enum Token {
    Plus {
        position: lachs::Span,
    },
    Integer {
        value: String,
        position: lachs::Span,
    }
}
```

Both, terminals and literals have a field named `position` to store the position in the originating text.
Literals have an additional field `value` which stores the value which matched the passed regular expression.

Additionally, the `Token` enum gets a function which lets you pass a string and get the result of the lexing back:

```rust
use lachs::token;

#[token]
pub enum Token {
    #[terminal("+")]
    Plus,
    #[literal("[0-9]+")]
    Integer
}

let result: Result<Vec<Token>, LexError> = Token::lex("2 + 2");
```

## Caveats

The macro also generates an implementation of `PartialEq` for the decorated enum. However, this implementation _does **not**_ take the position into account.

If you want to check whether two tokens are exactly the same, you can utilize the `Token::does_equal(...)` function.

### Generated Stuff

The macro generates additional structs for performing the actual lexing. These should not be touched, if possible. However, they can lead to name collisions.
