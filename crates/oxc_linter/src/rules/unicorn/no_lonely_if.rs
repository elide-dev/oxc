use oxc_ast::AstKind;
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_span::Span;

use crate::{AstNode, context::LintContext, rule::Rule};

fn no_lonely_if_diagnostic(if_stmt_span: Span, parent_if_stmt_span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Unexpected `if` as the only statement in a `if` block without `else`.")
        .with_help("Move the inner `if` test to the outer `if` test.")
        .with_labels([if_stmt_span, parent_if_stmt_span])
}

#[derive(Debug, Default, Clone)]
pub struct NoLonelyIf;

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallow `if` statements as the only statement in `if` blocks without `else`.
    ///
    /// ### Why is this bad?
    ///
    /// It can be confusing to have an `if` statement without an `else` clause as the only statement in an `if` block.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```javascript
    /// if (foo) {
    ///     if (bar) {
    ///     }
    /// }
    /// if (foo) if (bar) baz();
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```javascript
    /// if (foo && bar) {
    /// }
    /// if (foo && bar) baz();
    /// ```
    NoLonelyIf,
    unicorn,
    pedantic
);

impl Rule for NoLonelyIf {
    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        let AstKind::IfStatement(if_stmt) = node.kind() else {
            return;
        };

        if if_stmt.alternate.is_some() {
            return;
        }

        let parent = ctx.nodes().parent_node(node.id());

        let parent_if_stmt_span = match parent.kind() {
            AstKind::BlockStatement(block_stmt) => {
                if block_stmt.body.len() != 1 {
                    return;
                }

                let AstKind::IfStatement(parent_if_stmt) = ctx.nodes().parent_kind(parent.id())
                else {
                    return;
                };

                if parent_if_stmt.alternate.is_some() {
                    return;
                }
                parent_if_stmt.span
            }
            AstKind::IfStatement(parent_if_stmt) => {
                if parent_if_stmt.alternate.is_some() {
                    return;
                }

                parent_if_stmt.span
            }
            _ => return,
        };

        ctx.diagnostic(no_lonely_if_diagnostic(
            Span::sized(if_stmt.span.start, 2),
            Span::sized(parent_if_stmt_span.start, 2),
        ));
    }
}

#[test]
fn test() {
    use crate::tester::Tester;

    let pass = vec![
        r"if (a) { if (b) { } } else { }",
        r"if (a) { if (b) { } foo(); } else {}",
        r"if (a) { } else { if (y) { } }",
        r"if (a) { b ? c() : d()}",
    ];

    let fail = vec![
        r"
        if (a) {
            if (b) {
            }
        }
    ",
        // Inner one is `BlockStatement`
        r"
        if (a) if (b) {
            foo();
        }
    ",
        // Outer one is `BlockStatement`
        r"
        if (a) {
            if (b) foo();
        }
    ",
        // No `BlockStatement`
        r"if (a) if (b) foo();",
        r"
        if (a) {
            if (b) foo()
        }
    ",
        // `EmptyStatement`
        r"if (a) if (b);",
        // Nested
        r"
        if (a) {
            if (b) {
                // Should not report
            }
        } else if (c) {
            if (d) {
            }
        }
    ",
        // Need parenthesis
        r"
        function * foo() {
            if (a || b)
            if (a ?? b)
            if (a ? b : c)
            if (a = b)
            if (a += b)
            if (a -= b)
            if (a &&= b)
            if (yield a)
            if (a, b);
        }
    ",
        // Should not add parenthesis
        r"
        async function foo() {
            if (a)
            if (await a)
            if (a.b)
            if (a && b);
        }
    ",
        // Don't case parenthesis in outer test
        // 'if (((a || b))) if (((c || d)));',
        // Comments
        r"
        if // 1
        (
            // 2
            a // 3
                .b // 4
        ) // 5
        {
            // 6
            if (
                // 7
                c // 8
                    .d // 9
            ) {
                // 10
                foo();
                // 11
            }
            // 12
        }
    ",
        // Semicolon
        r"
        if (a) {
            if (b) foo()
        }
        [].forEach(bar)
    ",
        r"
        if (a)
            if (b) foo()
        ;[].forEach(bar)
    ",
        r"
        if (a) {
            if (b) foo()
        }
        ;[].forEach(bar)
    ",
    ];

    Tester::new(NoLonelyIf::NAME, NoLonelyIf::PLUGIN, pass, fail).test_and_snapshot();
}
