// File automatically generated by ast/asdl_rs.py.

use crate::fold_helpers::Foldable;
use crate::generic::Custom;
pub trait Fold<U> {
    type TargetU;
    type Error;
    fn map_user(&mut self, user: U) -> Result<Self::TargetU, Self::Error>;

    fn fold<X: Foldable<U, Self::TargetU>>(&mut self, node: X) -> Result<X::Mapped, Self::Error> {
        node.fold(self)
    }
    fn fold_mod(&mut self, node: Mod<U>) -> Result<Mod<Self::TargetU>, Self::Error> {
        fold_mod(self, node)
    }
    fn fold_stmt(&mut self, node: Stmt<U>) -> Result<Stmt<Self::TargetU>, Self::Error> {
        fold_stmt(self, node)
    }
    fn fold_expr(&mut self, node: Expr<U>) -> Result<Expr<Self::TargetU>, Self::Error> {
        fold_expr(self, node)
    }
    fn fold_expr_context(&mut self, node: ExprContext) -> Result<ExprContext, Self::Error> {
        fold_expr_context(self, node)
    }
    fn fold_boolop(&mut self, node: Boolop) -> Result<Boolop, Self::Error> {
        fold_boolop(self, node)
    }
    fn fold_operator(&mut self, node: Operator) -> Result<Operator, Self::Error> {
        fold_operator(self, node)
    }
    fn fold_unaryop(&mut self, node: Unaryop) -> Result<Unaryop, Self::Error> {
        fold_unaryop(self, node)
    }
    fn fold_cmpop(&mut self, node: Cmpop) -> Result<Cmpop, Self::Error> {
        fold_cmpop(self, node)
    }
    fn fold_comprehension(
        &mut self,
        node: Comprehension<U>,
    ) -> Result<Comprehension<Self::TargetU>, Self::Error> {
        fold_comprehension(self, node)
    }
    fn fold_excepthandler(
        &mut self,
        node: Excepthandler<U>,
    ) -> Result<Excepthandler<Self::TargetU>, Self::Error> {
        fold_excepthandler(self, node)
    }
    fn fold_arguments(
        &mut self,
        node: Arguments<U>,
    ) -> Result<Arguments<Self::TargetU>, Self::Error> {
        fold_arguments(self, node)
    }
    fn fold_arg(&mut self, node: Arg<U>) -> Result<Arg<Self::TargetU>, Self::Error> {
        fold_arg(self, node)
    }
    fn fold_keyword(&mut self, node: Keyword<U>) -> Result<Keyword<Self::TargetU>, Self::Error> {
        fold_keyword(self, node)
    }
    fn fold_alias(&mut self, node: Alias<U>) -> Result<Alias<Self::TargetU>, Self::Error> {
        fold_alias(self, node)
    }
    fn fold_withitem(&mut self, node: Withitem<U>) -> Result<Withitem<Self::TargetU>, Self::Error> {
        fold_withitem(self, node)
    }
    fn fold_match_case(
        &mut self,
        node: MatchCase<U>,
    ) -> Result<MatchCase<Self::TargetU>, Self::Error> {
        fold_match_case(self, node)
    }
    fn fold_pattern(&mut self, node: Pattern<U>) -> Result<Pattern<Self::TargetU>, Self::Error> {
        fold_pattern(self, node)
    }
    fn fold_type_ignore(&mut self, node: TypeIgnore) -> Result<TypeIgnore, Self::Error> {
        fold_type_ignore(self, node)
    }
}
fn fold_attributed<U, F: Fold<U> + ?Sized, T, MT>(
    folder: &mut F,
    node: Attributed<T, U>,
    f: impl FnOnce(&mut F, T) -> Result<MT, F::Error>,
) -> Result<Attributed<MT, F::TargetU>, F::Error>
where
    T: Ranged,
{
    let node = folder.map_attributed(node)?;
    Ok(Attributed {
        custom: node.custom,
        node: f(folder, node.node)?,
    })
}
impl<T, U> Foldable<T, U> for Mod<T> {
    type Mapped = Mod<U>;
    fn fold<F: Fold<T, TargetU = U> + ?Sized>(
        self,
        folder: &mut F,
    ) -> Result<Self::Mapped, F::Error> {
        folder.fold_mod(self)
    }
}
pub fn fold_mod<U, F: Fold<U> + ?Sized>(
    #[allow(unused)] folder: &mut F,
    node: Mod<U>,
) -> Result<Mod<F::TargetU>, F::Error> {
    match node {
        Mod::Module(ModModule {
            range,
            body,
            type_ignores,
        }) => Ok(Mod::Module(ModModule {
            body: Foldable::fold(body, folder)?,
            type_ignores: Foldable::fold(type_ignores, folder)?,
            range,
        })),
        Mod::Interactive(ModInteractive { range, body }) => Ok(Mod::Interactive(ModInteractive {
            body: Foldable::fold(body, folder)?,
            range,
        })),
        Mod::Expression(ModExpression { range, body }) => Ok(Mod::Expression(ModExpression {
            body: Foldable::fold(body, folder)?,
            range,
        })),
        Mod::FunctionType(ModFunctionType {
            range,
            argtypes,
            returns,
        }) => Ok(Mod::FunctionType(ModFunctionType {
            argtypes: Foldable::fold(argtypes, folder)?,
            returns: Foldable::fold(returns, folder)?,
            range,
        })),
    }
}
impl<T, U> Foldable<T, U> for Stmt<T> {
    type Mapped = Stmt<U>;
    fn fold<F: Fold<T, TargetU = U> + ?Sized>(
        self,
        folder: &mut F,
    ) -> Result<Self::Mapped, F::Error> {
        folder.fold_stmt(self)
    }
}
pub fn fold_stmt<U, F: Fold<U> + ?Sized>(
    #[allow(unused)] folder: &mut F,
    node: Stmt<U>,
) -> Result<Stmt<F::TargetU>, F::Error> {
    fold_attributed(folder, node, |folder, node| match node {
        Stmt::FunctionDef(StmtFunctionDef {
            range,
            name,
            args,
            body,
            decorator_list,
            returns,
            type_comment,
        }) => Ok(Stmt::FunctionDef(StmtFunctionDef {
            name: Foldable::fold(name, folder)?,
            args: Foldable::fold(args, folder)?,
            body: Foldable::fold(body, folder)?,
            decorator_list: Foldable::fold(decorator_list, folder)?,
            returns: Foldable::fold(returns, folder)?,
            type_comment: Foldable::fold(type_comment, folder)?,
            range,
        })),
        Stmt::AsyncFunctionDef(StmtAsyncFunctionDef {
            range,
            name,
            args,
            body,
            decorator_list,
            returns,
            type_comment,
        }) => Ok(Stmt::AsyncFunctionDef(StmtAsyncFunctionDef {
            name: Foldable::fold(name, folder)?,
            args: Foldable::fold(args, folder)?,
            body: Foldable::fold(body, folder)?,
            decorator_list: Foldable::fold(decorator_list, folder)?,
            returns: Foldable::fold(returns, folder)?,
            type_comment: Foldable::fold(type_comment, folder)?,
            range,
        })),
        Stmt::ClassDef(StmtClassDef {
            range,
            name,
            bases,
            keywords,
            body,
            decorator_list,
        }) => Ok(Stmt::ClassDef(StmtClassDef {
            name: Foldable::fold(name, folder)?,
            bases: Foldable::fold(bases, folder)?,
            keywords: Foldable::fold(keywords, folder)?,
            body: Foldable::fold(body, folder)?,
            decorator_list: Foldable::fold(decorator_list, folder)?,
            range,
        })),
        Stmt::Return(StmtReturn { range, value }) => Ok(Stmt::Return(StmtReturn {
            value: Foldable::fold(value, folder)?,
            range,
        })),
        Stmt::Delete(StmtDelete { range, targets }) => Ok(Stmt::Delete(StmtDelete {
            targets: Foldable::fold(targets, folder)?,
            range,
        })),
        Stmt::Assign(StmtAssign {
            range,
            targets,
            value,
            type_comment,
        }) => Ok(Stmt::Assign(StmtAssign {
            targets: Foldable::fold(targets, folder)?,
            value: Foldable::fold(value, folder)?,
            type_comment: Foldable::fold(type_comment, folder)?,
            range,
        })),
        Stmt::AugAssign(StmtAugAssign {
            range,
            target,
            op,
            value,
        }) => Ok(Stmt::AugAssign(StmtAugAssign {
            target: Foldable::fold(target, folder)?,
            op: Foldable::fold(op, folder)?,
            value: Foldable::fold(value, folder)?,
            range,
        })),
        Stmt::AnnAssign(StmtAnnAssign {
            range,
            target,
            annotation,
            value,
            simple,
        }) => Ok(Stmt::AnnAssign(StmtAnnAssign {
            target: Foldable::fold(target, folder)?,
            annotation: Foldable::fold(annotation, folder)?,
            value: Foldable::fold(value, folder)?,
            simple: Foldable::fold(simple, folder)?,
            range,
        })),
        Stmt::For(StmtFor {
            range,
            target,
            iter,
            body,
            orelse,
            type_comment,
        }) => Ok(Stmt::For(StmtFor {
            target: Foldable::fold(target, folder)?,
            iter: Foldable::fold(iter, folder)?,
            body: Foldable::fold(body, folder)?,
            orelse: Foldable::fold(orelse, folder)?,
            type_comment: Foldable::fold(type_comment, folder)?,
            range,
        })),
        Stmt::AsyncFor(StmtAsyncFor {
            range,
            target,
            iter,
            body,
            orelse,
            type_comment,
        }) => Ok(Stmt::AsyncFor(StmtAsyncFor {
            target: Foldable::fold(target, folder)?,
            iter: Foldable::fold(iter, folder)?,
            body: Foldable::fold(body, folder)?,
            orelse: Foldable::fold(orelse, folder)?,
            type_comment: Foldable::fold(type_comment, folder)?,
            range,
        })),
        Stmt::While(StmtWhile {
            range,
            test,
            body,
            orelse,
        }) => Ok(Stmt::While(StmtWhile {
            test: Foldable::fold(test, folder)?,
            body: Foldable::fold(body, folder)?,
            orelse: Foldable::fold(orelse, folder)?,
            range,
        })),
        Stmt::If(StmtIf {
            range,
            test,
            body,
            orelse,
        }) => Ok(Stmt::If(StmtIf {
            test: Foldable::fold(test, folder)?,
            body: Foldable::fold(body, folder)?,
            orelse: Foldable::fold(orelse, folder)?,
            range,
        })),
        Stmt::With(StmtWith {
            range,
            items,
            body,
            type_comment,
        }) => Ok(Stmt::With(StmtWith {
            items: Foldable::fold(items, folder)?,
            body: Foldable::fold(body, folder)?,
            type_comment: Foldable::fold(type_comment, folder)?,
            range,
        })),
        Stmt::AsyncWith(StmtAsyncWith {
            range,
            items,
            body,
            type_comment,
        }) => Ok(Stmt::AsyncWith(StmtAsyncWith {
            items: Foldable::fold(items, folder)?,
            body: Foldable::fold(body, folder)?,
            type_comment: Foldable::fold(type_comment, folder)?,
            range,
        })),
        Stmt::Match(StmtMatch {
            range,
            subject,
            cases,
        }) => Ok(Stmt::Match(StmtMatch {
            subject: Foldable::fold(subject, folder)?,
            cases: Foldable::fold(cases, folder)?,
            range,
        })),
        Stmt::Raise(StmtRaise { range, exc, cause }) => Ok(Stmt::Raise(StmtRaise {
            exc: Foldable::fold(exc, folder)?,
            cause: Foldable::fold(cause, folder)?,
            range,
        })),
        Stmt::Try(StmtTry {
            range,
            body,
            handlers,
            orelse,
            finalbody,
        }) => Ok(Stmt::Try(StmtTry {
            body: Foldable::fold(body, folder)?,
            handlers: Foldable::fold(handlers, folder)?,
            orelse: Foldable::fold(orelse, folder)?,
            finalbody: Foldable::fold(finalbody, folder)?,
            range,
        })),
        Stmt::TryStar(StmtTryStar {
            range,
            body,
            handlers,
            orelse,
            finalbody,
        }) => Ok(Stmt::TryStar(StmtTryStar {
            body: Foldable::fold(body, folder)?,
            handlers: Foldable::fold(handlers, folder)?,
            orelse: Foldable::fold(orelse, folder)?,
            finalbody: Foldable::fold(finalbody, folder)?,
            range,
        })),
        Stmt::Assert(StmtAssert { range, test, msg }) => Ok(Stmt::Assert(StmtAssert {
            test: Foldable::fold(test, folder)?,
            msg: Foldable::fold(msg, folder)?,
            range,
        })),
        Stmt::Import(StmtImport { range, names }) => Ok(Stmt::Import(StmtImport {
            names: Foldable::fold(names, folder)?,
            range,
        })),
        Stmt::ImportFrom(StmtImportFrom {
            range,
            module,
            names,
            level,
        }) => Ok(Stmt::ImportFrom(StmtImportFrom {
            module: Foldable::fold(module, folder)?,
            names: Foldable::fold(names, folder)?,
            level: Foldable::fold(level, folder)?,
            range,
        })),
        Stmt::Global(StmtGlobal { range, names }) => Ok(Stmt::Global(StmtGlobal {
            names: Foldable::fold(names, folder)?,
            range,
        })),
        Stmt::Nonlocal(StmtNonlocal { range, names }) => Ok(Stmt::Nonlocal(StmtNonlocal {
            names: Foldable::fold(names, folder)?,
            range,
        })),
        Stmt::Expr(StmtExpr { range, value }) => Ok(Stmt::Expr(StmtExpr {
            value: Foldable::fold(value, folder)?,
            range,
        })),
        Stmt::Pass(StmtPass { range }) => Ok(Stmt::Pass(StmtPass { range })),
        Stmt::Break(StmtBreak { range }) => Ok(Stmt::Break(StmtBreak { range })),
        Stmt::Continue(StmtContinue { range }) => Ok(Stmt::Continue(StmtContinue { range })),
    })
}
impl<T, U> Foldable<T, U> for Expr<T> {
    type Mapped = Expr<U>;
    fn fold<F: Fold<T, TargetU = U> + ?Sized>(
        self,
        folder: &mut F,
    ) -> Result<Self::Mapped, F::Error> {
        folder.fold_expr(self)
    }
}
pub fn fold_expr<U, F: Fold<U> + ?Sized>(
    #[allow(unused)] folder: &mut F,
    node: Expr<U>,
) -> Result<Expr<F::TargetU>, F::Error> {
    fold_attributed(folder, node, |folder, node| match node {
        Expr::BoolOp(ExprBoolOp { range, op, values }) => Ok(Expr::BoolOp(ExprBoolOp {
            op: Foldable::fold(op, folder)?,
            values: Foldable::fold(values, folder)?,
            range,
        })),
        Expr::NamedExpr(ExprNamedExpr {
            range,
            target,
            value,
        }) => Ok(Expr::NamedExpr(ExprNamedExpr {
            target: Foldable::fold(target, folder)?,
            value: Foldable::fold(value, folder)?,
            range,
        })),
        Expr::BinOp(ExprBinOp {
            range,
            left,
            op,
            right,
        }) => Ok(Expr::BinOp(ExprBinOp {
            left: Foldable::fold(left, folder)?,
            op: Foldable::fold(op, folder)?,
            right: Foldable::fold(right, folder)?,
            range,
        })),
        Expr::UnaryOp(ExprUnaryOp { range, op, operand }) => Ok(Expr::UnaryOp(ExprUnaryOp {
            op: Foldable::fold(op, folder)?,
            operand: Foldable::fold(operand, folder)?,
            range,
        })),
        Expr::Lambda(ExprLambda { range, args, body }) => Ok(Expr::Lambda(ExprLambda {
            args: Foldable::fold(args, folder)?,
            body: Foldable::fold(body, folder)?,
            range,
        })),
        Expr::IfExp(ExprIfExp {
            range,
            test,
            body,
            orelse,
        }) => Ok(Expr::IfExp(ExprIfExp {
            test: Foldable::fold(test, folder)?,
            body: Foldable::fold(body, folder)?,
            orelse: Foldable::fold(orelse, folder)?,
            range,
        })),
        Expr::Dict(ExprDict {
            range,
            keys,
            values,
        }) => Ok(Expr::Dict(ExprDict {
            keys: Foldable::fold(keys, folder)?,
            values: Foldable::fold(values, folder)?,
            range,
        })),
        Expr::Set(ExprSet { range, elts }) => Ok(Expr::Set(ExprSet {
            elts: Foldable::fold(elts, folder)?,
            range,
        })),
        Expr::ListComp(ExprListComp {
            range,
            elt,
            generators,
        }) => Ok(Expr::ListComp(ExprListComp {
            elt: Foldable::fold(elt, folder)?,
            generators: Foldable::fold(generators, folder)?,
            range,
        })),
        Expr::SetComp(ExprSetComp {
            range,
            elt,
            generators,
        }) => Ok(Expr::SetComp(ExprSetComp {
            elt: Foldable::fold(elt, folder)?,
            generators: Foldable::fold(generators, folder)?,
            range,
        })),
        Expr::DictComp(ExprDictComp {
            range,
            key,
            value,
            generators,
        }) => Ok(Expr::DictComp(ExprDictComp {
            key: Foldable::fold(key, folder)?,
            value: Foldable::fold(value, folder)?,
            generators: Foldable::fold(generators, folder)?,
            range,
        })),
        Expr::GeneratorExp(ExprGeneratorExp {
            range,
            elt,
            generators,
        }) => Ok(Expr::GeneratorExp(ExprGeneratorExp {
            elt: Foldable::fold(elt, folder)?,
            generators: Foldable::fold(generators, folder)?,
            range,
        })),
        Expr::Await(ExprAwait { range, value }) => Ok(Expr::Await(ExprAwait {
            value: Foldable::fold(value, folder)?,
            range,
        })),
        Expr::Yield(ExprYield { range, value }) => Ok(Expr::Yield(ExprYield {
            value: Foldable::fold(value, folder)?,
            range,
        })),
        Expr::YieldFrom(ExprYieldFrom { range, value }) => Ok(Expr::YieldFrom(ExprYieldFrom {
            value: Foldable::fold(value, folder)?,
            range,
        })),
        Expr::Compare(ExprCompare {
            range,
            left,
            ops,
            comparators,
        }) => Ok(Expr::Compare(ExprCompare {
            left: Foldable::fold(left, folder)?,
            ops: Foldable::fold(ops, folder)?,
            comparators: Foldable::fold(comparators, folder)?,
            range,
        })),
        Expr::Call(ExprCall {
            range,
            func,
            args,
            keywords,
        }) => Ok(Expr::Call(ExprCall {
            func: Foldable::fold(func, folder)?,
            args: Foldable::fold(args, folder)?,
            keywords: Foldable::fold(keywords, folder)?,
            range,
        })),
        Expr::FormattedValue(ExprFormattedValue {
            range,
            value,
            conversion,
            format_spec,
        }) => Ok(Expr::FormattedValue(ExprFormattedValue {
            value: Foldable::fold(value, folder)?,
            conversion: Foldable::fold(conversion, folder)?,
            format_spec: Foldable::fold(format_spec, folder)?,
            range,
        })),
        Expr::JoinedStr(ExprJoinedStr { range, values }) => Ok(Expr::JoinedStr(ExprJoinedStr {
            values: Foldable::fold(values, folder)?,
            range,
        })),
        Expr::Constant(ExprConstant { range, value, kind }) => Ok(Expr::Constant(ExprConstant {
            value: Foldable::fold(value, folder)?,
            kind: Foldable::fold(kind, folder)?,
            range,
        })),
        Expr::Attribute(ExprAttribute {
            range,
            value,
            attr,
            ctx,
        }) => Ok(Expr::Attribute(ExprAttribute {
            value: Foldable::fold(value, folder)?,
            attr: Foldable::fold(attr, folder)?,
            ctx: Foldable::fold(ctx, folder)?,
            range,
        })),
        Expr::Subscript(ExprSubscript {
            range,
            value,
            slice,
            ctx,
        }) => Ok(Expr::Subscript(ExprSubscript {
            value: Foldable::fold(value, folder)?,
            slice: Foldable::fold(slice, folder)?,
            ctx: Foldable::fold(ctx, folder)?,
            range,
        })),
        Expr::Starred(ExprStarred { range, value, ctx }) => Ok(Expr::Starred(ExprStarred {
            value: Foldable::fold(value, folder)?,
            ctx: Foldable::fold(ctx, folder)?,
            range,
        })),
        Expr::Name(ExprName { range, id, ctx }) => Ok(Expr::Name(ExprName {
            id: Foldable::fold(id, folder)?,
            ctx: Foldable::fold(ctx, folder)?,
            range,
        })),
        Expr::List(ExprList { range, elts, ctx }) => Ok(Expr::List(ExprList {
            elts: Foldable::fold(elts, folder)?,
            ctx: Foldable::fold(ctx, folder)?,
            range,
        })),
        Expr::Tuple(ExprTuple { range, elts, ctx }) => Ok(Expr::Tuple(ExprTuple {
            elts: Foldable::fold(elts, folder)?,
            ctx: Foldable::fold(ctx, folder)?,
            range,
        })),
        Expr::Slice(ExprSlice {
            range,
            lower,
            upper,
            step,
        }) => Ok(Expr::Slice(ExprSlice {
            lower: Foldable::fold(lower, folder)?,
            upper: Foldable::fold(upper, folder)?,
            step: Foldable::fold(step, folder)?,
            range,
        })),
    })
}
impl<T, U> Foldable<T, U> for ExprContext {
    type Mapped = ExprContext;
    fn fold<F: Fold<T, TargetU = U> + ?Sized>(
        self,
        folder: &mut F,
    ) -> Result<Self::Mapped, F::Error> {
        folder.fold_expr_context(self)
    }
}
pub fn fold_expr_context<U, F: Fold<U> + ?Sized>(
    #[allow(unused)] folder: &mut F,
    node: ExprContext,
) -> Result<ExprContext, F::Error> {
    match node {
        ExprContext::Load {} => Ok(ExprContext::Load {}),
        ExprContext::Store {} => Ok(ExprContext::Store {}),
        ExprContext::Del {} => Ok(ExprContext::Del {}),
    }
}
impl<T, U> Foldable<T, U> for Boolop {
    type Mapped = Boolop;
    fn fold<F: Fold<T, TargetU = U> + ?Sized>(
        self,
        folder: &mut F,
    ) -> Result<Self::Mapped, F::Error> {
        folder.fold_boolop(self)
    }
}
pub fn fold_boolop<U, F: Fold<U> + ?Sized>(
    #[allow(unused)] folder: &mut F,
    node: Boolop,
) -> Result<Boolop, F::Error> {
    match node {
        Boolop::And {} => Ok(Boolop::And {}),
        Boolop::Or {} => Ok(Boolop::Or {}),
    }
}
impl<T, U> Foldable<T, U> for Operator {
    type Mapped = Operator;
    fn fold<F: Fold<T, TargetU = U> + ?Sized>(
        self,
        folder: &mut F,
    ) -> Result<Self::Mapped, F::Error> {
        folder.fold_operator(self)
    }
}
pub fn fold_operator<U, F: Fold<U> + ?Sized>(
    #[allow(unused)] folder: &mut F,
    node: Operator,
) -> Result<Operator, F::Error> {
    match node {
        Operator::Add {} => Ok(Operator::Add {}),
        Operator::Sub {} => Ok(Operator::Sub {}),
        Operator::Mult {} => Ok(Operator::Mult {}),
        Operator::MatMult {} => Ok(Operator::MatMult {}),
        Operator::Div {} => Ok(Operator::Div {}),
        Operator::Mod {} => Ok(Operator::Mod {}),
        Operator::Pow {} => Ok(Operator::Pow {}),
        Operator::LShift {} => Ok(Operator::LShift {}),
        Operator::RShift {} => Ok(Operator::RShift {}),
        Operator::BitOr {} => Ok(Operator::BitOr {}),
        Operator::BitXor {} => Ok(Operator::BitXor {}),
        Operator::BitAnd {} => Ok(Operator::BitAnd {}),
        Operator::FloorDiv {} => Ok(Operator::FloorDiv {}),
    }
}
impl<T, U> Foldable<T, U> for Unaryop {
    type Mapped = Unaryop;
    fn fold<F: Fold<T, TargetU = U> + ?Sized>(
        self,
        folder: &mut F,
    ) -> Result<Self::Mapped, F::Error> {
        folder.fold_unaryop(self)
    }
}
pub fn fold_unaryop<U, F: Fold<U> + ?Sized>(
    #[allow(unused)] folder: &mut F,
    node: Unaryop,
) -> Result<Unaryop, F::Error> {
    match node {
        Unaryop::Invert {} => Ok(Unaryop::Invert {}),
        Unaryop::Not {} => Ok(Unaryop::Not {}),
        Unaryop::UAdd {} => Ok(Unaryop::UAdd {}),
        Unaryop::USub {} => Ok(Unaryop::USub {}),
    }
}
impl<T, U> Foldable<T, U> for Cmpop {
    type Mapped = Cmpop;
    fn fold<F: Fold<T, TargetU = U> + ?Sized>(
        self,
        folder: &mut F,
    ) -> Result<Self::Mapped, F::Error> {
        folder.fold_cmpop(self)
    }
}
pub fn fold_cmpop<U, F: Fold<U> + ?Sized>(
    #[allow(unused)] folder: &mut F,
    node: Cmpop,
) -> Result<Cmpop, F::Error> {
    match node {
        Cmpop::Eq {} => Ok(Cmpop::Eq {}),
        Cmpop::NotEq {} => Ok(Cmpop::NotEq {}),
        Cmpop::Lt {} => Ok(Cmpop::Lt {}),
        Cmpop::LtE {} => Ok(Cmpop::LtE {}),
        Cmpop::Gt {} => Ok(Cmpop::Gt {}),
        Cmpop::GtE {} => Ok(Cmpop::GtE {}),
        Cmpop::Is {} => Ok(Cmpop::Is {}),
        Cmpop::IsNot {} => Ok(Cmpop::IsNot {}),
        Cmpop::In {} => Ok(Cmpop::In {}),
        Cmpop::NotIn {} => Ok(Cmpop::NotIn {}),
    }
}
impl<T, U> Foldable<T, U> for Comprehension<T> {
    type Mapped = Comprehension<U>;
    fn fold<F: Fold<T, TargetU = U> + ?Sized>(
        self,
        folder: &mut F,
    ) -> Result<Self::Mapped, F::Error> {
        folder.fold_comprehension(self)
    }
}
pub fn fold_comprehension<U, F: Fold<U> + ?Sized>(
    #[allow(unused)] folder: &mut F,
    node: Comprehension<U>,
) -> Result<Comprehension<F::TargetU>, F::Error> {
    let Comprehension {
        range,
        target,
        iter,
        ifs,
        is_async,
    } = node;
    Ok(Comprehension {
        target: Foldable::fold(target, folder)?,
        iter: Foldable::fold(iter, folder)?,
        ifs: Foldable::fold(ifs, folder)?,
        is_async: Foldable::fold(is_async, folder)?,
        range,
    })
}
impl<T, U> Foldable<T, U> for Excepthandler<T> {
    type Mapped = Excepthandler<U>;
    fn fold<F: Fold<T, TargetU = U> + ?Sized>(
        self,
        folder: &mut F,
    ) -> Result<Self::Mapped, F::Error> {
        folder.fold_excepthandler(self)
    }
}
pub fn fold_excepthandler<U, F: Fold<U> + ?Sized>(
    #[allow(unused)] folder: &mut F,
    node: Excepthandler<U>,
) -> Result<Excepthandler<F::TargetU>, F::Error> {
    fold_attributed(folder, node, |folder, node| match node {
        Excepthandler::ExceptHandler(ExcepthandlerExceptHandler {
            range,
            type_,
            name,
            body,
        }) => Ok(Excepthandler::ExceptHandler(ExcepthandlerExceptHandler {
            type_: Foldable::fold(type_, folder)?,
            name: Foldable::fold(name, folder)?,
            body: Foldable::fold(body, folder)?,
            range,
        })),
    })
}
impl<T, U> Foldable<T, U> for Arguments<T> {
    type Mapped = Arguments<U>;
    fn fold<F: Fold<T, TargetU = U> + ?Sized>(
        self,
        folder: &mut F,
    ) -> Result<Self::Mapped, F::Error> {
        folder.fold_arguments(self)
    }
}
pub fn fold_arguments<U, F: Fold<U> + ?Sized>(
    #[allow(unused)] folder: &mut F,
    node: Arguments<U>,
) -> Result<Arguments<F::TargetU>, F::Error> {
    let Arguments {
        range,
        posonlyargs,
        args,
        vararg,
        kwonlyargs,
        kw_defaults,
        kwarg,
        defaults,
    } = node;
    Ok(Arguments {
        posonlyargs: Foldable::fold(posonlyargs, folder)?,
        args: Foldable::fold(args, folder)?,
        vararg: Foldable::fold(vararg, folder)?,
        kwonlyargs: Foldable::fold(kwonlyargs, folder)?,
        kw_defaults: Foldable::fold(kw_defaults, folder)?,
        kwarg: Foldable::fold(kwarg, folder)?,
        defaults: Foldable::fold(defaults, folder)?,
        range,
    })
}
impl<T, U> Foldable<T, U> for Arg<T> {
    type Mapped = Arg<U>;
    fn fold<F: Fold<T, TargetU = U> + ?Sized>(
        self,
        folder: &mut F,
    ) -> Result<Self::Mapped, F::Error> {
        folder.fold_arg(self)
    }
}
pub fn fold_arg<U, F: Fold<U> + ?Sized>(
    #[allow(unused)] folder: &mut F,
    node: Arg<U>,
) -> Result<Arg<F::TargetU>, F::Error> {
    fold_attributed(folder, node, |folder, node| {
        let ArgData {
            range,
            arg,
            annotation,
            type_comment,
        } = node;
        Ok(ArgData {
            arg: Foldable::fold(arg, folder)?,
            annotation: Foldable::fold(annotation, folder)?,
            type_comment: Foldable::fold(type_comment, folder)?,
            range,
        })
    })
}
impl<T, U> Foldable<T, U> for Keyword<T> {
    type Mapped = Keyword<U>;
    fn fold<F: Fold<T, TargetU = U> + ?Sized>(
        self,
        folder: &mut F,
    ) -> Result<Self::Mapped, F::Error> {
        folder.fold_keyword(self)
    }
}
pub fn fold_keyword<U, F: Fold<U> + ?Sized>(
    #[allow(unused)] folder: &mut F,
    node: Keyword<U>,
) -> Result<Keyword<F::TargetU>, F::Error> {
    fold_attributed(folder, node, |folder, node| {
        let KeywordData { range, arg, value } = node;
        Ok(KeywordData {
            arg: Foldable::fold(arg, folder)?,
            value: Foldable::fold(value, folder)?,
            range,
        })
    })
}
impl<T, U> Foldable<T, U> for Alias<T> {
    type Mapped = Alias<U>;
    fn fold<F: Fold<T, TargetU = U> + ?Sized>(
        self,
        folder: &mut F,
    ) -> Result<Self::Mapped, F::Error> {
        folder.fold_alias(self)
    }
}
pub fn fold_alias<U, F: Fold<U> + ?Sized>(
    #[allow(unused)] folder: &mut F,
    node: Alias<U>,
) -> Result<Alias<F::TargetU>, F::Error> {
    fold_attributed(folder, node, |folder, node| {
        let AliasData {
            range,
            name,
            asname,
        } = node;
        Ok(AliasData {
            name: Foldable::fold(name, folder)?,
            asname: Foldable::fold(asname, folder)?,
            range,
        })
    })
}
impl<T, U> Foldable<T, U> for Withitem<T> {
    type Mapped = Withitem<U>;
    fn fold<F: Fold<T, TargetU = U> + ?Sized>(
        self,
        folder: &mut F,
    ) -> Result<Self::Mapped, F::Error> {
        folder.fold_withitem(self)
    }
}
pub fn fold_withitem<U, F: Fold<U> + ?Sized>(
    #[allow(unused)] folder: &mut F,
    node: Withitem<U>,
) -> Result<Withitem<F::TargetU>, F::Error> {
    let Withitem {
        range,
        context_expr,
        optional_vars,
    } = node;
    Ok(Withitem {
        context_expr: Foldable::fold(context_expr, folder)?,
        optional_vars: Foldable::fold(optional_vars, folder)?,
        range,
    })
}
impl<T, U> Foldable<T, U> for MatchCase<T> {
    type Mapped = MatchCase<U>;
    fn fold<F: Fold<T, TargetU = U> + ?Sized>(
        self,
        folder: &mut F,
    ) -> Result<Self::Mapped, F::Error> {
        folder.fold_match_case(self)
    }
}
pub fn fold_match_case<U, F: Fold<U> + ?Sized>(
    #[allow(unused)] folder: &mut F,
    node: MatchCase<U>,
) -> Result<MatchCase<F::TargetU>, F::Error> {
    let MatchCase {
        range,
        pattern,
        guard,
        body,
    } = node;
    Ok(MatchCase {
        pattern: Foldable::fold(pattern, folder)?,
        guard: Foldable::fold(guard, folder)?,
        body: Foldable::fold(body, folder)?,
        range,
    })
}
impl<T, U> Foldable<T, U> for Pattern<T> {
    type Mapped = Pattern<U>;
    fn fold<F: Fold<T, TargetU = U> + ?Sized>(
        self,
        folder: &mut F,
    ) -> Result<Self::Mapped, F::Error> {
        folder.fold_pattern(self)
    }
}
pub fn fold_pattern<U, F: Fold<U> + ?Sized>(
    #[allow(unused)] folder: &mut F,
    node: Pattern<U>,
) -> Result<Pattern<F::TargetU>, F::Error> {
    fold_attributed(folder, node, |folder, node| match node {
        Pattern::MatchValue(PatternMatchValue { range, value }) => {
            Ok(Pattern::MatchValue(PatternMatchValue {
                value: Foldable::fold(value, folder)?,
                range,
            }))
        }
        Pattern::MatchSingleton(PatternMatchSingleton { range, value }) => {
            Ok(Pattern::MatchSingleton(PatternMatchSingleton {
                value: Foldable::fold(value, folder)?,
                range,
            }))
        }
        Pattern::MatchSequence(PatternMatchSequence { range, patterns }) => {
            Ok(Pattern::MatchSequence(PatternMatchSequence {
                patterns: Foldable::fold(patterns, folder)?,
                range,
            }))
        }
        Pattern::MatchMapping(PatternMatchMapping {
            range,
            keys,
            patterns,
            rest,
        }) => Ok(Pattern::MatchMapping(PatternMatchMapping {
            keys: Foldable::fold(keys, folder)?,
            patterns: Foldable::fold(patterns, folder)?,
            rest: Foldable::fold(rest, folder)?,
            range,
        })),
        Pattern::MatchClass(PatternMatchClass {
            range,
            cls,
            patterns,
            kwd_attrs,
            kwd_patterns,
        }) => Ok(Pattern::MatchClass(PatternMatchClass {
            cls: Foldable::fold(cls, folder)?,
            patterns: Foldable::fold(patterns, folder)?,
            kwd_attrs: Foldable::fold(kwd_attrs, folder)?,
            kwd_patterns: Foldable::fold(kwd_patterns, folder)?,
            range,
        })),
        Pattern::MatchStar(PatternMatchStar { range, name }) => {
            Ok(Pattern::MatchStar(PatternMatchStar {
                name: Foldable::fold(name, folder)?,
                range,
            }))
        }
        Pattern::MatchAs(PatternMatchAs {
            range,
            pattern,
            name,
        }) => Ok(Pattern::MatchAs(PatternMatchAs {
            pattern: Foldable::fold(pattern, folder)?,
            name: Foldable::fold(name, folder)?,
            range,
        })),
        Pattern::MatchOr(PatternMatchOr { range, patterns }) => {
            Ok(Pattern::MatchOr(PatternMatchOr {
                patterns: Foldable::fold(patterns, folder)?,
                range,
            }))
        }
    })
}
impl<T, U> Foldable<T, U> for TypeIgnore {
    type Mapped = TypeIgnore;
    fn fold<F: Fold<T, TargetU = U> + ?Sized>(
        self,
        folder: &mut F,
    ) -> Result<Self::Mapped, F::Error> {
        folder.fold_type_ignore(self)
    }
}
pub fn fold_type_ignore<U, F: Fold<U> + ?Sized>(
    #[allow(unused)] folder: &mut F,
    node: TypeIgnore,
) -> Result<TypeIgnore, F::Error> {
    match node {
        TypeIgnore::TypeIgnore(TypeIgnoreTypeIgnore { range, lineno, tag }) => {
            Ok(TypeIgnore::TypeIgnore(TypeIgnoreTypeIgnore {
                lineno: Foldable::fold(lineno, folder)?,
                tag: Foldable::fold(tag, folder)?,
                range,
            }))
        }
    }
}
