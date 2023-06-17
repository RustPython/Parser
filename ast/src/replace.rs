use crate as ast;
use crate::fold::{Fold, Foldable};
use crate::text_size::TextRange;
use std::collections::HashMap;

pub enum Replacement {
    Node(ast::Expr),
    Sequence(Vec<ast::Expr>),
}

impl From<ast::Expr> for Replacement {
    fn from(node: ast::Expr) -> Self {
        Self::Node(node)
    }
}

impl From<Vec<ast::Expr>> for Replacement {
    fn from(nodes: Vec<ast::Expr>) -> Self {
        Self::Sequence(nodes)
    }
}

pub struct ReplaceError {
    node: ast::Ast,
    reason: &'static str,
}

impl ReplaceError {
    fn new(node: impl Into<ast::Ast>, reason: &'static str) -> Self {
        let node = node.into();
        Self { node, reason }
    }
}

pub struct Replacer {
    map: HashMap<String, Replacement>,
}

impl Fold<TextRange> for Replacer {
    type TargetU = TextRange;
    type Error = ReplaceError;
    type UserContext = ();

    fn will_map_user(&mut self, _user: &TextRange) -> Self::UserContext {}
    fn map_user(&mut self, user: TextRange, _context: ()) -> Result<Self::TargetU, Self::Error> {
        Ok(user)
    }

    fn fold_expr(
        &mut self,
        node: ast::Expr<TextRange>,
    ) -> Result<ast::Expr<Self::TargetU>, Self::Error> {
        let node = match node {
            ast::Expr::Name(name) if self.map.contains_key(name.id.as_str()) => {
                let expr = &self.map[name.id.as_str()];
                match expr {
                    Replacement::Node(node) => node.clone(),
                    Replacement::Sequence(_) => {
                        return Err(ReplaceError::new(name, "single node expected"));
                    }
                }
            }
            // Starred
            node => node,
        };
        Ok(node)
    }
}

#[cfg(tests)]
mod tests {
    #[test]
    fn replace() {
        let replacements = vec![("a".to_owned(), ast::Constant::parse("2").unwrap())];
        let mut replacer = Replacer {
            map: replacements.into_iter().collect(),
        };

        let source = "x = (1, a, 3)";
        let ast = ast::Statement::parse(source, "<test>").unwrap();
        let edited = replacer.fold(ast).unwrap();

        let expected = ast::Statement::parse("x = (1, 2, 3)", "<test>").unwrap();
        assert_eq!(edited, expected);
    }
}
