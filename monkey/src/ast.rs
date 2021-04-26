use enum_dispatch::enum_dispatch;

use crate::token::Token;

#[enum_dispatch]
#[derive(Debug, Clone)]
pub(crate) enum NodeEnum<'src> {
    Expression(ExpressionEnum<'src>),
    Statement(StatementEnum<'src>),
    Program(Program<'src>),
}

#[enum_dispatch(NodeEnum)]
pub(crate) trait Node<'src>: std::fmt::Debug + Clone {
    fn TokenLiteral(&self) -> &'src str;
    fn String(&self) -> String;
}

#[enum_dispatch]
#[derive(Debug, Clone)]
pub(crate) enum StatementEnum<'src> {
    Let(LetStatement<'src>),
    Return(ReturnStatement<'src>),
    Expression(ExpressionStatement<'src>),
}

impl<'src> Node<'src> for StatementEnum<'src> {
    fn TokenLiteral(&self) -> &'src str {
        match self {
            Self::Let(s) => s.TokenLiteral(),
            Self::Return(s) => s.TokenLiteral(),
            Self::Expression(s) => s.TokenLiteral(),
        }
    }

    fn String(&self) -> String {
        match self {
            Self::Let(s) => s.String(),
            Self::Return(s) => s.String(),
            Self::Expression(s) => s.String(),
        }
    }
}

#[enum_dispatch(StatementEnum)]
pub(crate) trait Statement<'src>: Node<'src> {}

#[enum_dispatch]
#[derive(Debug, Clone)]
pub(crate) enum ExpressionEnum<'src> {
    Identifier(Identifier<'src>),
    IntegerLiteral(IntegerLiteral<'src>),
    PrefixExpression(PrefixExpression<'src>),
    InfixExpression(InfixExpression<'src>),
}

impl<'src> Node<'src> for ExpressionEnum<'src> {
    fn TokenLiteral(&self) -> &'src str {
        match self {
            Self::Identifier(e) => e.TokenLiteral(),
            Self::IntegerLiteral(e) => e.TokenLiteral(),
            Self::PrefixExpression(e) => e.TokenLiteral(),
            Self::InfixExpression(e) => e.TokenLiteral(),
        }
    }

    fn String(&self) -> String {
        match self {
            Self::Identifier(e) => e.String(),
            Self::IntegerLiteral(e) => e.String(),
            Self::PrefixExpression(e) => e.String(),
            Self::InfixExpression(e) => e.String(),
        }
    }
}

#[enum_dispatch(ExpressionEnum)]
pub(crate) trait Expression<'src>: Node<'src> {}

#[derive(Debug, Clone)]
pub(crate) struct Program<'src> {
    pub(crate) statements: Vec<StatementEnum<'src>>,
}

impl<'src> Node<'src> for Program<'src> {
    fn TokenLiteral(&self) -> &'src str {
        if !self.statements.is_empty() {
            self.statements[0].TokenLiteral()
        } else {
            ""
        }
    }

    fn String(&self) -> String {
        self.statements
            .iter()
            .map(|stmt| stmt.String())
            .reduce(|acc, stmt| acc + &stmt)
            .unwrap_or_default()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct LetStatement<'src> {
    pub(crate) token: Token<'src>,
    pub(crate) name: Identifier<'src>,
    pub(crate) value: ExpressionEnum<'src>,
}

impl<'src> Node<'src> for LetStatement<'src> {
    fn TokenLiteral(&self) -> &'src str {
        self.token.literal
    }

    fn String(&self) -> String {
        format!("let {} = {};", self.name.String(), self.value.String())
    }
}

impl<'src> Expression<'src> for LetStatement<'src> {}

#[derive(Debug, Clone)]
pub(crate) struct Identifier<'src> {
    pub(crate) token: Token<'src>,
    pub(crate) value: &'src str,
}

impl<'src> Node<'src> for Identifier<'src> {
    fn TokenLiteral(&self) -> &'src str {
        self.token.literal
    }

    fn String(&self) -> String {
        self.token.literal.to_string()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ReturnStatement<'src> {
    pub(crate) token: Token<'src>,
    pub(crate) returnValue: ExpressionEnum<'src>,
}

impl<'src> Node<'src> for ReturnStatement<'src> {
    fn TokenLiteral(&self) -> &'src str {
        self.token.literal
    }

    fn String(&self) -> String {
        format!("return {};", self.returnValue.String())
    }
}

impl<'src> Statement<'src> for ReturnStatement<'src> {}

#[derive(Debug, Clone)]
pub(crate) struct ExpressionStatement<'src> {
    pub(crate) token: Token<'src>,
    pub(crate) expression: Option<ExpressionEnum<'src>>,
}

impl<'src> Node<'src> for ExpressionStatement<'src> {
    fn TokenLiteral(&self) -> &'src str {
        self.token.literal
    }

    fn String(&self) -> String {
        self.expression
            .as_ref()
            .map(|e| e.String())
            .unwrap_or_default()
    }
}

impl<'src> Statement<'src> for ExpressionStatement<'src> {}

#[derive(Debug, Clone)]
pub(crate) struct IntegerLiteral<'src> {
    pub(crate) token: Token<'src>,
    pub(crate) value: i64,
}

impl<'src> Node<'src> for IntegerLiteral<'src> {
    fn TokenLiteral(&self) -> &'src str {
        self.token.literal
    }

    fn String(&self) -> String {
        self.TokenLiteral().to_string()
    }
}

impl<'src> Expression<'src> for IntegerLiteral<'src> {}

#[derive(Debug, Clone)]
pub(crate) struct PrefixExpression<'src> {
    pub(crate) token: Token<'src>,
    pub(crate) operator: &'src str,
    pub(crate) right: Box<ExpressionEnum<'src>>,
}

impl<'src> Node<'src> for PrefixExpression<'src> {
    fn TokenLiteral(&self) -> &'src str {
        self.token.literal
    }

    fn String(&self) -> String {
        format!("({}{})", self.operator, self.right.String())
    }
}

impl<'src> Expression<'src> for PrefixExpression<'src> {}

#[derive(Debug, Clone)]
pub(crate) struct InfixExpression<'src> {
    pub(crate) token: Token<'src>,
    pub(crate) left: Box<ExpressionEnum<'src>>,
    pub(crate) operator: &'src str,
    pub(crate) right: Box<ExpressionEnum<'src>>,
}

impl<'src> Node<'src> for InfixExpression<'src> {
    fn TokenLiteral(&self) -> &'src str {
        self.token.literal
    }

    fn String(&self) -> String {
        format!(
            "({} {} {})",
            self.left.String(),
            self.operator,
            self.right.String()
        )
    }
}

impl<'src> Expression<'src> for InfixExpression<'src> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::TokenKind;

    #[test]
    fn String() {
        let program = Program {
            statements: vec![LetStatement {
                token: Token {
                    kind: TokenKind::LET,
                    literal: "let",
                },
                name: Identifier {
                    token: Token {
                        kind: TokenKind::IDENT,
                        literal: "myVar",
                    },
                    value: "myVar",
                },
                value: Identifier {
                    token: Token {
                        kind: TokenKind::IDENT,
                        literal: "anotherVar",
                    },
                    value: "anotherVar",
                }
                .into(),
            }
            .into()],
        };

        assert_eq!(program.String(), "let myVar = anotherVar;")
    }
}