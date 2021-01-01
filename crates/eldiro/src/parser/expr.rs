use super::Parser;
use crate::lexer::SyntaxKind;

#[cfg(test)]
mod tests {
    use super::super::check;
    use super::*;
    use expect_test::expect;
    #[test]
    fn parse_number() {
        check(
            "123",
            expect![[r#"
Root@0..3
  Number@0..3 "123""#]],
        );
    }

    #[test]
    fn parse_binding_usage() {
        check(
            "abc",
            expect![[r#"Root@0..3
  Ident@0..3 "abc""#]],
        );
    }

    #[test]
    fn parse_simple_binary_expression() {
        check(
            "1+2",
            expect![[r#"
Root@0..3
  BinaryExpr@0..3
    Number@0..1 "1"
    Plus@1..2 "+"
    Number@2..3 "2""#]],
        );
    }

    #[test]
    fn parse_expression_with_precedence() {
        check(
            "1+2*3",
            expect![[r#"
Root@0..5
  BinaryExpr@0..5
    Number@0..1 "1"
    Plus@1..2 "+"
    BinaryExpr@2..5
      Number@2..3 "2"
      Star@3..4 "*"
      Number@4..5 "3""#
            ]]
        )
    }
    #[test]
    fn parse_binary_expression_with_mixed_binding_power() {
        check(
            "1+2*3-4",
            expect![[r#"
Root@0..7
  BinaryExpr@0..7
    BinaryExpr@0..5
      Number@0..1 "1"
      Plus@1..2 "+"
      BinaryExpr@2..5
        Number@2..3 "2"
        Star@3..4 "*"
        Number@4..5 "3"
    Minus@5..6 "-"
    Number@6..7 "4""#]],
        );
    }   

    #[test]
    fn negation_has_higher_binding_power_than_infix_operators() {
        check(
            "-20+20",
            expect![[r#"
Root@0..6
  BinaryExpr@0..6
    PrefixExpr@0..3
      Minus@0..1 "-"
      Number@1..3 "20"
    Plus@3..4 "+"
    Number@4..6 "20""#]],
        );
    }

    #[test]
    fn parse_nested_parentheses() {
        check(
            "((((((10))))))",
            expect![[r#"
Root@0..14
  LParen@0..1 "("
  LParen@1..2 "("
  LParen@2..3 "("
  LParen@3..4 "("
  LParen@4..5 "("
  LParen@5..6 "("
  Number@6..8 "10"
  RParen@8..9 ")"
  RParen@9..10 ")"
  RParen@10..11 ")"
  RParen@11..12 ")"
  RParen@12..13 ")"
  RParen@13..14 ")""#]],
        );
    }

    #[test]
    fn parentheses_affect_precedence() {
        check(
            "5*(2+1)",
            expect![[r#"
Root@0..7
  BinaryExpr@0..7
    Number@0..1 "5"
    Star@1..2 "*"
    LParen@2..3 "("
    BinaryExpr@3..6
      Number@3..4 "2"
      Plus@4..5 "+"
      Number@5..6 "1"
    RParen@6..7 ")""#]],
        );
    }


}

pub(super) fn expr(p: &mut Parser) {
    expr_binding_power(p, 0);
}

pub(super) fn expr_binding_power(p: &mut Parser, min_binding_power: u8) {
    let checkpoint = p.checkpoint();

    if let Some(kind) = p.peek() {
        match kind {
            SyntaxKind::Number | SyntaxKind::Ident => p.bump(),
            SyntaxKind::Minus | SyntaxKind::Plus => {
                let op = PrefixOP::Neg;
                let ((), right_binding_power) = op.binding_power();

                p.bump();

                p.start_node_at(checkpoint, SyntaxKind::PrefixExpr);
                expr_binding_power(p, right_binding_power);
                p.finish_node();
            },

            SyntaxKind::LParen => {
                p.bump();
                expr_binding_power(p, 0);
                assert_eq!(p.peek(), Some(SyntaxKind::RParen));
                p.bump();
            },
            _ => {}
        };
        
        loop {
            let op = match p.peek() {
                Some(SyntaxKind::Plus) => Op::Add,
                Some(SyntaxKind::Minus) => Op::Sub,
                Some(SyntaxKind::Star) => Op::Mul,
                Some(SyntaxKind::Slash) => Op::Div,
                _ => return,
            };

            let (l_bp, r_bp) = op.binding_power();

            if l_bp < min_binding_power {
                return;
            }

            p.bump();

            p.start_node_at(checkpoint, SyntaxKind::BinaryExpr);
            expr_binding_power(p, r_bp);
            p.finish_node();
        }
    }
}

enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn binding_power(&self) -> (u8, u8) {
        match self {
            Self::Add | Self::Sub => (1, 2),
            Self::Mul | Self::Div => (3, 4),
        }
    }
}

enum PrefixOP {
    Neg,
    Plus,
}

impl PrefixOP {
    fn binding_power(&self) -> ((), u8) {
        match self {
            Self::Neg | Self::Plus => ((), 5),
        }
    }
}