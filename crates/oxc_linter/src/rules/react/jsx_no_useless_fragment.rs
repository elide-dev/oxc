use oxc_ast::{
    AstKind,
    ast::{
        JSXAttributeItem, JSXAttributeName, JSXChild, JSXElement, JSXElementName, JSXExpression,
        JSXFragment, JSXMemberExpressionObject, JSXOpeningElement,
    },
};
use oxc_diagnostics::OxcDiagnostic;
use oxc_macros::declare_oxc_lint;
use oxc_semantic::NodeId;
use oxc_span::Span;

use crate::{
    AstNode,
    context::{ContextHost, LintContext},
    rule::Rule,
};

fn needs_more_children(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Fragments should contain more than one child.").with_label(span)
}

fn child_of_html_element(span: Span) -> OxcDiagnostic {
    OxcDiagnostic::warn("Passing a fragment to a HTML element is useless.").with_label(span)
}

#[derive(Debug, Default, Clone)]
pub struct JsxNoUselessFragment {
    /// Allow fragments with a single expression child.
    pub allow_expressions: bool,
}

declare_oxc_lint!(
    /// ### What it does
    ///
    /// Disallow unnecessary fragments.
    ///
    /// ### Why is this bad?
    ///
    /// Fragments are a useful tool when you need to group multiple children without adding a node to the DOM tree. However, sometimes you might end up with a fragment with a single child. When this child is an element, string, or expression, it's not necessary to use a fragment.
    ///
    /// ### Examples
    ///
    /// Examples of **incorrect** code for this rule:
    /// ```jsx
    /// <>foo</>
    /// <div><>foo</></div>
    /// ```
    ///
    /// Examples of **correct** code for this rule:
    /// ```jsx
    /// <>foo <div></div></>
    /// <div>foo</div>
    /// ```
    JsxNoUselessFragment,
    react,
    pedantic
);

impl Rule for JsxNoUselessFragment {
    fn from_configuration(value: serde_json::Value) -> Self {
        let value = value.as_array().and_then(|arr| arr.first()).and_then(|val| val.as_object());

        Self {
            allow_expressions: value
                .and_then(|val| val.get("allowExpressions").and_then(serde_json::Value::as_bool))
                .unwrap_or(Self::default().allow_expressions),
        }
    }

    fn run<'a>(&self, node: &AstNode<'a>, ctx: &LintContext<'a>) {
        match node.kind() {
            AstKind::JSXElement(jsx_elem) => {
                if !is_jsx_fragment(&jsx_elem.opening_element) {
                    return;
                }
                self.check_element(node, jsx_elem, ctx);
            }
            AstKind::JSXFragment(jsx_elem) => {
                self.check_fragment(node, jsx_elem, ctx);
            }
            _ => {}
        }
    }

    fn should_run(&self, ctx: &ContextHost) -> bool {
        ctx.source_type().is_jsx()
    }
}

impl JsxNoUselessFragment {
    fn check_element(&self, node: &AstNode, elem: &JSXElement, ctx: &LintContext) {
        if jsx_elem_has_key_attr(elem) {
            return;
        }

        if has_less_than_two_children(&elem.children)
            && !is_fragment_with_only_text_and_is_not_child(node.id(), &elem.children, ctx)
            && !(self.allow_expressions && is_fragment_with_single_expression(&elem.children))
        {
            let span = elem.opening_element.span;
            ctx.diagnostic(needs_more_children(span));
        }

        if is_child_of_html_element(node, ctx) {
            let span = elem.opening_element.span;
            ctx.diagnostic(child_of_html_element(span));
        }
    }

    fn check_fragment(&self, node: &AstNode, elem: &JSXFragment, ctx: &LintContext) {
        if has_less_than_two_children(&elem.children)
            && !is_fragment_with_only_text_and_is_not_child(node.id(), &elem.children, ctx)
            && !(self.allow_expressions && is_fragment_with_single_expression(&elem.children))
        {
            let span = elem.opening_fragment.span;
            ctx.diagnostic(needs_more_children(span));
        }

        if is_child_of_html_element(node, ctx) {
            let span = elem.opening_fragment.span;
            ctx.diagnostic(child_of_html_element(span));
        }
    }
}

fn jsx_elem_has_key_attr(elem: &JSXElement) -> bool {
    elem.opening_element.attributes.iter().any(|attr| {
        let JSXAttributeItem::Attribute(attr) = attr else {
            return false;
        };

        let JSXAttributeName::Identifier(ident) = &attr.name else {
            return false;
        };

        ident.name == "key"
    })
}

fn is_fragment_with_single_expression(children: &oxc_allocator::Vec<'_, JSXChild<'_>>) -> bool {
    let children = children.iter().filter(|v| is_padding_spaces(v)).collect::<Vec<_>>();

    children.len() == 1 && matches!(children[0], JSXChild::ExpressionContainer(_))
}

fn is_padding_spaces(v: &JSXChild<'_>) -> bool {
    if let JSXChild::Text(v) = v {
        return !(v.value.trim().is_empty() && v.value.contains('\n'));
    }

    true
}

