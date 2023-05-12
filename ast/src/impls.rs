use crate::ranged::Ranged;
use crate::text_size::TextRange;
use crate::{Constant, Excepthandler, Expr, Pattern, Stmt};

impl<R> Expr<R> {
    /// Returns a short name for the node suitable for use in error messages.
    pub fn python_name(&self) -> &'static str {
        match self {
            Expr::BoolOp { .. } | Expr::BinOp { .. } | Expr::UnaryOp { .. } => "operator",
            Expr::Subscript { .. } => "subscript",
            Expr::Await { .. } => "await expression",
            Expr::Yield { .. } | Expr::YieldFrom { .. } => "yield expression",
            Expr::Compare { .. } => "comparison",
            Expr::Attribute { .. } => "attribute",
            Expr::Call { .. } => "function call",
            Expr::Constant(crate::ExprConstant { value, .. }) => match value {
                Constant::Str(_)
                | Constant::Int(_)
                | Constant::Float(_)
                | Constant::Complex { .. }
                | Constant::Bytes(_) => "literal",
                Constant::Tuple(_) => "tuple",
                Constant::Bool(b) => {
                    if *b {
                        "True"
                    } else {
                        "False"
                    }
                }
                Constant::None => "None",
                Constant::Ellipsis => "ellipsis",
            },
            Expr::List { .. } => "list",
            Expr::Tuple { .. } => "tuple",
            Expr::Dict { .. } => "dict display",
            Expr::Set { .. } => "set display",
            Expr::ListComp { .. } => "list comprehension",
            Expr::DictComp { .. } => "dict comprehension",
            Expr::SetComp { .. } => "set comprehension",
            Expr::GeneratorExp { .. } => "generator expression",
            Expr::Starred { .. } => "starred",
            Expr::Slice { .. } => "slice",
            Expr::JoinedStr(crate::ExprJoinedStr { values, .. }) => {
                if values
                    .iter()
                    .any(|e| matches!(e.node, Expr::JoinedStr { .. }))
                {
                    "f-string expression"
                } else {
                    "literal"
                }
            }
            Expr::FormattedValue { .. } => "f-string expression",
            Expr::Name { .. } => "name",
            Expr::Lambda { .. } => "lambda",
            Expr::IfExp { .. } => "conditional expression",
            Expr::NamedExpr { .. } => "named expression",
        }
    }
}

impl Ranged for Expr<TextRange> {
    fn range(&self) -> TextRange {
        match self {
            Expr::BoolOp(node) => node.range(),
            Expr::NamedExpr(node) => node.range(),
            Expr::BinOp(node) => node.range(),
            Expr::UnaryOp(node) => node.range(),
            Expr::Lambda(node) => node.range(),
            Expr::IfExp(node) => node.range(),
            Expr::Dict(node) => node.range(),
            Expr::Set(node) => node.range(),
            Expr::ListComp(node) => node.range(),
            Expr::SetComp(node) => node.range(),
            Expr::DictComp(node) => node.range(),
            Expr::GeneratorExp(node) => node.range(),
            Expr::Await(node) => node.range(),
            Expr::Yield(node) => node.range(),
            Expr::YieldFrom(node) => node.range(),
            Expr::Compare(node) => node.range(),
            Expr::Call(node) => node.range(),
            Expr::FormattedValue(node) => node.range(),
            Expr::JoinedStr(node) => node.range(),
            Expr::Constant(node) => node.range(),
            Expr::Attribute(node) => node.range(),
            Expr::Subscript(node) => node.range(),
            Expr::Starred(node) => node.range(),
            Expr::Name(node) => node.range(),
            Expr::List(node) => node.range(),
            Expr::Tuple(node) => node.range(),
            Expr::Slice(node) => node.range(),
        }
    }
}

impl Ranged for Stmt<TextRange> {
    fn range(&self) -> TextRange {
        match self {
            Stmt::FunctionDef(stmt) => stmt.range(),
            Stmt::AsyncFunctionDef(stmt) => stmt.range(),
            Stmt::ClassDef(stmt) => stmt.range(),
            Stmt::Return(stmt) => stmt.range(),
            Stmt::Delete(stmt) => stmt.range(),
            Stmt::Assign(stmt) => stmt.range(),
            Stmt::AugAssign(stmt) => stmt.range(),
            Stmt::AnnAssign(stmt) => stmt.range(),
            Stmt::For(stmt) => stmt.range(),
            Stmt::AsyncFor(stmt) => stmt.range(),
            Stmt::While(stmt) => stmt.range(),
            Stmt::If(stmt) => stmt.range(),
            Stmt::With(stmt) => stmt.range(),
            Stmt::AsyncWith(stmt) => stmt.range(),
            Stmt::Match(stmt) => stmt.range(),
            Stmt::Raise(stmt) => stmt.range(),
            Stmt::Try(stmt) => stmt.range(),
            Stmt::TryStar(stmt) => stmt.range(),
            Stmt::Assert(stmt) => stmt.range(),
            Stmt::Import(stmt) => stmt.range(),
            Stmt::ImportFrom(stmt) => stmt.range(),
            Stmt::Global(stmt) => stmt.range(),
            Stmt::Nonlocal(stmt) => stmt.range(),
            Stmt::Expr(stmt) => stmt.range(),
            Stmt::Pass(stmt) => stmt.range(),
            Stmt::Break(stmt) => stmt.range(),
            Stmt::Continue(stmt) => stmt.range(),
        }
    }
}

impl Ranged for Pattern<TextRange> {
    fn range(&self) -> TextRange {
        match self {
            Pattern::MatchValue(pattern) => pattern.range(),
            Pattern::MatchSingleton(pattern) => pattern.range(),
            Pattern::MatchSequence(pattern) => pattern.range(),
            Pattern::MatchMapping(pattern) => pattern.range(),
            Pattern::MatchClass(pattern) => pattern.range(),
            Pattern::MatchStar(pattern) => pattern.range(),
            Pattern::MatchAs(pattern) => pattern.range(),
            Pattern::MatchOr(pattern) => pattern.range(),
        }
    }
}

impl Ranged for Excepthandler<TextRange> {
    fn range(&self) -> TextRange {
        match self {
            Excepthandler::ExceptHandler(handler) => handler.range(),
        }
    }
}
