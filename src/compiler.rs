use crate::types::Expression;

pub struct Context {
    // TODO
}

pub fn compile_identifier(v: &str) -> String {
    // Don't care about UTF-8 since identifiers are guaranteed to be ASCII
    let mut chars = v.chars();

    match chars.next() {
        Some(c) => {
            let rest: String = chars.collect();
            if rest.len() == 0 {
                c.to_string()
            } else {
                format!("{}_{{{}}}", c, rest)
            }
        }
        None => "".to_string(),
    }
}

pub fn compile_expr(ctx: &mut Context, expr: Expression) -> String {
    match expr {
        Expression::Num { val: v } => v.to_string(),
        Expression::Variable { val: v } => compile_identifier(v),
        Expression::BinaryExpr {
            left: l,
            operator: op,
            right: r,
        } => format!("{}{}{}", compile_expr(ctx, *l), op, compile_expr(ctx, *r)),
        Expression::UnaryExpr {
            val: v,
            operator: op,
        } => format!("{}{}", compile_expr(ctx, *v), op),
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(i: Expression, r: &str) {
        assert_eq!(compile_expr(&mut Context {}, i), r.to_string());
    }

    #[test]
    fn num() {
        check(Expression::Num { val: "5" }, "5");
        check(Expression::Num { val: "2.3" }, "2.3");
    }

    #[test]
    fn variable() {
        check(Expression::Variable { val: "" }, "");
        check(Expression::Variable { val: "a" }, "a");
        check(Expression::Variable { val: "abc" }, "a_{bc}");
    }

    #[test]
    fn binary_expr() {
        check(
            Expression::BinaryExpr {
                left: Box::new(Expression::Num { val: "1" }),
                operator: "+",
                right: Box::new(Expression::Num { val: "2" }),
            },
            "1+2",
        )
    }

    #[test]
    fn unary_expression() {
        check(
            Expression::UnaryExpr {
                val: Box::new(Expression::Num { val: "2" }),
                operator: "!",
            },
            "2!",
        );
    }
}