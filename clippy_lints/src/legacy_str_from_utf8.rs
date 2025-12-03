use clippy_config::Conf;
use clippy_utils::diagnostics::{span_lint_and_help, span_lint_and_sugg};
use clippy_utils::msrvs::{self, Msrv};
use clippy_utils::source::SpanRangeExt;
use rustc_errors::Applicability;
use rustc_hir::{ExprKind, Item, ItemKind, PathSegment, QPath, UseKind};
use rustc_lint::{LateContext, LateLintPass, LintContext};
use rustc_session::impl_lint_pass;
use rustc_span::Symbol;
use rustc_span::sym::{self};

declare_clippy_lint! {
    /// ### What it does
    /// Checks for usage of `core::str::from_utf8`, `core::str::from_utf8_mut`,
    /// `core::str::from_utf8_unchecked`, or `core::str::from_utf8_unchecked_mut`.
    ///
    /// ### Why is this bad?
    /// These can be instead be imported from their publicly
    /// avalible re-export in `std`, such as `std::str::from_utf8`.
    /// These legacy items may be deprecated in a future version of rust.
    ///
    /// ### Example
    /// ```rust
    /// let crab = core::str::from_utf8(&[0xF0, 0x9F, 0xA6, 0x80])?;
    /// ```
    /// Use instead:
    /// ```rust
    /// let crab = str::from_utf8(&[0xF0, 0x9F, 0xA6, 0x80])?;
    /// ```
    ///
    /// ### Example
    /// ```rust
    /// use core::str::from_utf8;
    /// ```
    /// Use instead:
    /// ```rust
    /// use std::str::from_utf8;
    /// ```
    #[clippy::version = "1.93.0"]
    pub LEGACY_STR_FROM_UTF8,
    style,
    "checks for usage of legacy `core::str::from_utf8`, `core::str::from_utf8_mut`, `core::str::from_utf8_unchecked`, or `core::str::from_utf8_unchecked_mut`"
}
pub struct LegacyStrFromUtf8 {
    msrv: Msrv,
}

impl LegacyStrFromUtf8 {
    pub fn new(conf: &'static Conf) -> Self {
        Self { msrv: conf.msrv }
    }
}

impl_lint_pass!(LegacyStrFromUtf8 => [LEGACY_STR_FROM_UTF8]);

impl<'tcx> LateLintPass<'tcx> for LegacyStrFromUtf8 {
    fn check_item(&mut self, cx: &LateContext<'tcx>, item: &'tcx Item<'tcx>) {
        if !self.msrv.meets(cx, msrvs::STR_FROM_UTF8) {
            return;
        }

        // lint on the `use` statement directly.
        let ItemKind::Use(path, UseKind::Single(_)) = item.kind else {
            return;
        };

        // Do not lint if we are inside an external macro.
        if item.span.in_external_macro(cx.sess().source_map()) {
            return;
        }

        if !should_lint_on_path(path.segments) {
            return;
        }

        let Some(func) = path.segments.last() else {
            return;
        };
        let func = func.ident.name.as_str();

        if path
            .span
            .check_source_text(cx, |use_stmt| use_stmt == format!("core::str::{func}"))
        {
            // Emit a suggestion if this is a "simple" use statement.
            span_lint_and_sugg(
                cx,
                LEGACY_STR_FROM_UTF8,
                path.span,
                format!("importing legacy `core::str::{func}` function"),
                "consider instead importing the item from `std`",
                format!("std::str::{func}"),
                Applicability::MachineApplicable,
            );
        } else {
            // Else emit a help message.
            span_lint_and_help(
                cx,
                LEGACY_STR_FROM_UTF8,
                path.span,
                format!("importing legacy `core::str::{func}` function"),
                None,
                format!("consider instead importing the item from `std::str::{func}`"),
            );
        }
    }

    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx rustc_hir::Expr<'tcx>) {
        if !self.msrv.meets(cx, msrvs::STR_FROM_UTF8) {
            return;
        }

        // If the expression is a function call,
        let ExprKind::Call(expr, _) = expr.kind else {
            return;
        };

        // Do not lint if we are inside an external macro.
        if expr.span.in_external_macro(cx.sess().source_map()) {
            return;
        }

        // And call is of the form `<T>::something`
        // Here, <T> is <str>
        let ExprKind::Path(QPath::Resolved(_, path)) = expr.kind else {
            return;
        };

        if !should_lint_on_path(path.segments) {
            return;
        }

        let Some(func) = path.segments.last() else {
            return;
        };
        let suggestion = format!("std::str::{}", func.ident.as_str());

        span_lint_and_sugg(
            cx,
            LEGACY_STR_FROM_UTF8,
            expr.span,
            "usage of legacy `core::str::from_utf8` function",
            "try instead",
            suggestion,
            Applicability::MachineApplicable,
        );
    }
}

fn is_from_utf8(symbol: Symbol) -> bool {
    let from_utf8 = Symbol::intern("from_utf8");
    let from_utf8_mut = Symbol::intern("from_utf8_mut");
    let from_utf8_unchecked = Symbol::intern("from_utf8_unchecked");
    let from_utf8_unchecked_mut = Symbol::intern("from_utf8_unchecked_mut");

    symbol == from_utf8 || symbol == from_utf8_mut || symbol == from_utf8_unchecked || symbol == from_utf8_unchecked_mut
}

fn should_lint_on_path(segments: &[PathSegment<'_>]) -> bool {
    // And that crate is `core`
    let Some(module) = segments.first() else {
        return false;
    };
    if module.ident.name != sym::core {
        return false;
    }

    // And that function is `from_utf8`
    let Some(func) = segments.last() else {
        return false;
    };
    if !is_from_utf8(func.ident.name) {
        return false;
    }

    true
}
