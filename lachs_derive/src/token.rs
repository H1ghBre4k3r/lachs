use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, ExprLit, Lit, Variant};

pub fn impl_token_macro(item: syn::Item) -> TokenStream {
    let syn::Item::Enum(syn::ItemEnum {
        ident,
        variants,
        vis,
        ..
    }) = item
    else {
        panic!("")
    };

    let variants_with_fields = variants.clone().into_iter().filter_map(|variant| {
        let Variant { attrs, ident, .. } = variant;

        for attr in &attrs {
            let Some(attr_ident) = attr.path().get_ident() else {
                continue;
            };

            let Ok(Expr::Lit(ExprLit {
                lit: Lit::Str(literal),
                ..
            })) = attr.parse_args::<Expr>()
            else {
                panic!("missing matcher for #[terminal] {ident}");
            };

            let _ = (ident.clone(), quote! { position: lachs::Span, });

            let (fields, insertions) = if *attr_ident == "terminal" {
                (
                    quote! {
                        position: lachs::Span,
                    },
                    {
                        let literal = literal.value();

                        quote! {
                            terminal!(entries, #ident, #literal);
                        }
                    },
                )
            } else if *attr_ident == "literal" {
                (
                    quote! {
                        position: lachs::Span,
                        value: String,
                    },
                    {
                        let literal = literal.value();

                        quote! {
                            literal!(entries, #ident, #literal);
                        }
                    },
                )
            } else {
                continue;
            };

            return Some((ident, fields, insertions));
        }

        None
    });

    let filled_variants = variants_with_fields.clone().map(|(ident, fields, _)| {
        quote! {
            #ident {
                #fields
            },
        }
    });

    let equals_cases = variants_with_fields.clone().map(|(ident, _, _)| {
        quote! {
            (Self::#ident { .. }, Self::#ident { .. }) => true,
        }
    });

    let get_position_cases = variants_with_fields.clone().map(|(ident, _, _)| {
        quote! {
            Self::#ident { position, .. } => position.clone(),
        }
    });

    let insertions = variants_with_fields
        .clone()
        .map(|(_, _, insertion)| insertion);

    let gen = quote! {
        #[derive(Debug, Clone)]
        #vis enum #ident {
            #(#filled_variants)*
        }

        impl PartialEq for #ident {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    #(#equals_cases)*
                    _ => false,
                }
            }
        }

        impl Eq for #ident {}

        impl #ident {
            fn position(&self) -> lachs::Span {
                match self {
                    #(#get_position_cases)*
                }
            }

            fn does_equal(&self, other: &Self) -> bool {
                self.position() == other.position()
            }

            pub fn lex(input: impl ToString) -> LexResult<Vec<#ident>> {
                Lexer::new(input.to_string().as_str()).lex()
            }
        }

        use lachs::colored::Colorize;
        use lachs::regex::{Match, Regex};

        macro_rules! terminal {
            ($entries:ident, $name:ident, $value:expr) => {
                Self::insert(
                    &mut $entries,
                    Regex::new(&$value.escape_unicode().to_string()).unwrap(),
                    |matched, (line, col), source| #ident::$name {
                        position: lachs::Span { start: (line, col), end: (line, (col+matched.as_str().len())), source }
                    },
                );
            };
        }

        macro_rules! literal {
            ($entries:ident, $name:ident, $value:expr) => {
                Self::insert(
                    &mut $entries,
                    Regex::new($value).unwrap(),
                    |matched, (line, col), source| #ident::$name {
                        value: matched.as_str().parse().unwrap(),
                        position: lachs::Span { start: (line, col), end: (line, (col+matched.as_str().len())), source }
                    },
                );
            };
        }

        type EntryInputSpan = (usize, usize);

        type Entries = Vec<(Regex, Box<dyn Fn(Match, EntryInputSpan, String) -> #ident>)>;

        struct Lexikon {
            entries: Entries,
        }

        impl<'a> Lexikon {
            pub fn new() -> Lexikon {
                let mut entries = vec![];

                #(#insertions)*

                Lexikon { entries }
            }

            fn insert<F: Fn(Match, EntryInputSpan, String) -> #ident + 'static>(entries: &mut Entries, reg: Regex, f: F) {
                entries.push((reg, Box::new(f)))
            }

            pub fn find_longest_match(
                &self,
                pattern: &'a str,
                position: EntryInputSpan,
                source: String
            ) -> (usize, Option<#ident>) {
                let mut longest = (0, None);

                for (reg, mapper) in &self.entries {
                    let Some(res) = reg.captures_at(pattern, 0).and_then(|res| res.get(0)) else {
                        continue;
                    };

                    let len = res.len();

                    if len > longest.0 && res.start() == 0 {
                        longest = (len, Some(mapper(res, position, source.clone())));
                    }
                }

                longest
            }
        }

        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct LexError(String);

        pub type LexResult<T> = Result<T, LexError>;

        impl std::fmt::Display for LexError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.0.as_str())
            }
        }

        impl std::error::Error for LexError {}

        struct Lexer<'a> {
            tokens: Vec<#ident>,
            lexikon: Lexikon,
            position: usize,
            col: usize,
            line: usize,
            input: &'a str,
        }

        impl<'a> Lexer<'a> {
            pub fn new(input: &'a str) -> Self {
                Self {
                    tokens: vec![],
                    lexikon: Lexikon::new(),
                    position: 0,
                    col: 0,
                    line: 0,
                    input,
                }
            }

            fn eat_whitespace(&mut self) {
                while let Some(c) = self.input.as_bytes().get(self.position) {
                    if !c.is_ascii_whitespace() {
                        return;
                    }

                    if *c == b'\n' {
                        self.line += 1;
                        self.col = 0;
                    } else {
                        self.col += 1;
                    }
                    self.position += 1;
                }
            }

            pub fn lex(mut self) -> LexResult<Vec<#ident>> {
                while self.position != self.input.len() {
                    self.eat_whitespace();
                    let (len, res) = self
                        .lexikon
                        .find_longest_match(
                            &self.input[self.position..],
                            (self.line, self.col),
                            self.input.to_string(),
                        )
                        .clone();

                    match res {
                        Some(t) => self.tokens.push(t),
                        None => {
                            if self.position == self.input.len() {
                                return Ok(self.tokens);
                            }
                            return Err(LexError(format!(
                                "Failed to lex '{}' at position {}; remaining '{}'",
                                self.input,
                                self.position,
                                &self.input[self.position..]
                            )));
                        }
                    };
                    self.position += len;
                    self.col += len;
                }

                Ok(self.tokens)
            }
        }
    };

    gen.into()
}
