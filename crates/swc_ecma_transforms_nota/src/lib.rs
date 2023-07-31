use swc_atoms::JsWord;
use swc_common::DUMMY_SP;
use swc_ecma_ast::{ArrayLit, Expr, ExprOrSpread, Lit, NotaTemplate, Str};
use swc_ecma_visit::{as_folder, noop_visit_mut_type, Fold, VisitMut, VisitMutWith};

struct Nota;

impl Nota {
    fn nota_template_to_expr(&self, tpl: &NotaTemplate) -> Expr {
        Expr::Array(ArrayLit {
            span: tpl.span,
            elems: tpl
                .exprs
                .iter()
                .map(|elem| {
                    let lit = Expr::Lit(Lit::Str(Str {
                        span: DUMMY_SP,
                        value: JsWord::from(elem.to_string()),
                        raw: None,
                    }));
                    Some(ExprOrSpread::from(Box::new(lit)))
                })
                .collect(),
        })
    }
}

impl VisitMut for Nota {
    noop_visit_mut_type!();

    fn visit_mut_expr(&mut self, expr: &mut Expr) {
        match expr {
            Expr::NotaTemplate(tpl) => {
                *expr = self.nota_template_to_expr(tpl);
            }
            _ => {
                expr.visit_mut_children_with(self);
            }
        }
    }
}

pub fn nota() -> impl Fold + VisitMut {
    as_folder(Nota)
}
