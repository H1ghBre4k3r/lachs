# Lachs Derive

> IMPORTANT: You are not supposed to use this crate directly.
> The only way we guarantee a working usage is to directly use the `lachs` crate.

This crate provides the implementation of a lexer generator in the form
of the `#[token]` proc macro. Applying this macro to an enum will consume
this enum and generate a new enum where each variant has the fields
corresponding to the type of token this field represents.