fn is_child_of_html_element(node: &AstNode, ctx: &LintContext) -> bool {
    if let Some(AstKind::JSXElement(elem)) = ctx.nodes().parent_kind(node.id()) {
        if is_html_element(&elem.opening_element.name) {
            return true;
        }
    }

    false
}

fn is_html_element(elem_name: &JSXElementName) -> bool {
    let JSXElementName::Identifier(ident) = elem_name else {
        return false;
    };

    ident.name.starts_with(char::is_lowercase)
}

fn is_jsx_fragment(elem: &JSXOpeningElement) -> bool {
    match &elem.name {
        JSXElementName::IdentifierReference(ident) => ident.name == "Fragment",
        JSXElementName::MemberExpression(mem_expr) => {
            if let JSXMemberExpressionObject::IdentifierReference(ident) = &mem_expr.object {
                ident.name == "React" && mem_expr.property.name == "Fragment"
            } else {
                false
            }
        }
        JSXElementName::NamespacedName(_)
        | JSXElementName::Identifier(_)
        | JSXElementName::ThisExpression(_) => false,
    }
}

fn has_less_than_two_children(children: &oxc_allocator::Vec<'_, JSXChild<'_>>) -> bool {
    let non_padding_children = children.iter().filter(|v| is_padding_spaces(v)).collect::<Vec<_>>();

    if non_padding_children.len() < 2 {
        return !non_padding_children.iter().any(|v| {
            if let JSXChild::ExpressionContainer(v) = v {
                if let JSXExpression::CallExpression(_) = v.expression {
                    return true;
                }
                return false;
            }

            false
        });
    }
    false
}

fn is_fragment_with_only_text_and_is_not_child<'a>(
    id: NodeId,
    node: &oxc_allocator::Vec<'a, JSXChild<'a>>,
    ctx: &LintContext,
) -> bool {
    if node.len() != 1 {
        return false;
    }

    if let Some(JSXChild::Text(_)) = node.first() {
        let Some(parent) = ctx.nodes().parent_kind(id) else {
            return false;
        };
        return !matches!(parent, AstKind::JSXElement(_) | AstKind::JSXFragment(_));
    }

    false
}

#[test]
fn test() {
    use serde_json::json;

    use crate::tester::Tester;

    let pass = vec![
        (r"<><Foo /><Bar /></>", None),
        (r"<>foo<div /></>", None),
        (r"<> <div /></>", None),
        (r#"<>{"moo"} </>"#, None),
        (r"<NotFragment />", None),
        (r"<React.NotFragment />", None),
        (r"<NotReact.Fragment />", None),
        (r"<Foo><><div /><div /></></Foo>", None),
        (r#"<div p={<>{"a"}{"b"}</>} />"#, None),
        (r"<Fragment key={item.id}>{item.value}</Fragment>", None),
        (r"<Fooo content={<>eeee ee eeeeeee eeeeeeee</>} />", None),
        (r"<>{foos.map(foo => foo)}</>", None),
        (r"<>{moo}</>", Some(json!([{ "allowExpressions": true }]))),
        (
            r"
        <>
            {moo}
        </>
        ",
            Some(json!([{ "allowExpressions": true }])),
        ),
        (r"{1 && <>{1}</>}", Some(json!([{"allowExpressions": true}]))),
    ];

    let fail = vec![
        (r"<></>", None),
        (r"<>{}</>", None),
        (r"<p>moo<>foo</></p>", None),
        (r"<>{meow}</>", None),
        (r"<p><>{meow}</></p>", None),
        (r"<><div/></>", None),
        (
            r"
            <>
              <div/>
            </>
        ",
            None,
        ),
        (r"<Fragment />", None),
        (
            r"
                <React.Fragment>
                  <Foo />
                </React.Fragment>
            ",
            None,
        ),
        (r"<Eeee><>foo</></Eeee>", None),
        (r"<div><>foo</></div>", None),
        (r#"<div><>{"a"}{"b"}</></div>"#, None),
        (r#"<div><>{"a"}{"b"}</></div>"#, None),
        (
            r#"
            <section>
              <Eeee />
              <Eeee />
              <>{"a"}{"b"}</>
            </section>"#,
            None,
        ),
        (r#"<div><Fragment>{"a"}{"b"}</Fragment></div>"#, None),
        (
            r"
            <section>
              git<>
                <b>hub</b>.
              </>

              git<> <b>hub</b></>
            </section>
            ",
            None,
        ),
        (r#"<div>a <>{""}{""}</> a</div>"#, None),
        (
            r"
            const Comp = () => (
              <html>
                <React.Fragment />
              </html>
            );
        ",
            None,
        ),
        (r"<><Foo>{moo}</Foo></>", None),
    ];

    Tester::new(JsxNoUselessFragment::NAME, JsxNoUselessFragment::PLUGIN, pass, fail)
        .test_and_snapshot();
}
