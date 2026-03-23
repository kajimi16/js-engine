use crate::ast::position::{LinearPosition, LinearSpan, Position, Span};
use num_bigint::BigInt;
pub struct Token {
    kind: TokenKind,
    span: Span,
    linear_span: LinearSpan,
}

impl Token {
    pub const fn new(kind: TokenKind, span: Span, linear_span: LinearSpan) -> Self {
        Self {
            kind,
            span,
            linear_span,
        }
    }

    pub const fn kind(&self) -> &TokenKind {
        &self.kind
    }

    pub const fn span(&self) -> &Span {
        &self.span
    }

    pub const fn linear_span(&self) -> &LinearSpan {
        &self.linear_span
    }
}

#[derive(Debug, Clone)]
pub enum Numeric {
    Rational(f64),
    Integer(i32),
    BigInt(Box<BigInt>), //Bigint is a struct that represents a big integer, it is stored in a Box to avoid the size of the Token struct being too large
}

impl from<f64> for Numeric {
    #[inline]
    fn from(value: f64) -> Self {
        Self::Rational(value)
    }
}

impl from<i32> for Numeric {
    #[inline]
    fn from(value: i32) -> Self {
        Self::Integer(value)
    }
}

impl from<BigInt> for Numeric {
    #[inline]
    fn from(value: BigInt) -> Self {
        Self::BigInt(Box::new(value))
    }
}

#[derive(Debug, Clone)]
pub enum TokenKind {
    EOF,                      // end of file
    LineTerminator,           // line terminator
    Comment,                  // comment
    Keyword,                  // keyword
    IdentifierName,           // identifier name
    PrivateIdentifier,        // private identifier
    Punctuator,               // punctuator
    BooleanLiteral,           // boolean literal
    NullLiteral,              // null literal
    NumericLiteral(Numeric),  // numeric literal
    StringLiteral,            // string literal
    RegularExpressionLiteral, // regular expression literal
    TemplateNoSubstitution,   // template literal with no substitution
    TemplateMiddle,           // template literal middle
}

impl TokenKind {
    #[inline]
    pub const fn eof() -> Self {
        Self::EOF
    }

    #[inline]
    pub const fn line_terminator() -> Self {
        Self::LineTerminator
    }

    #[inline]
    pub const fn comment() -> Self {
        Self::Comment
    }

    #[inline]
    pub const fn keyword() -> Self {
        Self::Keyword
    }

    #[inline]
    pub const fn identifier_name() -> Self {
        Self::IdentifierName
    }

    #[inline]
    pub const fn private_identifier() -> Self {
        Self::PrivateIdentifier
    }

    #[inline]
    pub const fn punctuator() -> Self {
        Self::Punctuator
    }

    #[inline]
    pub const fn boolean_literal() -> Self {
        Self::BooleanLiteral
    }

    #[inline]
    pub const fn null_literal() -> Self {
        Self::NullLiteral
    }

    #[inline]
    pub fn numeric_literal(value: Numeric) -> Self {
        Self::NumericLiteral(value)
    }

    #[inline]
    pub const fn string_literal() -> Self {
        Self::StringLiteral
    }

    #[inline]
    pub const fn regular_expression_literal() -> Self {
        Self::RegularExpressionLiteral
    }

    #[inline]
    pub const fn template_no_substitution() -> Self {
        Self::TemplateNoSubstitution
    }

    #[inline]
    pub const fn template_middle() -> Self {
        Self::TemplateMiddle
    }
}
