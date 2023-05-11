use crate::text_size::TextRange;
use crate::{Constant, ExcepthandlerKind, ExprKind, PatternKind, Ranged, StmtKind};

impl<U> ExprKind<U> {
    /// Returns a short name for the node suitable for use in error messages.
    pub fn python_name(&self) -> &'static str {
        match self {
            ExprKind::BoolOp { .. } | ExprKind::BinOp { .. } | ExprKind::UnaryOp { .. } => {
                "operator"
            }
            ExprKind::Subscript { .. } => "subscript",
            ExprKind::Await { .. } => "await expression",
            ExprKind::Yield { .. } | ExprKind::YieldFrom { .. } => "yield expression",
            ExprKind::Compare { .. } => "comparison",
            ExprKind::Attribute { .. } => "attribute",
            ExprKind::Call { .. } => "function call",
            ExprKind::Constant(crate::ExprConstant { value, .. }) => match value {
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
            ExprKind::List { .. } => "list",
            ExprKind::Tuple { .. } => "tuple",
            ExprKind::Dict { .. } => "dict display",
            ExprKind::Set { .. } => "set display",
            ExprKind::ListComp { .. } => "list comprehension",
            ExprKind::DictComp { .. } => "dict comprehension",
            ExprKind::SetComp { .. } => "set comprehension",
            ExprKind::GeneratorExp { .. } => "generator expression",
            ExprKind::Starred { .. } => "starred",
            ExprKind::Slice { .. } => "slice",
            ExprKind::JoinedStr(crate::ExprJoinedStr { values, .. }) => {
                if values
                    .iter()
                    .any(|e| matches!(e.node, ExprKind::JoinedStr { .. }))
                {
                    "f-string expression"
                } else {
                    "literal"
                }
            }
            ExprKind::FormattedValue { .. } => "f-string expression",
            ExprKind::Name { .. } => "name",
            ExprKind::Lambda { .. } => "lambda",
            ExprKind::IfExp { .. } => "conditional expression",
            ExprKind::NamedExpr { .. } => "named expression",
        }
    }
}

impl<U> Ranged for ExprKind<U> {
    fn range(&self) -> TextRange {
        match self {
            ExprKind::BoolOp(node) => node.range(),
            ExprKind::NamedExpr(node) => node.range(),
            ExprKind::BinOp(node) => node.range(),
            ExprKind::UnaryOp(node) => node.range(),
            ExprKind::Lambda(node) => node.range(),
            ExprKind::IfExp(node) => node.range(),
            ExprKind::Dict(node) => node.range(),
            ExprKind::Set(node) => node.range(),
            ExprKind::ListComp(node) => node.range(),
            ExprKind::SetComp(node) => node.range(),
            ExprKind::DictComp(node) => node.range(),
            ExprKind::GeneratorExp(node) => node.range(),
            ExprKind::Await(node) => node.range(),
            ExprKind::Yield(node) => node.range(),
            ExprKind::YieldFrom(node) => node.range(),
            ExprKind::Compare(node) => node.range(),
            ExprKind::Call(node) => node.range(),
            ExprKind::FormattedValue(node) => node.range(),
            ExprKind::JoinedStr(node) => node.range(),
            ExprKind::Constant(node) => node.range(),
            ExprKind::Attribute(node) => node.range(),
            ExprKind::Subscript(node) => node.range(),
            ExprKind::Starred(node) => node.range(),
            ExprKind::Name(node) => node.range(),
            ExprKind::List(node) => node.range(),
            ExprKind::Tuple(node) => node.range(),
            ExprKind::Slice(node) => node.range(),
        }
    }
}

impl<U> Ranged for StmtKind<U> {
    fn range(&self) -> TextRange {
        match self {
            StmtKind::FunctionDef(stmt) => stmt.range(),
            StmtKind::AsyncFunctionDef(stmt) => stmt.range(),
            StmtKind::ClassDef(stmt) => stmt.range(),
            StmtKind::Return(stmt) => stmt.range(),
            StmtKind::Delete(stmt) => stmt.range(),
            StmtKind::Assign(stmt) => stmt.range(),
            StmtKind::AugAssign(stmt) => stmt.range(),
            StmtKind::AnnAssign(stmt) => stmt.range(),
            StmtKind::For(stmt) => stmt.range(),
            StmtKind::AsyncFor(stmt) => stmt.range(),
            StmtKind::While(stmt) => stmt.range(),
            StmtKind::If(stmt) => stmt.range(),
            StmtKind::With(stmt) => stmt.range(),
            StmtKind::AsyncWith(stmt) => stmt.range(),
            StmtKind::Match(stmt) => stmt.range(),
            StmtKind::Raise(stmt) => stmt.range(),
            StmtKind::Try(stmt) => stmt.range(),
            StmtKind::TryStar(stmt) => stmt.range(),
            StmtKind::Assert(stmt) => stmt.range(),
            StmtKind::Import(stmt) => stmt.range(),
            StmtKind::ImportFrom(stmt) => stmt.range(),
            StmtKind::Global(stmt) => stmt.range(),
            StmtKind::Nonlocal(stmt) => stmt.range(),
            StmtKind::Expr(stmt) => stmt.range(),
            StmtKind::Pass(stmt) => stmt.range(),
            StmtKind::Break(stmt) => stmt.range(),
            StmtKind::Continue(stmt) => stmt.range(),
        }
    }
}

impl<U> Ranged for PatternKind<U> {
    fn range(&self) -> TextRange {
        match self {
            PatternKind::MatchValue(pattern) => pattern.range(),
            PatternKind::MatchSingleton(pattern) => pattern.range(),
            PatternKind::MatchSequence(pattern) => pattern.range(),
            PatternKind::MatchMapping(pattern) => pattern.range(),
            PatternKind::MatchClass(pattern) => pattern.range(),
            PatternKind::MatchStar(pattern) => pattern.range(),
            PatternKind::MatchAs(pattern) => pattern.range(),
            PatternKind::MatchOr(pattern) => pattern.range(),
        }
    }
}

impl<U> Ranged for ExcepthandlerKind<U> {
    fn range(&self) -> TextRange {
        match self {
            ExcepthandlerKind::ExceptHandler(handler) => handler.range(),
        }
    }
}
