use serde::Deserialize;
use swc_common::DUMMY_SP;
use swc_ecma_ast::{Expr, Lit, MemberExpr, Str};
use swc_ecma_visit::{VisitMut, VisitMutWith};

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum Config {
    All(bool),
    WithOptions(Options),
}

impl Config {
    pub fn truthy(&self) -> bool {
        match self {
            Config::All(b) => *b,
            Config::WithOptions(_) => true,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Options {
    #[serde(default)]
    pub only: Vec<String>,
}

struct InlineEnv {
    only: Option<Vec<String>>,
    parent: Option<Expr>,
    replace_value: Option<Expr>,
}

impl InlineEnv {
    fn replace_process_env_var(&self, var_name: &str) -> Option<Expr> {
        std::env::vars()
            .find(|(key, _)| key == var_name)
            .map(|(_, value)| {
                Expr::Lit(Lit::Str(Str {
                    value: value.into(),
                    span: DUMMY_SP,
                    raw: None, // raw value is not required for replacement
                }))
            })
    }
}

impl VisitMut for InlineEnv {
    fn visit_mut_expr(&mut self, expr: &mut Expr) {
        let old_parent = self.parent.take();
        self.parent = Some(expr.clone());

        expr.visit_mut_children_with(self);

        if let Some(replace_value) = self.replace_value.take() {
            *expr = replace_value;
            self.replace_value = None;
        }

        self.parent = old_parent;

        match expr {
            Expr::Member(MemberExpr { obj, prop, .. }) => {
                if let Some(obj_ident) = obj.as_ident() {
                    if obj_ident.sym.as_ref() == "process" {
                        if let Some(prop_ident) = prop.as_ident() {
                            if prop_ident.sym.as_ref() == "env" {
                                if let Some(Expr::Member(MemberExpr {
                                    prop: parent_prop, ..
                                })) = self.parent.as_ref()
                                {
                                    if let Some(next_prop_ident) = parent_prop.as_ident() {
                                        let env_var = next_prop_ident.sym.as_ref();

                                        if self.only.is_none()
                                            || self.only.as_ref().unwrap().contains(&env_var.into())
                                        {
                                            if let Some(new_expr) =
                                                self.replace_process_env_var(env_var)
                                            {
                                                self.replace_value = Some(new_expr);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn inline_env(config: Config) -> impl VisitMut {
    let only: Option<Vec<String>> = match config {
        Config::WithOptions(x) => Some(x.only),
        _ => None,
    };

    InlineEnv {
        parent: None,
        replace_value: None,
        only: only,
    }
}
