use logos::Logos;

#[derive(Debug,Logos,Clone,PartialEq)]
// Skip whitespace
#[logos(skip r"[ \t\n\f]+")]
// Skip single line comments
#[logos(skip r"\/\/[^\n\r]*/")]
pub enum Token {
    // CamelCase
    #[regex(r#"[a-z]+(?:[A-Z][a-z]*)+"#)]
    // PascalCase
    #[regex(r#"[A-Z][a-z]*(?:[A-Z][a-z]*)*"#)]
    // Lowercase
    #[regex(r#"[a-z_][a-z0-9_]*"#)]
    // Uppercase
    #[regex(r#"[A-Z_][A-Z0-9_]*/i"#)]
    // Attribute Identifier
    #[regex(r#"@[a-z](\.?[a-z0-9_]+)*"#)]
    Identifier,

    // Integers
    #[regex(r#"-?[0-9]+"#)]
    // Floats
    #[regex(r#"-?\d+\.\d+"#)]
    // String Literals
    #[regex(r#""([^"]+)""#)]
    // Boolean
    #[regex(r"true|false")]
    Literal,

    #[token("null")]
    Null,

    // Symbols
    #[token("(")]
    OpenParen,
    #[token(")")]
    CloseParen,
    #[token("{")]
    OpenBrace,
    #[token("}")]
    CloseBrace,
    #[token("[")]
    OpenSquare,
    #[token("]")]
    CloseSquare,
    #[token("!")]
    Final,

    // Keywords
    #[token("plugin")]
    PluginKeyword,
    #[token("use")]
    UseKeyword,
    #[token("prop")]
    PropKeyword,
    #[token("enum")]
    EnumKeyword,
    #[token("type")]
    TypeKeyword,
    #[token("model")]
    ModelKeyword,
}