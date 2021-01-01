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

}

pub(super) fn expr(p: &mut Parser) {
    expr_binding_power(p, 0);
}

pub(super) fn expr_binding_power(p: &mut Parser, min_binding_power: u8) {
    let checkpoint = p.checkpoint();

    if let Some(kind) = p.peek() {
        match kind {
            SyntaxKind::Number | SyntaxKind::Ident => p.bump(),
            _ => {}
        };

        match kind {
            SyntaxKind::Minus | SyntaxKind::Plus => {},
            _ => {}
        }
        
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
