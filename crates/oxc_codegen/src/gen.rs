use std::ops::Not;

use cow_utils::CowUtils;

use oxc_ast::ast::*;
use oxc_span::GetSpan;
use oxc_syntax::{
    operator::UnaryOperator,
    precedence::{GetPrecedence, Precedence},
};

use crate::{
    Codegen, Context, Operator, Quote,
    binary_expr_visitor::{BinaryExpressionVisitor, Binaryish, BinaryishOperator},
};

const PURE_COMMENT: &str = "/* @__PURE__ */ ";
const NO_SIDE_EFFECTS_NEW_LINE_COMMENT: &str = "/* @__NO_SIDE_EFFECTS__ */\n";
const NO_SIDE_EFFECTS_COMMENT: &str = "/* @__NO_SIDE_EFFECTS__ */ ";

/// Generate source code for an AST node.
pub trait Gen: GetSpan {
    /// Generate code for an AST node.
    fn r#gen(&self, p: &mut Codegen, ctx: Context);

    /// Generate code for an AST node. Alias for `gen`.
    #[inline]
    fn print(&self, p: &mut Codegen, ctx: Context) {
        self.r#gen(p, ctx);
    }
}

/// Generate source code for an expression.
pub trait GenExpr: GetSpan {
    /// Generate code for an expression, respecting operator precedence.
    fn gen_expr(&self, p: &mut Codegen, precedence: Precedence, ctx: Context);

    /// Generate code for an expression, respecting operator precedence. Alias for `gen_expr`.
    #[inline]
    fn print_expr(&self, p: &mut Codegen, precedence: Precedence, ctx: Context) {
        self.gen_expr(p, precedence, ctx);
    }
}

impl Gen for Program<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.is_jsx = self.source_type.is_jsx();

        if let Some(hashbang) = &self.hashbang {
            hashbang.print(p, ctx);
        }
        p.print_directives_and_statements(&self.directives, &self.body, ctx);
        p.print_semicolon_if_needed();
        // Print trailing statement comments.
        p.print_comments_at(self.span.end);
    }
}

impl Gen for Hashbang<'_> {
    fn r#gen(&self, p: &mut Codegen, _ctx: Context) {
        p.print_str("#!");
        p.print_str(self.value.as_str());
        p.print_hard_newline();
    }
}

impl Gen for Directive<'_> {
    fn r#gen(&self, p: &mut Codegen, _ctx: Context) {
        p.add_source_mapping(self.span);
        p.print_indent();
        // A Use Strict Directive may not contain an EscapeSequence or LineContinuation.
        // So here should print original `directive` value, the `expression` value is escaped str.
        // See https://github.com/babel/babel/blob/v7.26.2/packages/babel-generator/src/generators/base.ts#L64
        let directive = self.directive.as_str();

        let mut bytes = directive.as_bytes().iter();
        let mut quote = p.quote;
        while let Some(&b) = bytes.next() {
            match b {
                b'"' => {
                    quote = Quote::Single;
                    break;
                }
                b'\'' => {
                    quote = Quote::Double;
                    break;
                }
                b'\\' => {
                    bytes.next();
                }
                _ => {}
            }
        }
        quote.print(p);
        p.print_str(directive);
        quote.print(p);
        p.print_ascii_byte(b';');
        p.print_soft_newline();
    }
}

impl Gen for Statement<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        match self {
            Self::BlockStatement(stmt) => {
                p.print_comments_at(stmt.span.start);
                stmt.print(p, ctx);
            }
            Self::BreakStatement(stmt) => stmt.print(p, ctx),
            Self::ContinueStatement(stmt) => stmt.print(p, ctx),
            Self::DebuggerStatement(stmt) => stmt.print(p, ctx),
            Self::DoWhileStatement(stmt) => stmt.print(p, ctx),
            Self::EmptyStatement(stmt) => stmt.print(p, ctx),
            Self::ExpressionStatement(stmt) => stmt.print(p, ctx),
            Self::ForInStatement(stmt) => stmt.print(p, ctx),
            Self::ForOfStatement(stmt) => stmt.print(p, ctx),
            Self::ForStatement(stmt) => stmt.print(p, ctx),
            Self::IfStatement(stmt) => stmt.print(p, ctx),
            Self::LabeledStatement(stmt) => stmt.print(p, ctx),
            Self::ReturnStatement(stmt) => stmt.print(p, ctx),
            Self::SwitchStatement(stmt) => stmt.print(p, ctx),
            Self::ThrowStatement(stmt) => stmt.print(p, ctx),
            Self::TryStatement(stmt) => stmt.print(p, ctx),
            Self::WhileStatement(stmt) => stmt.print(p, ctx),
            Self::WithStatement(stmt) => stmt.print(p, ctx),
            Self::ImportDeclaration(decl) => decl.print(p, ctx),
            Self::ExportAllDeclaration(decl) => decl.print(p, ctx),
            Self::ExportDefaultDeclaration(decl) => decl.print(p, ctx),
            Self::ExportNamedDeclaration(decl) => decl.print(p, ctx),
            Self::TSExportAssignment(decl) => decl.print(p, ctx),
            Self::TSNamespaceExportDeclaration(decl) => decl.print(p, ctx),
            Self::VariableDeclaration(decl) => {
                p.print_comments_at(decl.span.start);
                p.print_indent();
                decl.print(p, ctx);
                p.print_semicolon_after_statement();
            }
            Self::FunctionDeclaration(decl) => {
                p.print_comments_at(decl.span.start);
                if decl.pure && p.options.print_annotation_comment() {
                    p.print_indent();
                    p.print_str(NO_SIDE_EFFECTS_NEW_LINE_COMMENT);
                }
                p.print_indent();
                decl.print(p, ctx);
                p.print_soft_newline();
            }
            Self::ClassDeclaration(decl) => {
                p.print_comments_at(decl.span.start);
                p.print_indent();
                decl.print(p, ctx);
                p.print_soft_newline();
            }
            Self::TSModuleDeclaration(decl) => {
                p.print_comments_at(decl.span.start);
                p.print_indent();
                decl.print(p, ctx);
                p.print_soft_newline();
            }
            Self::TSTypeAliasDeclaration(decl) => {
                p.print_indent();
                p.print_comments_at(decl.span.start);
                decl.print(p, ctx);
                p.print_semicolon_after_statement();
            }
            Self::TSInterfaceDeclaration(decl) => {
                p.print_indent();
                p.print_comments_at(decl.span.start);
                decl.print(p, ctx);
                p.print_soft_newline();
            }
            Self::TSEnumDeclaration(decl) => {
                p.print_indent();
                p.print_comments_at(decl.span.start);
                decl.print(p, ctx);
                p.print_soft_newline();
            }
            Self::TSImportEqualsDeclaration(decl) => {
                p.print_indent();
                p.print_comments_at(decl.span.start);
                decl.print(p, ctx);
                p.print_semicolon_after_statement();
            }
        }
    }
}

impl Gen for ExpressionStatement<'_> {
    fn r#gen(&self, p: &mut Codegen, _ctx: Context) {
        p.print_comments_at(self.span.start);
        p.print_indent();
        p.start_of_stmt = p.code_len();
        p.print_expression(&self.expression);
        if self.expression.is_specific_id("let") {
            p.print_semicolon();
        } else {
            p.print_semicolon_after_statement();
        }
    }
}

impl Gen for IfStatement<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.print_comments_at(self.span.start);
        p.add_source_mapping(self.span);
        p.print_indent();
        print_if(self, p, ctx);
    }
}

fn print_if(if_stmt: &IfStatement<'_>, p: &mut Codegen, ctx: Context) {
    p.print_space_before_identifier();
    p.print_str("if");
    p.print_soft_space();
    p.print_ascii_byte(b'(');
    p.print_expression(&if_stmt.test);
    p.print_ascii_byte(b')');

    match &if_stmt.consequent {
        Statement::BlockStatement(block) => {
            p.print_soft_space();
            p.print_block_statement(block, ctx);
            if if_stmt.alternate.is_some() {
                p.print_soft_space();
            } else {
                p.print_soft_newline();
            }
        }
        stmt if wrap_to_avoid_ambiguous_else(stmt) => {
            p.print_soft_space();
            p.print_block_start(stmt.span());
            stmt.print(p, ctx);
            p.needs_semicolon = false;
            p.print_block_end(stmt.span());
            if if_stmt.alternate.is_some() {
                p.print_soft_space();
            } else {
                p.print_soft_newline();
            }
        }
        stmt => {
            p.print_body(stmt, false, ctx);
            if if_stmt.alternate.is_some() {
                p.print_indent();
            }
        }
    }
    if let Some(alternate) = if_stmt.alternate.as_ref() {
        p.print_semicolon_if_needed();
        p.print_space_before_identifier();
        p.print_str("else");
        match alternate {
            Statement::BlockStatement(block) => {
                p.print_soft_space();
                p.print_block_statement(block, ctx);
                p.print_soft_newline();
            }
            Statement::IfStatement(if_stmt) => {
                p.print_hard_space();
                print_if(if_stmt, p, ctx);
            }
            stmt => p.print_body(stmt, true, ctx),
        }
    }
}

// <https://github.com/evanw/esbuild/blob/e6a8169c3a574f4c67d4cdd5f31a938b53eb7421/internal/js_printer/js_printer.go#L3444>
fn wrap_to_avoid_ambiguous_else(stmt: &Statement) -> bool {
    let mut current = stmt;
    loop {
        current = match current {
            Statement::IfStatement(if_stmt) => {
                if let Some(stmt) = &if_stmt.alternate {
                    stmt
                } else {
                    return true;
                }
            }
            Statement::ForStatement(for_stmt) => &for_stmt.body,
            Statement::ForOfStatement(for_of_stmt) => &for_of_stmt.body,
            Statement::ForInStatement(for_in_stmt) => &for_in_stmt.body,
            Statement::WhileStatement(while_stmt) => &while_stmt.body,
            Statement::WithStatement(with_stmt) => &with_stmt.body,
            Statement::LabeledStatement(labeled_stmt) => &labeled_stmt.body,
            _ => return false,
        }
    }
}

impl Gen for BlockStatement<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.print_indent();
        p.print_block_statement(self, ctx);
        p.print_soft_newline();
    }
}

impl Gen for ForStatement<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.print_comments_at(self.span.start);
        p.add_source_mapping(self.span);
        p.print_indent();
        p.print_space_before_identifier();
        p.print_str("for");
        p.print_soft_space();
        p.print_ascii_byte(b'(');

        if let Some(init) = &self.init {
            init.print(p, Context::FORBID_IN);
        }

        p.print_semicolon();

        if let Some(test) = self.test.as_ref() {
            p.print_soft_space();
            p.print_expression(test);
        }

        p.print_semicolon();

        if let Some(update) = self.update.as_ref() {
            p.print_soft_space();
            p.print_expression(update);
        }

        p.print_ascii_byte(b')');
        p.print_body(&self.body, false, ctx);
    }
}

impl Gen for ForInStatement<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.print_comments_at(self.span.start);
        p.add_source_mapping(self.span);
        p.print_indent();
        p.print_space_before_identifier();
        p.print_str("for");
        p.print_soft_space();
        p.print_ascii_byte(b'(');
        self.left.print(p, Context::FORBID_IN);
        p.print_soft_space();
        p.print_space_before_identifier();
        p.print_str("in");
        p.print_soft_space();
        p.print_expression(&self.right);
        p.print_ascii_byte(b')');
        p.print_body(&self.body, false, ctx);
    }
}

impl Gen for ForOfStatement<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.print_comments_at(self.span.start);
        p.add_source_mapping(self.span);
        p.print_indent();
        p.print_space_before_identifier();
        p.print_str("for");
        if self.r#await {
            p.print_str(" await");
        }
        p.print_soft_space();
        p.print_ascii_byte(b'(');
        self.left.print(p, ctx);
        p.print_soft_space();
        p.print_space_before_identifier();
        p.print_str("of");
        p.print_soft_space();
        self.right.print_expr(p, Precedence::Comma, Context::empty());
        p.print_ascii_byte(b')');
        p.print_body(&self.body, false, ctx);
    }
}

impl Gen for ForStatementInit<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        match self {
            Self::VariableDeclaration(var) => var.print(p, ctx),
            _ => self.to_expression().print_expr(p, Precedence::Lowest, ctx),
        }
    }
}

impl Gen for ForStatementLeft<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        match self {
            ForStatementLeft::VariableDeclaration(var) => var.print(p, ctx),
            ForStatementLeft::AssignmentTargetIdentifier(identifier) => {
                let wrap = identifier.name == "async";
                p.wrap(wrap, |p| self.to_assignment_target().print(p, ctx));
            }
            match_assignment_target!(ForStatementLeft) => {
                p.wrap(false, |p| self.to_assignment_target().print(p, ctx));
            }
        }
    }
}

impl Gen for WhileStatement<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.print_comments_at(self.span.start);
        p.add_source_mapping(self.span);
        p.print_indent();
        p.print_space_before_identifier();
        p.print_str("while");
        p.print_soft_space();
        p.print_ascii_byte(b'(');
        p.print_expression(&self.test);
        p.print_ascii_byte(b')');
        p.print_body(&self.body, false, ctx);
    }
}

impl Gen for DoWhileStatement<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.print_comments_at(self.span.start);
        p.add_source_mapping(self.span);
        p.print_indent();
        p.print_space_before_identifier();
        p.print_str("do");
        match &self.body {
            Statement::BlockStatement(block) => {
                p.print_soft_space();
                p.print_block_statement(block, ctx);
                p.print_soft_space();
            }
            Statement::EmptyStatement(s) => s.print(p, ctx),
            _ => {
                p.print_soft_newline();
                p.indent();
                self.body.print(p, ctx);
                p.print_semicolon_if_needed();
                p.dedent();
                p.print_indent();
            }
        }
        p.print_str("while");
        p.print_soft_space();
        p.print_ascii_byte(b'(');
        p.print_expression(&self.test);
        p.print_ascii_byte(b')');
        p.print_semicolon_after_statement();
    }
}

impl Gen for EmptyStatement {
    fn r#gen(&self, p: &mut Codegen, _ctx: Context) {
        p.print_comments_at(self.span.start);
        p.add_source_mapping(self.span);
        p.print_indent();
        p.print_semicolon();
        p.print_soft_newline();
    }
}

impl Gen for ContinueStatement<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.print_comments_at(self.span.start);
        p.add_source_mapping(self.span);
        p.print_indent();
        p.print_space_before_identifier();
        p.print_str("continue");
        if let Some(label) = &self.label {
            p.print_soft_space();
            label.print(p, ctx);
        }
        p.print_semicolon_after_statement();
    }
}

impl Gen for BreakStatement<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.print_comments_at(self.span.start);
        p.add_source_mapping(self.span);
        p.print_indent();
        p.print_space_before_identifier();
        p.print_str("break");
        if let Some(label) = &self.label {
            p.print_soft_space();
            label.print(p, ctx);
        }
        p.print_semicolon_after_statement();
    }
}

impl Gen for SwitchStatement<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.print_comments_at(self.span.start);
        p.add_source_mapping(self.span);
        p.print_indent();
        p.print_space_before_identifier();
        p.print_str("switch");
        p.print_soft_space();
        p.print_ascii_byte(b'(');
        p.print_expression(&self.discriminant);
        p.print_ascii_byte(b')');
        p.print_soft_space();
        p.print_curly_braces(self.span, self.cases.is_empty(), |p| {
            for case in &self.cases {
                case.print(p, ctx);
            }
        });
        p.print_soft_newline();
        p.needs_semicolon = false;
    }
}

impl Gen for SwitchCase<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.print_semicolon_if_needed();
        p.print_indent();
        p.add_source_mapping(self.span);
        match &self.test {
            Some(test) => {
                p.print_str("case");
                p.print_soft_space();
                p.print_expression(test);
            }
            None => p.print_str("default"),
        }
        p.print_colon();

        if self.consequent.len() == 1 {
            p.print_body(&self.consequent[0], false, ctx);
            return;
        }

        p.print_soft_newline();
        p.indent();
        for item in &self.consequent {
            p.print_semicolon_if_needed();
            item.print(p, ctx);
        }
        p.dedent();
    }
}

impl Gen for ReturnStatement<'_> {
    fn r#gen(&self, p: &mut Codegen, _ctx: Context) {
        p.print_comments_at(self.span.start);
        p.add_source_mapping(self.span);
        p.print_indent();
        p.print_space_before_identifier();
        p.print_str("return");
        if let Some(arg) = &self.argument {
            p.print_soft_space();
            p.print_expression(arg);
        }
        p.print_semicolon_after_statement();
    }
}

impl Gen for LabeledStatement<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.print_comments_at(self.span.start);
        if !p.options.minify && (p.indent > 0 || p.print_next_indent_as_space) {
            p.add_source_mapping(self.span);
            p.print_indent();
        }
        p.print_space_before_identifier();
        self.label.print(p, ctx);
        p.print_colon();
        p.print_body(&self.body, false, ctx);
    }
}

impl Gen for TryStatement<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.print_comments_at(self.span.start);
        p.add_source_mapping(self.span);
        p.print_indent();
        p.print_space_before_identifier();
        p.print_str("try");
        p.print_soft_space();
        p.print_block_statement(&self.block, ctx);
        if let Some(handler) = &self.handler {
            handler.r#gen(p, ctx);
        }
        if let Some(finalizer) = &self.finalizer {
            p.print_soft_space();
            p.print_str("finally");
            p.print_soft_space();
            p.print_block_statement(finalizer, ctx);
        }
        p.print_soft_newline();
    }
}

impl Gen for CatchClause<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.print_soft_space();
        p.print_comments_at(self.span.start);
        p.print_str("catch");
        if let Some(param) = &self.param {
            p.print_soft_space();
            p.print_str("(");
            param.pattern.print(p, ctx);
            p.print_str(")");
        }
        p.print_soft_space();
        p.print_block_statement(&self.body, ctx);
    }
}

impl Gen for ThrowStatement<'_> {
    fn r#gen(&self, p: &mut Codegen, _ctx: Context) {
        p.print_comments_at(self.span.start);
        p.add_source_mapping(self.span);
        p.print_indent();
        p.print_space_before_identifier();
        p.print_str("throw");
        p.print_soft_space();
        p.print_expression(&self.argument);
        p.print_semicolon_after_statement();
    }
}

impl Gen for WithStatement<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.print_comments_at(self.span.start);
        p.add_source_mapping(self.span);
        p.print_indent();
        p.print_space_before_identifier();
        p.print_str("with");
        p.print_ascii_byte(b'(');
        p.print_expression(&self.object);
        p.print_ascii_byte(b')');
        p.print_body(&self.body, false, ctx);
    }
}

impl Gen for DebuggerStatement {
    fn r#gen(&self, p: &mut Codegen, _ctx: Context) {
        p.print_comments_at(self.span.start);
        p.add_source_mapping(self.span);
        p.print_indent();
        p.print_space_before_identifier();
        p.print_str("debugger");
        p.print_semicolon_after_statement();
    }
}

impl Gen for VariableDeclaration<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.add_source_mapping(self.span);
        p.print_space_before_identifier();
        if self.declare {
            p.print_str("declare ");
        }

        p.print_str(match self.kind {
            VariableDeclarationKind::Const => "const",
            VariableDeclarationKind::Let => "let",
            VariableDeclarationKind::Var => "var",
            VariableDeclarationKind::Using => "using",
            VariableDeclarationKind::AwaitUsing => "await using",
        });
        if !self.declarations.is_empty() {
            p.print_soft_space();
        }
        p.print_list(&self.declarations, ctx);
    }
}

impl Gen for VariableDeclarator<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        self.id.kind.print(p, ctx);
        if self.definite {
            p.print_ascii_byte(b'!');
        }
        if self.id.optional {
            p.print_str("?");
        }
        if let Some(type_annotation) = &self.id.type_annotation {
            p.print_colon();
            p.print_soft_space();
            type_annotation.print(p, ctx);
        }
        if let Some(init) = &self.init {
            p.print_soft_space();
            p.print_equal();
            p.print_soft_space();
            init.print_expr(p, Precedence::Comma, ctx);
        }
    }
}

impl Gen for Function<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        let n = p.code_len();
        let wrap = self.is_expression() && (p.start_of_stmt == n || p.start_of_default_export == n);
        p.wrap(wrap, |p| {
            p.print_space_before_identifier();
            p.add_source_mapping(self.span);
            if self.declare {
                p.print_str("declare ");
            }
            if self.r#async {
                p.print_str("async ");
            }
            p.print_str("function");
            if self.generator {
                p.print_ascii_byte(b'*');
                p.print_soft_space();
            }
            if let Some(id) = &self.id {
                p.print_space_before_identifier();
                id.print(p, ctx);
            }
            if let Some(type_parameters) = &self.type_parameters {
                type_parameters.print(p, ctx);
            }
            p.print_ascii_byte(b'(');
            if let Some(this_param) = &self.this_param {
                this_param.print(p, ctx);
                if !self.params.is_empty() || self.params.rest.is_some() {
                    p.print_str(",");
                    p.print_soft_space();
                }
            }
            self.params.print(p, ctx);
            p.print_ascii_byte(b')');
            if let Some(return_type) = &self.return_type {
                p.print_str(": ");
                return_type.print(p, ctx);
            }
            if let Some(body) = &self.body {
                p.print_soft_space();
                body.print(p, ctx);
            } else {
                p.print_semicolon();
            }
        });
    }
}

impl Gen for FunctionBody<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        let span_end = self.span.end;
        let comments_at_end = if span_end > 0 { p.get_comments(span_end - 1) } else { None };
        let single_line = if self.is_empty() {
            comments_at_end.as_ref().is_none_or(|comments| {
                comments.iter().all(|c| !c.preceded_by_newline() && !c.followed_by_newline())
            })
        } else {
            false
        };
        p.print_curly_braces(self.span, single_line, |p| {
            p.print_directives_and_statements(&self.directives, &self.statements, ctx);
            // Print trailing statement comments.
            if let Some(comments) = comments_at_end {
                p.print_comments(&comments);
                p.print_next_indent_as_space = false;
            }
        });
        p.needs_semicolon = false;
    }
}

impl Gen for FormalParameter<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.print_decorators(&self.decorators, ctx);
        if let Some(accessibility) = self.accessibility {
            p.print_space_before_identifier();
            p.print_str(accessibility.as_str());
            p.print_soft_space();
        }
        if self.r#override {
            p.print_space_before_identifier();
            p.print_str("override");
            p.print_soft_space();
        }
        if self.readonly {
            p.print_space_before_identifier();
            p.print_str("readonly");
            p.print_soft_space();
        }
        self.pattern.print(p, ctx);
    }
}

impl Gen for FormalParameters<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.print_list(&self.items, ctx);
        if let Some(rest) = &self.rest {
            if !self.items.is_empty() {
                p.print_comma();
                p.print_soft_space();
            }
            rest.print(p, ctx);
        }
    }
}

impl Gen for ImportDeclaration<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.print_comments_at(self.span.start);
        p.add_source_mapping(self.span);
        p.print_indent();
        p.print_space_before_identifier();
        p.print_str("import");
        if self.import_kind.is_type() {
            p.print_str(" type");
        }
        if let Some(phase) = self.phase {
            p.print_hard_space();
            p.print_str(phase.as_str());
        }
        if let Some(specifiers) = &self.specifiers {
            if specifiers.is_empty() {
                p.print_soft_space();
                p.print_str("{}");
                p.print_soft_space();
                p.print_str("from");
                p.print_soft_space();
                p.print_ascii_byte(b'"');
                p.print_str(self.source.value.as_str());
                p.print_ascii_byte(b'"');
                if let Some(with_clause) = &self.with_clause {
                    p.print_hard_space();
                    with_clause.print(p, ctx);
                }
                p.print_semicolon_after_statement();
                return;
            }

            let mut in_block = false;
            for (index, specifier) in specifiers.iter().enumerate() {
                match specifier {
                    ImportDeclarationSpecifier::ImportDefaultSpecifier(spec) => {
                        if in_block {
                            p.print_soft_space();
                            p.print_str("},");
                            in_block = false;
                        } else if index == 0 {
                            p.print_hard_space();
                        } else {
                            p.print_comma();
                            p.print_soft_space();
                        }
                        spec.local.print(p, ctx);
                        if index == specifiers.len() - 1 {
                            p.print_hard_space();
                        }
                    }
                    ImportDeclarationSpecifier::ImportNamespaceSpecifier(spec) => {
                        if in_block {
                            p.print_soft_space();
                            p.print_str("},");
                            in_block = false;
                        } else if index == 0 {
                            p.print_soft_space();
                        } else {
                            p.print_comma();
                            p.print_soft_space();
                        }
                        p.print_ascii_byte(b'*');
                        p.print_soft_space();
                        p.print_str("as ");
                        spec.local.print(p, ctx);
                        p.print_hard_space();
                    }
                    ImportDeclarationSpecifier::ImportSpecifier(spec) => {
                        if in_block {
                            p.print_comma();
                            p.print_soft_space();
                        } else {
                            if index != 0 {
                                p.print_comma();
                            }
                            in_block = true;
                            p.print_soft_space();
                            p.print_ascii_byte(b'{');
                            p.print_soft_space();
                        }

                        if spec.import_kind.is_type() {
                            p.print_str("type ");
                        }

                        spec.imported.print(p, ctx);
                        let local_name = p.get_binding_identifier_name(&spec.local);
                        let imported_name = get_module_export_name(&spec.imported, p);
                        if imported_name != local_name {
                            p.print_str(" as ");
                            spec.local.print(p, ctx);
                        }
                    }
                }
            }
            if in_block {
                p.print_soft_space();
                p.print_ascii_byte(b'}');
                p.print_soft_space();
            }
            p.print_str("from");
        }
        p.print_soft_space();
        p.print_string_literal(&self.source, false);
        if let Some(with_clause) = &self.with_clause {
            p.print_soft_space();
            with_clause.print(p, ctx);
        }
        p.add_source_mapping_end(self.span);
        p.print_semicolon_after_statement();
    }
}

impl Gen for WithClause<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.add_source_mapping(self.span);
        self.attributes_keyword.print(p, ctx);
        p.print_soft_space();
        p.add_source_mapping(self.span);
        p.print_ascii_byte(b'{');
        if !self.with_entries.is_empty() {
            p.print_soft_space();
            p.print_list(&self.with_entries, ctx);
            p.print_soft_space();
        }
        p.add_source_mapping_end(self.span);
        p.print_ascii_byte(b'}');
    }
}

impl Gen for ImportAttribute<'_> {
    fn r#gen(&self, p: &mut Codegen, _ctx: Context) {
        match &self.key {
            ImportAttributeKey::Identifier(identifier) => {
                p.print_str(identifier.name.as_str());
            }
            ImportAttributeKey::StringLiteral(literal) => {
                p.print_string_literal(literal, false);
            }
        }
        p.print_colon();
        p.print_soft_space();
        p.print_string_literal(&self.value, false);
    }
}

impl Gen for ExportNamedDeclaration<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.print_comments_at(self.span.start);
        if let Some(Declaration::FunctionDeclaration(func)) = &self.declaration {
            if func.pure && p.options.print_annotation_comment() {
                p.print_str(NO_SIDE_EFFECTS_NEW_LINE_COMMENT);
            }
        }
        p.add_source_mapping(self.span);
        p.print_indent();
        p.print_str("export");
        if let Some(decl) = &self.declaration {
            p.print_hard_space();
            match decl {
                Declaration::VariableDeclaration(decl) => decl.print(p, ctx),
                Declaration::FunctionDeclaration(decl) => decl.print(p, ctx),
                Declaration::ClassDeclaration(decl) => decl.print(p, ctx),
                Declaration::TSModuleDeclaration(decl) => decl.print(p, ctx),
                Declaration::TSTypeAliasDeclaration(decl) => decl.print(p, ctx),
                Declaration::TSInterfaceDeclaration(decl) => decl.print(p, ctx),
                Declaration::TSEnumDeclaration(decl) => decl.print(p, ctx),
                Declaration::TSImportEqualsDeclaration(decl) => decl.print(p, ctx),
            }
            if matches!(
                decl,
                Declaration::VariableDeclaration(_)
                    | Declaration::TSTypeAliasDeclaration(_)
                    | Declaration::TSImportEqualsDeclaration(_)
            ) {
                p.print_semicolon_after_statement();
            } else {
                p.print_soft_newline();
                p.needs_semicolon = false;
            }
        } else {
            if self.export_kind.is_type() {
                p.print_hard_space();
                p.print_str("type");
            }
            p.print_soft_space();
            p.print_ascii_byte(b'{');
            if !self.specifiers.is_empty() {
                p.print_soft_space();
                p.print_list(&self.specifiers, ctx);
                p.print_soft_space();
            }
            p.print_ascii_byte(b'}');
            if let Some(source) = &self.source {
                p.print_soft_space();
                p.print_str("from");
                p.print_soft_space();
                p.print_string_literal(source, false);
            }
            p.print_semicolon_after_statement();
        }
    }
}

impl Gen for TSExportAssignment<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.print_indent();
        p.print_comments_at(self.span.start);
        p.print_str("export = ");
        self.expression.print_expr(p, Precedence::Lowest, ctx);
        p.print_semicolon_after_statement();
    }
}

impl Gen for TSNamespaceExportDeclaration<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.print_indent();
        p.print_comments_at(self.span.start);
        p.print_str("export as namespace ");
        self.id.print(p, ctx);
        p.print_semicolon_after_statement();
    }
}

fn get_module_export_name<'a>(
    module_export_name: &ModuleExportName<'a>,
    p: &Codegen<'a>,
) -> &'a str {
    match module_export_name {
        ModuleExportName::IdentifierName(ident) => ident.name.as_str(),
        ModuleExportName::IdentifierReference(ident) => p.get_identifier_reference_name(ident),
        ModuleExportName::StringLiteral(s) => s.value.as_str(),
    }
}

impl Gen for ExportSpecifier<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        if self.export_kind.is_type() {
            p.print_str("type ");
        }
        self.local.print(p, ctx);
        let local_name = get_module_export_name(&self.local, p);
        let exported_name = get_module_export_name(&self.exported, p);
        if local_name != exported_name {
            p.print_str(" as ");
            self.exported.print(p, ctx);
        }
    }
}

impl Gen for ModuleExportName<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        match self {
            Self::IdentifierName(ident) => ident.print(p, ctx),
            Self::IdentifierReference(ident) => ident.print(p, ctx),
            Self::StringLiteral(literal) => p.print_string_literal(literal, false),
        }
    }
}

impl Gen for ExportAllDeclaration<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.print_comments_at(self.span.start);
        p.add_source_mapping(self.span);
        p.print_indent();
        p.print_str("export");
        if self.export_kind.is_type() {
            p.print_str(" type ");
        } else {
            p.print_soft_space();
        }
        p.print_ascii_byte(b'*');

        if let Some(exported) = &self.exported {
            p.print_soft_space();
            p.print_str("as ");
            exported.print(p, ctx);
            p.print_hard_space();
        } else {
            p.print_soft_space();
        }

        p.print_str("from");
        p.print_soft_space();
        p.print_string_literal(&self.source, false);
        if let Some(with_clause) = &self.with_clause {
            p.print_hard_space();
            with_clause.print(p, ctx);
        }
        p.print_semicolon_after_statement();
    }
}

impl Gen for ExportDefaultDeclaration<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.print_comments_at(self.span.start);
        if let ExportDefaultDeclarationKind::FunctionDeclaration(func) = &self.declaration {
            if func.pure && p.options.print_annotation_comment() {
                p.print_str(NO_SIDE_EFFECTS_NEW_LINE_COMMENT);
            }
        }
        p.add_source_mapping(self.span);
        p.print_indent();
        p.print_str("export default ");
        self.declaration.print(p, ctx);
    }
}
impl Gen for ExportDefaultDeclarationKind<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        match self {
            Self::FunctionDeclaration(func) => {
                func.print(p, ctx);
                p.print_soft_newline();
            }
            Self::ClassDeclaration(class) => {
                class.print(p, ctx);
                p.print_soft_newline();
            }
            Self::TSInterfaceDeclaration(interface) => interface.print(p, ctx),
            _ => {
                p.start_of_default_export = p.code_len();
                self.to_expression().print_expr(p, Precedence::Comma, Context::empty());
                p.print_semicolon_after_statement();
            }
        }
    }
}

impl GenExpr for Expression<'_> {
    fn gen_expr(&self, p: &mut Codegen, precedence: Precedence, ctx: Context) {
        match self {
            Self::BooleanLiteral(lit) => lit.print(p, ctx),
            Self::NullLiteral(lit) => lit.print(p, ctx),
            Self::NumericLiteral(lit) => lit.print_expr(p, precedence, ctx),
            Self::BigIntLiteral(lit) => lit.print_expr(p, precedence, ctx),
            Self::RegExpLiteral(lit) => lit.print(p, ctx),
            Self::StringLiteral(lit) => lit.print(p, ctx),
            Self::Identifier(ident) => ident.print(p, ctx),
            Self::ThisExpression(expr) => expr.print(p, ctx),
            Self::ComputedMemberExpression(expr) => expr.print_expr(p, precedence, ctx),
            Self::StaticMemberExpression(expr) => expr.print_expr(p, precedence, ctx),
            Self::PrivateFieldExpression(expr) => expr.print_expr(p, precedence, ctx),
            Self::CallExpression(expr) => expr.print_expr(p, precedence, ctx),
            Self::ArrayExpression(expr) => expr.print(p, ctx),
            Self::ObjectExpression(expr) => expr.print_expr(p, precedence, ctx),
            Self::FunctionExpression(func) => {
                if func.pure && p.options.print_annotation_comment() {
                    p.print_str(NO_SIDE_EFFECTS_COMMENT);
                }
                func.print(p, ctx);
            }
            Self::ArrowFunctionExpression(func) => {
                if func.pure && p.options.print_annotation_comment() {
                    p.print_str(NO_SIDE_EFFECTS_COMMENT);
                }
                func.print_expr(p, precedence, ctx);
            }
            Self::YieldExpression(expr) => expr.print_expr(p, precedence, ctx),
            Self::UpdateExpression(expr) => expr.print_expr(p, precedence, ctx),
            Self::UnaryExpression(expr) => expr.print_expr(p, precedence, ctx),
            Self::BinaryExpression(expr) => expr.print_expr(p, precedence, ctx),
            Self::PrivateInExpression(expr) => expr.print_expr(p, precedence, ctx),
            Self::LogicalExpression(expr) => expr.print_expr(p, precedence, ctx),
            Self::ConditionalExpression(expr) => expr.print_expr(p, precedence, ctx),
            Self::AssignmentExpression(expr) => expr.print_expr(p, precedence, ctx),
            Self::SequenceExpression(expr) => expr.print_expr(p, precedence, ctx),
            Self::ImportExpression(expr) => expr.print_expr(p, precedence, ctx),
            Self::TemplateLiteral(literal) => literal.print(p, ctx),
            Self::TaggedTemplateExpression(expr) => expr.print(p, ctx),
            Self::Super(sup) => sup.print(p, ctx),
            Self::AwaitExpression(expr) => expr.print_expr(p, precedence, ctx),
            Self::ChainExpression(expr) => expr.print_expr(p, precedence, ctx),
            Self::NewExpression(expr) => expr.print_expr(p, precedence, ctx),
            Self::MetaProperty(expr) => expr.print(p, ctx),
            Self::ClassExpression(expr) => expr.print(p, ctx),
            Self::JSXElement(el) => el.print(p, ctx),
            Self::JSXFragment(fragment) => fragment.print(p, ctx),
            Self::ParenthesizedExpression(e) => e.print_expr(p, precedence, ctx),
            Self::TSAsExpression(e) => e.print_expr(p, precedence, ctx),
            Self::TSSatisfiesExpression(e) => e.print_expr(p, precedence, ctx),
            Self::TSTypeAssertion(e) => e.print_expr(p, precedence, ctx),
            Self::TSNonNullExpression(e) => e.print_expr(p, precedence, ctx),
            Self::TSInstantiationExpression(e) => e.print_expr(p, precedence, ctx),
            Self::V8IntrinsicExpression(e) => e.print_expr(p, precedence, ctx),
        }
    }
}

impl GenExpr for ParenthesizedExpression<'_> {
    fn gen_expr(&self, p: &mut Codegen, precedence: Precedence, ctx: Context) {
        self.expression.print_expr(p, precedence, ctx);
    }
}

impl Gen for IdentifierReference<'_> {
    fn r#gen(&self, p: &mut Codegen, _ctx: Context) {
        let name = p.get_identifier_reference_name(self);
        p.print_space_before_identifier();
        p.add_source_mapping_for_name(self.span, name);
        p.print_str(name);
    }
}

impl Gen for IdentifierName<'_> {
    fn r#gen(&self, p: &mut Codegen, _ctx: Context) {
        p.print_space_before_identifier();
        p.add_source_mapping(self.span);
        p.print_str(self.name.as_str());
    }
}

impl Gen for BindingIdentifier<'_> {
    fn r#gen(&self, p: &mut Codegen, _ctx: Context) {
        let name = p.get_binding_identifier_name(self);
        p.print_space_before_identifier();
        p.add_source_mapping_for_name(self.span, name);
        p.print_str(name);
    }
}

impl Gen for LabelIdentifier<'_> {
    fn r#gen(&self, p: &mut Codegen, _ctx: Context) {
        p.print_space_before_identifier();
        p.add_source_mapping_for_name(self.span, &self.name);
        p.print_str(self.name.as_str());
    }
}

impl Gen for BooleanLiteral {
    fn r#gen(&self, p: &mut Codegen, _ctx: Context) {
        p.add_source_mapping(self.span);
        p.print_space_before_identifier();
        p.print_str(self.as_str());
    }
}

impl Gen for NullLiteral {
    fn r#gen(&self, p: &mut Codegen, _ctx: Context) {
        p.print_space_before_identifier();
        p.add_source_mapping(self.span);
        p.print_str("null");
    }
}

impl GenExpr for NumericLiteral<'_> {
    fn gen_expr(&self, p: &mut Codegen, precedence: Precedence, ctx: Context) {
        p.add_source_mapping(self.span);
        let value = self.value;
        if ctx.contains(Context::TYPESCRIPT) {
            p.print_str(&self.raw_str());
        } else if value.is_nan() {
            p.print_space_before_identifier();
            p.print_str("NaN");
        } else if value.is_infinite() {
            let wrap = (p.options.minify && precedence >= Precedence::Multiply)
                || (value.is_sign_negative() && precedence >= Precedence::Prefix);
            p.wrap(wrap, |p| {
                if value.is_sign_negative() {
                    p.print_space_before_operator(Operator::Unary(UnaryOperator::UnaryNegation));
                    p.print_ascii_byte(b'-');
                } else {
                    p.print_space_before_identifier();
                }
                if p.options.minify {
                    p.print_str("1/0");
                } else {
                    p.print_str("Infinity");
                }
            });
        } else if value.is_sign_positive() {
            p.print_space_before_identifier();
            p.print_non_negative_float(value);
        } else if precedence >= Precedence::Prefix {
            p.print_str("(-");
            p.print_non_negative_float(value.abs());
            p.print_ascii_byte(b')');
        } else {
            p.print_space_before_operator(Operator::Unary(UnaryOperator::UnaryNegation));
            p.print_ascii_byte(b'-');
            p.print_non_negative_float(value.abs());
        }
    }
}

impl GenExpr for BigIntLiteral<'_> {
    fn gen_expr(&self, p: &mut Codegen, precedence: Precedence, _ctx: Context) {
        p.print_space_before_identifier();
        p.add_source_mapping(self.span);
        let value = self.value.as_str();
        if value.starts_with('-') && precedence >= Precedence::Prefix {
            p.print_ascii_byte(b'(');
            p.print_str(value);
            p.print_str("n)");
        } else {
            p.print_str(value);
            p.print_ascii_byte(b'n');
        }
    }
}

impl Gen for RegExpLiteral<'_> {
    fn r#gen(&self, p: &mut Codegen, _ctx: Context) {
        p.add_source_mapping(self.span);
        // Avoid forming a single-line comment or "</script" sequence
        let last = p.last_byte();
        if last == Some(b'/')
            || (last == Some(b'<')
                && self
                    .regex
                    .pattern
                    .text
                    .get(..6)
                    .is_some_and(|first_six| first_six.cow_to_ascii_lowercase() == "script"))
        {
            p.print_hard_space();
        }
        p.print_ascii_byte(b'/');
        p.print_str(self.regex.pattern.text.as_str());
        p.print_ascii_byte(b'/');
        p.print_str(self.regex.flags.to_inline_string().as_str());
        p.prev_reg_exp_end = p.code().len();
    }
}

impl Gen for StringLiteral<'_> {
    fn r#gen(&self, p: &mut Codegen, _ctx: Context) {
        p.print_string_literal(self, true);
    }
}

impl Gen for ThisExpression {
    fn r#gen(&self, p: &mut Codegen, _ctx: Context) {
        p.print_space_before_identifier();
        p.add_source_mapping(self.span);
        p.print_str("this");
    }
}

impl GenExpr for MemberExpression<'_> {
    fn gen_expr(&self, p: &mut Codegen, precedence: Precedence, ctx: Context) {
        match self {
            Self::ComputedMemberExpression(expr) => expr.print_expr(p, precedence, ctx),
            Self::StaticMemberExpression(expr) => expr.print_expr(p, precedence, ctx),
            Self::PrivateFieldExpression(expr) => expr.print_expr(p, precedence, ctx),
        }
    }
}

impl GenExpr for ComputedMemberExpression<'_> {
    fn gen_expr(&self, p: &mut Codegen, _precedence: Precedence, ctx: Context) {
        // `(let[0] = 100);` -> `(let)[0] = 100`;
        let wrap = self.object.get_identifier_reference().is_some_and(|r| r.name == "let");
        p.wrap(wrap, |p| {
            self.object.print_expr(p, Precedence::Postfix, ctx.intersection(Context::FORBID_CALL));
        });
        if self.optional {
            p.print_str("?.");
        }
        p.print_ascii_byte(b'[');
        self.expression.print_expr(p, Precedence::Lowest, Context::empty());
        p.print_ascii_byte(b']');
    }
}

impl GenExpr for StaticMemberExpression<'_> {
    fn gen_expr(&self, p: &mut Codegen, _precedence: Precedence, ctx: Context) {
        self.object.print_expr(p, Precedence::Postfix, ctx.intersection(Context::FORBID_CALL));
        if self.optional {
            p.print_ascii_byte(b'?');
        } else if p.need_space_before_dot == p.code_len() {
            // `0.toExponential()` is invalid, add a space before the dot, `0 .toExponential()` is valid
            p.print_hard_space();
        }
        p.print_ascii_byte(b'.');
        self.property.print(p, ctx);
    }
}

impl GenExpr for PrivateFieldExpression<'_> {
    fn gen_expr(&self, p: &mut Codegen, _precedence: Precedence, ctx: Context) {
        self.object.print_expr(p, Precedence::Postfix, ctx.intersection(Context::FORBID_CALL));
        if self.optional {
            p.print_str("?");
        }
        p.print_ascii_byte(b'.');
        self.field.print(p, ctx);
    }
}

impl GenExpr for CallExpression<'_> {
    fn gen_expr(&self, p: &mut Codegen, precedence: Precedence, ctx: Context) {
        let is_statement = p.start_of_stmt == p.code_len();
        let is_export_default = p.start_of_default_export == p.code_len();
        let mut wrap = precedence >= Precedence::New || ctx.intersects(Context::FORBID_CALL);
        let pure = self.pure && p.options.print_annotation_comment();
        if !wrap && pure && precedence >= Precedence::Postfix {
            wrap = true;
        }

        p.wrap(wrap, |p| {
            if pure {
                p.print_str(PURE_COMMENT);
            }
            if is_export_default {
                p.start_of_default_export = p.code_len();
            } else if is_statement {
                p.start_of_stmt = p.code_len();
            }
            self.callee.print_expr(p, Precedence::Postfix, Context::empty());
            if self.optional {
                p.print_str("?.");
            }
            if let Some(type_parameters) = &self.type_arguments {
                type_parameters.print(p, ctx);
            }
            p.print_arguments(self.span, &self.arguments, ctx);
            p.add_source_mapping_end(self.span);
        });
    }
}

impl Gen for Argument<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        match self {
            Self::SpreadElement(elem) => elem.print(p, ctx),
            _ => self.to_expression().print_expr(p, Precedence::Comma, Context::empty()),
        }
    }
}

impl Gen for ArrayExpressionElement<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        match self {
            Self::SpreadElement(elem) => elem.print(p, ctx),
            Self::Elision(_span) => {}
            _ => self.to_expression().print_expr(p, Precedence::Comma, Context::empty()),
        }
    }
}

impl Gen for SpreadElement<'_> {
    fn r#gen(&self, p: &mut Codegen, _ctx: Context) {
        p.add_source_mapping(self.span);
        p.print_ellipsis();
        self.argument.print_expr(p, Precedence::Comma, Context::empty());
    }
}

impl Gen for ArrayExpression<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        let is_multi_line = self.elements.len() > 2;
        p.add_source_mapping(self.span);
        p.print_ascii_byte(b'[');
        if is_multi_line {
            p.indent();
        }
        for (i, item) in self.elements.iter().enumerate() {
            if i != 0 {
                p.print_comma();
            }
            if is_multi_line {
                p.print_soft_newline();
                p.print_indent();
            } else if i != 0 {
                p.print_soft_space();
            }
            item.print(p, ctx);
            if i == self.elements.len() - 1 && matches!(item, ArrayExpressionElement::Elision(_)) {
                p.print_comma();
            }
        }
        if is_multi_line {
            p.print_soft_newline();
            p.dedent();
            p.print_indent();
        }
        p.add_source_mapping_end(self.span);
        p.print_ascii_byte(b']');
    }
}

impl GenExpr for ObjectExpression<'_> {
    fn gen_expr(&self, p: &mut Codegen, _precedence: Precedence, ctx: Context) {
        let n = p.code_len();
        let len = self.properties.len();
        let is_multi_line = len > 1;
        let has_comment = p.has_comment(self.span.start);
        let wrap = has_comment || p.start_of_stmt == n || p.start_of_arrow_expr == n;
        p.wrap(wrap, |p| {
            // Print comments for lingui https://lingui.dev/ref/macro#definemessage
            // `const message = /*i18n*/ { };`
            if has_comment {
                p.print_leading_comments(self.span.start);
                p.print_indent();
            }
            p.add_source_mapping(self.span);
            p.print_ascii_byte(b'{');
            if is_multi_line {
                p.indent();
            }
            for (i, item) in self.properties.iter().enumerate() {
                if i != 0 {
                    p.print_comma();
                }
                if is_multi_line {
                    p.print_soft_newline();
                    p.print_indent();
                } else {
                    p.print_soft_space();
                }
                item.print(p, ctx);
            }
            if is_multi_line {
                p.print_soft_newline();
                p.dedent();
                p.print_indent();
            } else if len > 0 {
                p.print_soft_space();
            }
            p.add_source_mapping_end(self.span);
            p.print_ascii_byte(b'}');
        });
    }
}

impl Gen for ObjectPropertyKind<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        match self {
            Self::ObjectProperty(prop) => prop.print(p, ctx),
            Self::SpreadProperty(elem) => elem.print(p, ctx),
        }
    }
}

impl Gen for ObjectProperty<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        if let Expression::FunctionExpression(func) = &self.value {
            p.add_source_mapping(self.span);
            let is_accessor = match &self.kind {
                PropertyKind::Init => false,
                PropertyKind::Get => {
                    p.add_source_mapping(self.span);
                    p.print_str("get");
                    p.print_soft_space();
                    true
                }
                PropertyKind::Set => {
                    p.add_source_mapping(self.span);
                    p.print_str("set");
                    p.print_soft_space();
                    true
                }
            };
            if self.method || is_accessor {
                if func.r#async {
                    p.print_space_before_identifier();
                    p.print_str("async");
                    p.print_soft_space();
                }
                if func.generator {
                    p.print_str("*");
                }
                if self.computed {
                    p.print_ascii_byte(b'[');
                }
                self.key.print(p, ctx);
                if self.computed {
                    p.print_ascii_byte(b']');
                }
                if let Some(type_parameters) = &func.type_parameters {
                    type_parameters.print(p, ctx);
                }
                p.print_ascii_byte(b'(');
                func.params.print(p, ctx);
                p.print_ascii_byte(b')');
                if let Some(return_type) = &func.return_type {
                    p.print_colon();
                    p.print_soft_space();
                    return_type.print(p, ctx);
                }
                if let Some(body) = &func.body {
                    p.print_soft_space();
                    body.print(p, ctx);
                }
                return;
            }
        }

        let mut shorthand = false;
        if let PropertyKey::StaticIdentifier(key) = &self.key {
            if key.name == "__proto__" {
                shorthand = self.shorthand;
            } else if let Expression::Identifier(ident) = self.value.without_parentheses() {
                if key.name == p.get_identifier_reference_name(ident) {
                    shorthand = true;
                }
            }
        }

        let mut computed = self.computed;

        // "{ -1: 0 }" must be printed as "{ [-1]: 0 }"
        // "{ 1/0: 0 }" must be printed as "{ [1/0]: 0 }"
        if !computed {
            if let Some(Expression::NumericLiteral(n)) = self.key.as_expression() {
                if n.value.is_sign_negative() || n.value.is_infinite() {
                    computed = true;
                }
            }
        }

        if computed {
            p.print_ascii_byte(b'[');
        }
        if !shorthand {
            self.key.print(p, ctx);
        }
        if computed {
            p.print_ascii_byte(b']');
        }
        if !shorthand {
            p.print_colon();
            p.print_soft_space();
        }
        self.value.print_expr(p, Precedence::Comma, Context::empty());
    }
}

impl Gen for PropertyKey<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        match self {
            Self::StaticIdentifier(ident) => ident.print(p, ctx),
            Self::PrivateIdentifier(ident) => ident.print(p, ctx),
            Self::StringLiteral(s) => p.print_string_literal(s, /* allow_backtick */ false),
            _ => self.to_expression().print_expr(p, Precedence::Comma, Context::empty()),
        }
    }
}

impl GenExpr for ArrowFunctionExpression<'_> {
    fn gen_expr(&self, p: &mut Codegen, precedence: Precedence, ctx: Context) {
        p.wrap(precedence >= Precedence::Assign || self.pife, |p| {
            if self.r#async {
                p.print_space_before_identifier();
                p.add_source_mapping(self.span);
                p.print_str("async");
                p.print_soft_space();
            }
            if let Some(type_parameters) = &self.type_parameters {
                type_parameters.print(p, ctx);
            }
            p.add_source_mapping(self.span);
            let remove_params_wrap = p.options.minify
                && self.params.items.len() == 1
                && self.params.rest.is_none()
                && self.type_parameters.is_none()
                && self.return_type.is_none()
                && {
                    let param = &self.params.items[0];
                    param.decorators.is_empty()
                        && !param.has_modifier()
                        && param.pattern.kind.is_binding_identifier()
                        && param.pattern.type_annotation.is_none()
                        && !param.pattern.optional
                };
            p.wrap(!remove_params_wrap, |p| {
                self.params.print(p, ctx);
            });
            if let Some(return_type) = &self.return_type {
                p.print_str(":");
                p.print_soft_space();
                return_type.print(p, ctx);
            }
            p.print_soft_space();
            p.print_str("=>");
            p.print_soft_space();
            if self.expression {
                if let Some(Statement::ExpressionStatement(stmt)) = &self.body.statements.first() {
                    p.start_of_arrow_expr = p.code_len();
                    stmt.expression.print_expr(p, Precedence::Comma, ctx);
                }
            } else {
                self.body.print(p, ctx);
            }
        });
    }
}

impl GenExpr for YieldExpression<'_> {
    fn gen_expr(&self, p: &mut Codegen, precedence: Precedence, _ctx: Context) {
        p.wrap(precedence >= Precedence::Assign, |p| {
            p.print_space_before_identifier();
            p.add_source_mapping(self.span);
            p.print_str("yield");
            if self.delegate {
                p.print_ascii_byte(b'*');
            }
            if let Some(argument) = self.argument.as_ref() {
                p.print_soft_space();
                argument.print_expr(p, Precedence::Yield, Context::empty());
            }
        });
    }
}

impl GenExpr for UpdateExpression<'_> {
    fn gen_expr(&self, p: &mut Codegen, precedence: Precedence, ctx: Context) {
        let operator = self.operator.as_str();
        p.wrap(precedence >= self.precedence(), |p| {
            if self.prefix {
                p.add_source_mapping(self.span);
                p.print_space_before_operator(self.operator.into());
                p.print_str(operator);
                p.prev_op = Some(self.operator.into());
                p.prev_op_end = p.code().len();
                self.argument.print_expr(p, Precedence::Prefix, ctx);
            } else {
                p.print_space_before_operator(self.operator.into());
                self.argument.print_expr(p, Precedence::Postfix, ctx);
                p.print_str(operator);
                p.prev_op = Some(self.operator.into());
                p.prev_op_end = p.code().len();
            }
        });
    }
}

impl GenExpr for UnaryExpression<'_> {
    fn gen_expr(&self, p: &mut Codegen, precedence: Precedence, ctx: Context) {
        p.wrap(precedence >= self.precedence(), |p| {
            let operator = self.operator.as_str();
            if self.operator.is_keyword() {
                p.print_space_before_identifier();
                p.add_source_mapping(self.span);
                p.print_str(operator);
                p.print_soft_space();
            } else {
                p.print_space_before_operator(self.operator.into());
                p.add_source_mapping(self.span);
                p.print_str(operator);
                p.prev_op = Some(self.operator.into());
                p.prev_op_end = p.code().len();
            }
            // Forbid `delete Infinity`, which is syntax error in strict mode.
            let is_delete_infinity = self.operator == UnaryOperator::Delete
                && !p.options.minify
                && matches!(&self.argument, Expression::NumericLiteral(lit) if lit.value.is_sign_positive() && lit.value.is_infinite());
            if is_delete_infinity {
                p.print_str("(0,");
                p.print_soft_space();
            }
            self.argument.print_expr(p, Precedence::Exponentiation, ctx);
            if is_delete_infinity{
                p.print_ascii_byte(b')');
            }
        });
    }
}

impl GenExpr for BinaryExpression<'_> {
    fn gen_expr(&self, p: &mut Codegen, precedence: Precedence, ctx: Context) {
        let v = BinaryExpressionVisitor {
            // SAFETY:
            // The pointer is stored on the heap and all will be consumed in the binary expression visitor.
            e: Binaryish::Binary(unsafe {
                std::mem::transmute::<&BinaryExpression<'_>, &BinaryExpression<'_>>(self)
            }),
            precedence,
            ctx,
            left_precedence: Precedence::Lowest,
            operator: BinaryishOperator::Binary(self.operator),
            wrap: false,
            right_precedence: Precedence::Lowest,
        };
        BinaryExpressionVisitor::gen_expr(v, p);
    }
}

impl GenExpr for PrivateInExpression<'_> {
    fn gen_expr(&self, p: &mut Codegen, precedence: Precedence, ctx: Context) {
        p.wrap(precedence >= Precedence::Compare, |p| {
            self.left.print(p, ctx);
            p.print_str(" in ");
            self.right.print_expr(p, Precedence::Equals, Context::FORBID_IN);
        });
    }
}

impl GenExpr for LogicalExpression<'_> {
    fn gen_expr(&self, p: &mut Codegen, precedence: Precedence, ctx: Context) {
        let v = BinaryExpressionVisitor {
            // SAFETY:
            // The pointer is stored on the heap and all will be consumed in the binary expression visitor.
            e: Binaryish::Logical(unsafe {
                std::mem::transmute::<&LogicalExpression<'_>, &LogicalExpression<'_>>(self)
            }),
            precedence,
            ctx,
            left_precedence: Precedence::Lowest,
            operator: BinaryishOperator::Logical(self.operator),
            wrap: false,
            right_precedence: Precedence::Lowest,
        };
        BinaryExpressionVisitor::gen_expr(v, p);
    }
}

impl GenExpr for ConditionalExpression<'_> {
    fn gen_expr(&self, p: &mut Codegen, precedence: Precedence, ctx: Context) {
        let mut ctx = ctx;
        let wrap = precedence >= self.precedence();
        if wrap {
            ctx &= Context::FORBID_IN.not();
        }
        p.wrap(wrap, |p| {
            self.test.print_expr(p, Precedence::Conditional, ctx & Context::FORBID_IN);
            p.print_soft_space();
            p.print_ascii_byte(b'?');
            p.print_soft_space();
            self.consequent.print_expr(p, Precedence::Yield, Context::empty());
            p.print_soft_space();
            p.print_colon();
            p.print_soft_space();
            self.alternate.print_expr(p, Precedence::Yield, ctx & Context::FORBID_IN);
        });
    }
}

impl GenExpr for AssignmentExpression<'_> {
    fn gen_expr(&self, p: &mut Codegen, precedence: Precedence, ctx: Context) {
        let n = p.code_len();
        // Destructuring assignments must be parenthesized
        let wrap = (p.start_of_stmt == n || p.start_of_arrow_expr == n)
            && matches!(self.left, AssignmentTarget::ObjectAssignmentTarget(_));
        p.wrap(wrap || precedence >= self.precedence(), |p| {
            self.left.print(p, ctx);
            p.print_soft_space();
            p.print_str(self.operator.as_str());
            p.print_soft_space();
            self.right.print_expr(p, Precedence::Comma, ctx);
        });
    }
}

impl Gen for AssignmentTarget<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        match self {
            match_simple_assignment_target!(Self) => {
                self.to_simple_assignment_target().print_expr(
                    p,
                    Precedence::Comma,
                    Context::empty(),
                );
            }
            match_assignment_target_pattern!(Self) => {
                self.to_assignment_target_pattern().print(p, ctx);
            }
        }
    }
}

impl GenExpr for SimpleAssignmentTarget<'_> {
    fn gen_expr(&self, p: &mut Codegen, precedence: Precedence, ctx: Context) {
        match self {
            Self::AssignmentTargetIdentifier(ident) => ident.print(p, ctx),
            Self::ComputedMemberExpression(expr) => expr.print_expr(p, precedence, ctx),
            Self::StaticMemberExpression(expr) => expr.print_expr(p, precedence, ctx),
            Self::PrivateFieldExpression(expr) => expr.print_expr(p, precedence, ctx),
            Self::TSAsExpression(e) => e.print_expr(p, precedence, ctx),
            Self::TSSatisfiesExpression(e) => e.print_expr(p, precedence, ctx),
            Self::TSNonNullExpression(e) => e.print_expr(p, precedence, ctx),
            Self::TSTypeAssertion(e) => e.print_expr(p, precedence, ctx),
        }
    }
}

impl Gen for AssignmentTargetPattern<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        match self {
            Self::ArrayAssignmentTarget(target) => target.print(p, ctx),
            Self::ObjectAssignmentTarget(target) => target.print(p, ctx),
        }
    }
}

impl Gen for ArrayAssignmentTarget<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.add_source_mapping(self.span);
        p.print_ascii_byte(b'[');
        for (i, item) in self.elements.iter().enumerate() {
            if i != 0 {
                p.print_comma();
                p.print_soft_space();
            }
            if let Some(item) = item {
                item.print(p, ctx);
            }
            if i == self.elements.len() - 1 && (item.is_none() || self.rest.is_some()) {
                p.print_comma();
            }
        }
        if let Some(target) = &self.rest {
            if !self.elements.is_empty() {
                p.print_soft_space();
            }
            p.add_source_mapping(self.span);
            target.print(p, ctx);
        }
        p.print_ascii_byte(b']');
        p.add_source_mapping_end(self.span);
    }
}

impl Gen for ObjectAssignmentTarget<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.add_source_mapping(self.span);
        p.print_ascii_byte(b'{');
        p.print_list(&self.properties, ctx);
        if let Some(target) = &self.rest {
            if !self.properties.is_empty() {
                p.print_comma();
                p.print_soft_space();
            }
            p.add_source_mapping(self.span);
            target.print(p, ctx);
        }
        p.print_ascii_byte(b'}');
        p.add_source_mapping_end(self.span);
    }
}

impl Gen for AssignmentTargetMaybeDefault<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        match self {
            match_assignment_target!(Self) => self.to_assignment_target().print(p, ctx),
            Self::AssignmentTargetWithDefault(target) => target.print(p, ctx),
        }
    }
}

impl Gen for AssignmentTargetWithDefault<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        self.binding.print(p, ctx);
        p.print_soft_space();
        p.print_equal();
        p.print_soft_space();
        self.init.print_expr(p, Precedence::Comma, Context::empty());
    }
}

impl Gen for AssignmentTargetProperty<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        match self {
            Self::AssignmentTargetPropertyIdentifier(ident) => ident.print(p, ctx),
            Self::AssignmentTargetPropertyProperty(prop) => prop.print(p, ctx),
        }
    }
}

impl Gen for AssignmentTargetPropertyIdentifier<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        let ident_name = p.get_identifier_reference_name(&self.binding);
        if ident_name == self.binding.name.as_str() {
            self.binding.print(p, ctx);
        } else {
            // `({x: a} = y);`
            p.print_str(self.binding.name.as_str());
            p.print_colon();
            p.print_soft_space();
            p.print_str(ident_name);
        }
        if let Some(expr) = &self.init {
            p.print_soft_space();
            p.print_equal();
            p.print_soft_space();
            expr.print_expr(p, Precedence::Comma, Context::empty());
        }
    }
}

impl Gen for AssignmentTargetPropertyProperty<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        let omit_key = if p.options.minify {
            let key_name = match &self.name {
                PropertyKey::StaticIdentifier(ident) => Some(&ident.name),
                _ => None,
            };
            let value_name =
                self.binding.identifier().map(|id| p.get_identifier_reference_name(id));
            match (key_name, value_name) {
                (Some(key_name), Some(value_name)) => key_name == value_name,
                _ => false,
            }
        } else {
            false
        };
        if !omit_key {
            match &self.name {
                PropertyKey::StaticIdentifier(ident) => {
                    ident.print(p, ctx);
                }
                PropertyKey::PrivateIdentifier(ident) => {
                    ident.print(p, ctx);
                }
                key => {
                    if self.computed {
                        p.print_ascii_byte(b'[');
                    }
                    key.to_expression().print_expr(p, Precedence::Comma, Context::empty());
                    if self.computed {
                        p.print_ascii_byte(b']');
                    }
                }
            }
            p.print_colon();
            p.print_soft_space();
        }
        self.binding.print(p, ctx);
    }
}

impl Gen for AssignmentTargetRest<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.print_ellipsis();
        self.target.print(p, ctx);
    }
}

impl GenExpr for SequenceExpression<'_> {
    fn gen_expr(&self, p: &mut Codegen, precedence: Precedence, ctx: Context) {
        p.wrap(precedence >= self.precedence(), |p| {
            p.print_expressions(&self.expressions, Precedence::Lowest, ctx.and_forbid_call(false));
        });
    }
}

impl GenExpr for ImportExpression<'_> {
    fn gen_expr(&self, p: &mut Codegen, precedence: Precedence, ctx: Context) {
        let wrap = precedence >= Precedence::New || ctx.intersects(Context::FORBID_CALL);

        let has_comment_before_right_paren = p.options.print_annotation_comment()
            && self.span.end > 0
            && p.has_comment(self.span.end - 1);
        let has_comment = p.options.print_annotation_comment()
            && (has_comment_before_right_paren
                || p.has_comment(self.source.span().start)
                || self
                    .options
                    .as_ref()
                    .is_some_and(|options| p.has_comment(options.span().start)));

        p.wrap(wrap, |p| {
            p.print_space_before_identifier();
            p.add_source_mapping(self.span);
            p.print_str("import");
            if let Some(phase) = self.phase {
                p.print_str(phase.as_str());
            }
            p.print_ascii_byte(b'(');
            if has_comment {
                p.indent();
            }
            if p.print_expr_comments(self.source.span().start) {
                p.print_indent();
            } else if has_comment {
                p.print_soft_newline();
                p.print_indent();
            }
            self.source.print_expr(p, Precedence::Comma, Context::empty());
            if let Some(options) = &self.options {
                p.print_comma();
                if has_comment {
                    p.print_soft_newline();
                    p.print_indent();
                } else {
                    p.print_soft_space();
                }
                options.gen_expr(p, Precedence::Comma, Context::empty());
            }
            if has_comment {
                // Handle `/* comment */);`
                if !has_comment_before_right_paren || !p.print_expr_comments(self.span.end - 1) {
                    p.print_soft_newline();
                }
                p.dedent();
            }
            p.print_ascii_byte(b')');
        });
    }
}

impl Gen for TemplateLiteral<'_> {
    fn r#gen(&self, p: &mut Codegen, _ctx: Context) {
        p.add_source_mapping(self.span);
        p.print_ascii_byte(b'`');

        debug_assert_eq!(self.quasis.len(), self.expressions.len() + 1);

        let (first_quasi, remaining_quasis) = self.quasis.split_first().unwrap();

        p.add_source_mapping(first_quasi.span);
        p.print_str_escaping_script_close_tag(first_quasi.value.raw.as_str());
        p.add_source_mapping_end(first_quasi.span);

        for (expr, quasi) in self.expressions.iter().zip(remaining_quasis) {
            p.print_str("${");
            p.print_expression(expr);
            p.print_ascii_byte(b'}');

            p.add_source_mapping(quasi.span);
            p.print_str_escaping_script_close_tag(quasi.value.raw.as_str());
            p.add_source_mapping_end(quasi.span);
        }

        p.print_ascii_byte(b'`');
        p.add_source_mapping_end(self.span);
    }
}

impl Gen for TaggedTemplateExpression<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.add_source_mapping(self.span);
        self.tag.print_expr(p, Precedence::Postfix, Context::empty());
        if let Some(type_parameters) = &self.type_arguments {
            type_parameters.print(p, ctx);
        }
        self.quasi.print(p, ctx);
    }
}

impl Gen for Super {
    fn r#gen(&self, p: &mut Codegen, _ctx: Context) {
        p.print_space_before_identifier();
        p.add_source_mapping(self.span);
        p.print_str("super");
    }
}

impl GenExpr for AwaitExpression<'_> {
    fn gen_expr(&self, p: &mut Codegen, precedence: Precedence, ctx: Context) {
        p.wrap(precedence >= self.precedence(), |p| {
            p.print_space_before_identifier();
            p.add_source_mapping(self.span);
            p.print_str("await");
            p.print_soft_space();
            self.argument.print_expr(p, Precedence::Exponentiation, ctx);
        });
    }
}

impl GenExpr for ChainExpression<'_> {
    fn gen_expr(&self, p: &mut Codegen, precedence: Precedence, ctx: Context) {
        p.wrap(precedence >= Precedence::Postfix, |p| match &self.expression {
            ChainElement::CallExpression(expr) => expr.print_expr(p, precedence, ctx),
            ChainElement::TSNonNullExpression(expr) => expr.print_expr(p, precedence, ctx),
            ChainElement::ComputedMemberExpression(expr) => expr.print_expr(p, precedence, ctx),
            ChainElement::StaticMemberExpression(expr) => expr.print_expr(p, precedence, ctx),
            ChainElement::PrivateFieldExpression(expr) => expr.print_expr(p, precedence, ctx),
        });
    }
}

impl GenExpr for NewExpression<'_> {
    fn gen_expr(&self, p: &mut Codegen, precedence: Precedence, ctx: Context) {
        let mut wrap = precedence >= self.precedence();
        let pure = self.pure && p.options.print_annotation_comment();
        if precedence >= Precedence::Postfix && pure {
            wrap = true;
        }
        p.wrap(wrap, |p| {
            if pure {
                p.print_str(PURE_COMMENT);
            }
            p.print_space_before_identifier();
            p.add_source_mapping(self.span);
            p.print_str("new");
            p.print_soft_space();
            self.callee.print_expr(p, Precedence::New, Context::FORBID_CALL);

            // Omit the "()" when minifying, but only when safe to do so
            if !p.options.minify || !self.arguments.is_empty() || precedence >= Precedence::Postfix
            {
                p.print_arguments(self.span, &self.arguments, ctx);
            }
        });
    }
}

impl GenExpr for TSAsExpression<'_> {
    fn gen_expr(&self, p: &mut Codegen, precedence: Precedence, ctx: Context) {
        let wrap = precedence >= Precedence::Shift;

        p.wrap(wrap, |p| {
            self.expression.print_expr(p, Precedence::Exponentiation, ctx);
            p.print_str(" as ");
            self.type_annotation.print(p, ctx);
        });
    }
}

impl GenExpr for TSSatisfiesExpression<'_> {
    fn gen_expr(&self, p: &mut Codegen, precedence: Precedence, ctx: Context) {
        p.print_ascii_byte(b'(');
        p.print_ascii_byte(b'(');
        self.expression.print_expr(p, precedence, Context::default());
        p.print_ascii_byte(b')');
        p.print_str(" satisfies ");
        self.type_annotation.print(p, ctx);
        p.print_ascii_byte(b')');
    }
}

impl GenExpr for TSNonNullExpression<'_> {
    fn gen_expr(&self, p: &mut Codegen, precedence: Precedence, ctx: Context) {
        p.wrap(matches!(self.expression, Expression::ParenthesizedExpression(_)), |p| {
            self.expression.print_expr(p, precedence, ctx);
        });
        p.print_ascii_byte(b'!');
        if p.options.minify {
            p.print_hard_space();
        }
    }
}

impl GenExpr for TSInstantiationExpression<'_> {
    fn gen_expr(&self, p: &mut Codegen, precedence: Precedence, ctx: Context) {
        self.expression.print_expr(p, precedence, ctx);
        self.type_arguments.print(p, ctx);
        if p.options.minify {
            p.print_hard_space();
        }
    }
}

impl GenExpr for TSTypeAssertion<'_> {
    fn gen_expr(&self, p: &mut Codegen, precedence: Precedence, ctx: Context) {
        p.wrap(precedence >= self.precedence(), |p| {
            p.print_str("<");
            // var r = < <T>(x: T) => T > ((x) => { return null; });
            //          ^ make sure space is printed here.
            if matches!(self.type_annotation, TSType::TSFunctionType(_)) {
                p.print_hard_space();
            }
            self.type_annotation.print(p, ctx);
            p.print_str(">");
            self.expression.print_expr(p, Precedence::Member, ctx);
        });
    }
}

impl Gen for MetaProperty<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.print_space_before_identifier();
        p.add_source_mapping(self.span);
        self.meta.print(p, ctx);
        p.print_ascii_byte(b'.');
        self.property.print(p, ctx);
    }
}

impl Gen for Class<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.add_source_mapping(self.span);
        let n = p.code_len();
        let wrap = self.is_expression() && (p.start_of_stmt == n || p.start_of_default_export == n);
        p.wrap(wrap, |p| {
            p.print_decorators(&self.decorators, ctx);
            p.print_space_before_identifier();
            p.add_source_mapping(self.span);
            if self.declare {
                p.print_str("declare ");
            }
            if self.r#abstract {
                p.print_str("abstract ");
            }
            p.print_str("class");
            if let Some(id) = &self.id {
                p.print_hard_space();
                id.print(p, ctx);
                if let Some(type_parameters) = self.type_parameters.as_ref() {
                    type_parameters.print(p, ctx);
                }
            }
            if let Some(super_class) = self.super_class.as_ref() {
                p.print_str(" extends ");
                super_class.print_expr(p, Precedence::Postfix, Context::empty());
                if let Some(super_type_parameters) = &self.super_type_arguments {
                    super_type_parameters.print(p, ctx);
                }
            }
            if !self.implements.is_empty() {
                p.print_str(" implements ");
                p.print_list(&self.implements, ctx);
            }
            p.print_soft_space();
            self.body.print(p, ctx);
            p.needs_semicolon = false;
        });
    }
}

impl Gen for ClassBody<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.print_curly_braces(self.span, self.body.is_empty(), |p| {
            for item in &self.body {
                p.print_semicolon_if_needed();
                p.print_leading_comments(item.span().start);
                p.print_indent();
                item.print(p, ctx);
            }
        });
    }
}

impl Gen for ClassElement<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        match self {
            Self::StaticBlock(elem) => {
                elem.print(p, ctx);
                p.print_soft_newline();
            }
            Self::MethodDefinition(elem) => {
                elem.print(p, ctx);
                p.print_soft_newline();
            }
            Self::PropertyDefinition(elem) => {
                elem.print(p, ctx);
                p.print_semicolon_after_statement();
            }
            Self::AccessorProperty(elem) => {
                elem.print(p, ctx);
                p.print_semicolon_after_statement();
            }
            Self::TSIndexSignature(elem) => {
                elem.print(p, ctx);
                p.print_semicolon_after_statement();
            }
        }
    }
}

impl Gen for JSXIdentifier<'_> {
    fn r#gen(&self, p: &mut Codegen, _ctx: Context) {
        p.add_source_mapping_for_name(self.span, &self.name);
        p.print_str(self.name.as_str());
    }
}

impl Gen for JSXMemberExpressionObject<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        match self {
            Self::IdentifierReference(ident) => ident.print(p, ctx),
            Self::MemberExpression(member_expr) => member_expr.print(p, ctx),
            Self::ThisExpression(expr) => expr.print(p, ctx),
        }
    }
}

impl Gen for JSXMemberExpression<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        self.object.print(p, ctx);
        p.print_ascii_byte(b'.');
        self.property.print(p, ctx);
    }
}

impl Gen for JSXElementName<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        match self {
            Self::Identifier(identifier) => identifier.print(p, ctx),
            Self::IdentifierReference(identifier) => identifier.print(p, ctx),
            Self::NamespacedName(namespaced_name) => namespaced_name.print(p, ctx),
            Self::MemberExpression(member_expr) => member_expr.print(p, ctx),
            Self::ThisExpression(expr) => expr.print(p, ctx),
        }
    }
}

impl Gen for JSXNamespacedName<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        self.namespace.print(p, ctx);
        p.print_colon();
        self.name.print(p, ctx);
    }
}

impl Gen for JSXAttributeName<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        match self {
            Self::Identifier(ident) => ident.print(p, ctx),
            Self::NamespacedName(namespaced_name) => namespaced_name.print(p, ctx),
        }
    }
}

impl Gen for JSXAttribute<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        self.name.print(p, ctx);
        if let Some(value) = &self.value {
            p.print_equal();
            value.print(p, ctx);
        }
    }
}

impl Gen for JSXEmptyExpression {
    fn r#gen(&self, p: &mut Codegen, _ctx: Context) {
        p.print_comments_at(self.span.end);
    }
}

impl Gen for JSXExpression<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        match self {
            Self::EmptyExpression(expr) => expr.print(p, ctx),
            _ => p.print_expression(self.to_expression()),
        }
    }
}

impl Gen for JSXExpressionContainer<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.print_ascii_byte(b'{');
        self.expression.print(p, ctx);
        p.print_ascii_byte(b'}');
    }
}

impl Gen for JSXAttributeValue<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        match self {
            Self::Fragment(fragment) => fragment.print(p, ctx),
            Self::Element(el) => el.print(p, ctx),
            Self::StringLiteral(lit) => {
                let quote = if lit.value.contains('"') { b'\'' } else { b'"' };
                p.print_ascii_byte(quote);
                p.print_str(&lit.value);
                p.print_ascii_byte(quote);
            }
            Self::ExpressionContainer(expr_container) => expr_container.print(p, ctx),
        }
    }
}

impl Gen for JSXSpreadAttribute<'_> {
    fn r#gen(&self, p: &mut Codegen, _ctx: Context) {
        p.print_str("{...");
        self.argument.print_expr(p, Precedence::Comma, Context::empty());
        p.print_ascii_byte(b'}');
    }
}

impl Gen for JSXAttributeItem<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        match self {
            Self::Attribute(attr) => attr.print(p, ctx),
            Self::SpreadAttribute(spread_attr) => spread_attr.print(p, ctx),
        }
    }
}

impl Gen for JSXElement<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        // Opening element.
        // Cannot `impl Gen for JSXOpeningElement` because it needs to know value of `self.closing_element`
        // to determine whether to print a trailing `/`.
        p.add_source_mapping(self.opening_element.span);
        p.print_ascii_byte(b'<');
        self.opening_element.name.print(p, ctx);
        for attr in &self.opening_element.attributes {
            match attr {
                JSXAttributeItem::Attribute(_) => {
                    p.print_hard_space();
                }
                JSXAttributeItem::SpreadAttribute(_) => {
                    p.print_soft_space();
                }
            }
            attr.print(p, ctx);
        }
        if self.closing_element.is_none() {
            p.print_soft_space();
            p.print_str("/");
        }
        p.print_ascii_byte(b'>');

        // Children
        for child in &self.children {
            child.print(p, ctx);
        }

        // Closing element
        if let Some(closing_element) = &self.closing_element {
            p.add_source_mapping(closing_element.span);
            p.print_str("</");
            closing_element.name.print(p, ctx);
            p.print_ascii_byte(b'>');
        }
    }
}

impl Gen for JSXOpeningFragment {
    fn r#gen(&self, p: &mut Codegen, _ctx: Context) {
        p.add_source_mapping(self.span);
        p.print_str("<>");
    }
}

impl Gen for JSXClosingFragment {
    fn r#gen(&self, p: &mut Codegen, _ctx: Context) {
        p.add_source_mapping(self.span);
        p.print_str("</>");
    }
}

impl Gen for JSXText<'_> {
    fn r#gen(&self, p: &mut Codegen, _ctx: Context) {
        p.add_source_mapping(self.span);
        p.print_str(self.value.as_str());
    }
}

impl Gen for JSXSpreadChild<'_> {
    fn r#gen(&self, p: &mut Codegen, _ctx: Context) {
        p.print_str("{...");
        p.print_expression(&self.expression);
        p.print_ascii_byte(b'}');
    }
}

impl Gen for JSXChild<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        match self {
            Self::Fragment(fragment) => fragment.print(p, ctx),
            Self::Element(el) => el.print(p, ctx),
            Self::Spread(spread) => spread.print(p, ctx),
            Self::ExpressionContainer(expr_container) => expr_container.print(p, ctx),
            Self::Text(text) => text.print(p, ctx),
        }
    }
}

impl Gen for JSXFragment<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        self.opening_fragment.print(p, ctx);
        for child in &self.children {
            child.print(p, ctx);
        }
        self.closing_fragment.print(p, ctx);
    }
}

impl Gen for StaticBlock<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.add_source_mapping(self.span);
        p.print_str("static");
        p.print_soft_space();
        p.print_curly_braces(self.span, self.body.is_empty(), |p| {
            for stmt in &self.body {
                p.print_semicolon_if_needed();
                stmt.print(p, ctx);
            }
        });
        p.needs_semicolon = false;
    }
}

impl Gen for MethodDefinition<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.add_source_mapping(self.span);
        p.print_decorators(&self.decorators, ctx);
        if let Some(accessibility) = &self.accessibility {
            p.print_space_before_identifier();
            p.print_str(accessibility.as_str());
            p.print_soft_space();
        }
        if self.r#type == MethodDefinitionType::TSAbstractMethodDefinition {
            p.print_space_before_identifier();
            p.print_str("abstract");
            p.print_soft_space();
        }
        if self.r#static {
            p.print_space_before_identifier();
            p.add_source_mapping(self.span);
            p.print_str("static");
            p.print_soft_space();
        }
        match &self.kind {
            MethodDefinitionKind::Constructor | MethodDefinitionKind::Method => {}
            MethodDefinitionKind::Get => {
                p.print_space_before_identifier();
                p.add_source_mapping(self.span);
                p.print_str("get");
                p.print_soft_space();
            }
            MethodDefinitionKind::Set => {
                p.print_space_before_identifier();
                p.add_source_mapping(self.span);
                p.print_str("set");
                p.print_soft_space();
            }
        }
        if self.value.r#async {
            p.print_space_before_identifier();
            p.add_source_mapping(self.span);
            p.print_str("async");
            p.print_soft_space();
        }
        if self.value.generator {
            p.add_source_mapping(self.span);
            p.print_str("*");
        }
        if self.computed {
            p.print_ascii_byte(b'[');
        }
        self.key.print(p, ctx);
        if self.computed {
            p.print_ascii_byte(b']');
        }
        if self.optional {
            p.print_ascii_byte(b'?');
        }
        if let Some(type_parameters) = self.value.type_parameters.as_ref() {
            type_parameters.print(p, ctx);
        }
        p.print_ascii_byte(b'(');
        if let Some(this_param) = &self.value.this_param {
            this_param.print(p, ctx);
            if !self.value.params.is_empty() || self.value.params.rest.is_some() {
                p.print_str(",");
                p.print_soft_space();
            }
        }
        self.value.params.print(p, ctx);
        p.print_ascii_byte(b')');
        if let Some(return_type) = &self.value.return_type {
            p.print_colon();
            p.print_soft_space();
            return_type.print(p, ctx);
        }
        if let Some(body) = &self.value.body {
            p.print_soft_space();
            body.print(p, ctx);
        } else {
            p.print_semicolon();
        }
    }
}

impl Gen for PropertyDefinition<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.add_source_mapping(self.span);
        p.print_decorators(&self.decorators, ctx);
        if self.declare {
            p.print_space_before_identifier();
            p.print_str("declare");
            p.print_soft_space();
        }
        if let Some(accessibility) = self.accessibility {
            p.print_space_before_identifier();
            p.print_str(accessibility.as_str());
            p.print_soft_space();
        }
        if self.r#type == PropertyDefinitionType::TSAbstractPropertyDefinition {
            p.print_space_before_identifier();
            p.print_str("abstract");
            p.print_soft_space();
        }
        if self.r#static {
            p.print_space_before_identifier();
            p.add_source_mapping(self.span);
            p.print_str("static");
            p.print_soft_space();
        }
        if self.readonly {
            p.print_space_before_identifier();
            p.print_str("readonly");
            p.print_soft_space();
        }
        if self.computed {
            p.print_ascii_byte(b'[');
        }
        self.key.print(p, ctx);
        if self.computed {
            p.print_ascii_byte(b']');
        }
        if self.optional {
            p.print_str("?");
        }
        if let Some(type_annotation) = &self.type_annotation {
            p.print_colon();
            p.print_soft_space();
            type_annotation.print(p, ctx);
        }
        if let Some(value) = &self.value {
            p.print_soft_space();
            p.print_equal();
            p.print_soft_space();
            value.print_expr(p, Precedence::Comma, Context::empty());
        }
    }
}

impl Gen for AccessorProperty<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.add_source_mapping(self.span);
        p.print_decorators(&self.decorators, ctx);
        if self.r#type.is_abstract() {
            p.print_space_before_identifier();
            p.add_source_mapping(self.span);
            p.print_str("abstract");
            p.print_soft_space();
        }
        if let Some(accessibility) = self.accessibility {
            p.print_space_before_identifier();
            p.print_str(accessibility.as_str());
            p.print_soft_space();
        }
        if self.r#static {
            p.print_space_before_identifier();
            p.add_source_mapping(self.span);
            p.print_str("static");
            p.print_soft_space();
        }
        if self.r#override {
            p.print_space_before_identifier();
            p.print_str("override");
            p.print_soft_space();
        }
        p.print_space_before_identifier();
        p.print_str("accessor");
        if self.computed {
            p.print_soft_space();
            p.print_ascii_byte(b'[');
        } else {
            p.print_hard_space();
        }
        self.key.print(p, ctx);
        if self.computed {
            p.print_ascii_byte(b']');
        }
        if let Some(type_annotation) = &self.type_annotation {
            p.print_colon();
            p.print_soft_space();
            type_annotation.print(p, ctx);
        }
        if let Some(value) = &self.value {
            p.print_soft_space();
            p.print_equal();
            p.print_soft_space();
            value.print_expr(p, Precedence::Comma, Context::empty());
        }
    }
}

impl Gen for PrivateIdentifier<'_> {
    fn r#gen(&self, p: &mut Codegen, _ctx: Context) {
        p.add_source_mapping_for_name(self.span, &self.name);
        p.print_ascii_byte(b'#');
        p.print_str(self.name.as_str());
    }
}

impl Gen for BindingPattern<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        self.kind.print(p, ctx);
        if self.optional {
            p.print_str("?");
        }
        if let Some(type_annotation) = &self.type_annotation {
            p.print_colon();
            p.print_soft_space();
            type_annotation.print(p, ctx);
        }
    }
}

impl Gen for BindingPatternKind<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        match self {
            BindingPatternKind::BindingIdentifier(ident) => ident.print(p, ctx),
            BindingPatternKind::ObjectPattern(pattern) => pattern.print(p, ctx),
            BindingPatternKind::ArrayPattern(pattern) => pattern.print(p, ctx),
            BindingPatternKind::AssignmentPattern(pattern) => pattern.print(p, ctx),
        }
    }
}

impl Gen for ObjectPattern<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.add_source_mapping(self.span);
        p.print_ascii_byte(b'{');
        if !self.is_empty() {
            p.print_soft_space();
        }
        p.print_list(&self.properties, ctx);
        if let Some(rest) = &self.rest {
            if !self.properties.is_empty() {
                p.print_comma();
            }
            rest.print(p, ctx);
        }
        if !self.is_empty() {
            p.print_soft_space();
        }
        p.print_ascii_byte(b'}');
        p.add_source_mapping_end(self.span);
    }
}

impl Gen for BindingProperty<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.add_source_mapping(self.span);
        if self.computed {
            p.print_ascii_byte(b'[');
        }

        let mut shorthand = false;
        if let PropertyKey::StaticIdentifier(key) = &self.key {
            match &self.value.kind {
                BindingPatternKind::BindingIdentifier(ident)
                    if key.name == p.get_binding_identifier_name(ident) =>
                {
                    shorthand = true;
                }
                BindingPatternKind::AssignmentPattern(assignment_pattern) => {
                    if let BindingPatternKind::BindingIdentifier(ident) =
                        &assignment_pattern.left.kind
                    {
                        if key.name == p.get_binding_identifier_name(ident) {
                            shorthand = true;
                        }
                    }
                }
                _ => {}
            }
        }

        if !shorthand {
            self.key.print(p, ctx);
        }
        if self.computed {
            p.print_ascii_byte(b']');
        }
        if !shorthand {
            p.print_colon();
            p.print_soft_space();
        }
        self.value.print(p, ctx);
    }
}

impl Gen for BindingRestElement<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.add_source_mapping(self.span);
        p.print_ellipsis();
        self.argument.print(p, ctx);
    }
}

impl Gen for ArrayPattern<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.add_source_mapping(self.span);
        p.print_ascii_byte(b'[');
        for (index, item) in self.elements.iter().enumerate() {
            if index != 0 {
                p.print_comma();
                p.print_soft_space();
            }
            if let Some(item) = item {
                item.print(p, ctx);
            }
            if index == self.elements.len() - 1 && (item.is_none() || self.rest.is_some()) {
                p.print_comma();
            }
        }
        if let Some(rest) = &self.rest {
            p.print_soft_space();
            rest.print(p, ctx);
        }
        p.print_ascii_byte(b']');
        p.add_source_mapping_end(self.span);
    }
}

impl Gen for AssignmentPattern<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        self.left.print(p, ctx);
        p.print_soft_space();
        p.print_equal();
        p.print_soft_space();
        self.right.print_expr(p, Precedence::Comma, Context::empty());
    }
}

impl Gen for Decorator<'_> {
    fn r#gen(&self, p: &mut Codegen, _ctx: Context) {
        fn need_wrap(expr: &Expression) -> bool {
            match expr {
                // "@foo"
                // "@foo.bar"
                // "@foo.#bar"
                Expression::Identifier(_)
                | Expression::StaticMemberExpression(_)
                | Expression::PrivateFieldExpression(_) => false,
                Expression::CallExpression(call_expr) => need_wrap(&call_expr.callee),
                // "@(foo + bar)"
                // "@(() => {})"
                // "@(foo['bar'])"
                _ => true,
            }
        }

        p.add_source_mapping(self.span);
        p.print_ascii_byte(b'@');
        let wrap = need_wrap(&self.expression);
        p.wrap(wrap, |p| {
            self.expression.print_expr(p, Precedence::Lowest, Context::empty());
        });
    }
}

impl Gen for TSClassImplements<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        self.expression.print(p, ctx);
        if let Some(type_parameters) = self.type_arguments.as_ref() {
            type_parameters.print(p, ctx);
        }
    }
}

impl Gen for TSTypeParameterDeclaration<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        let is_multi_line = self.params.len() >= 2;
        p.print_ascii_byte(b'<');
        if is_multi_line {
            p.indent();
        }
        for (index, item) in self.params.iter().enumerate() {
            if index != 0 {
                p.print_comma();
            }
            if is_multi_line {
                p.print_soft_newline();
                p.print_indent();
            } else if index != 0 {
                p.print_soft_space();
            }
            item.print(p, ctx);
        }
        if is_multi_line {
            p.print_soft_newline();
            p.dedent();
            p.print_indent();
        } else if p.is_jsx {
            // `<T,>() => {}`
            //    ^ We need a comma here, otherwise it will be regarded as a JSX element.
            p.print_str(",");
        }
        p.print_ascii_byte(b'>');
    }
}

impl Gen for TSTypeAnnotation<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        self.type_annotation.print(p, ctx);
    }
}

impl Gen for TSType<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        let ctx = ctx.with_typescript();
        match self {
            Self::TSFunctionType(ty) => ty.print(p, ctx),
            Self::TSConstructorType(ty) => ty.print(p, ctx),
            Self::TSArrayType(ty) => ty.print(p, ctx),
            Self::TSTupleType(ty) => ty.print(p, ctx),
            Self::TSUnionType(ty) => ty.print(p, ctx),
            Self::TSParenthesizedType(ty) => ty.print(p, ctx),
            Self::TSIntersectionType(ty) => ty.print(p, ctx),
            Self::TSConditionalType(ty) => ty.print(p, ctx),
            Self::TSInferType(ty) => ty.print(p, ctx),
            Self::TSIndexedAccessType(ty) => ty.print(p, ctx),
            Self::TSMappedType(ty) => ty.print(p, ctx),
            Self::TSNamedTupleMember(ty) => ty.print(p, ctx),
            Self::TSLiteralType(ty) => ty.literal.print(p, ctx),
            Self::TSImportType(ty) => ty.print(p, ctx),
            Self::TSAnyKeyword(_) => p.print_str("any"),
            Self::TSBigIntKeyword(_) => p.print_str("bigint"),
            Self::TSBooleanKeyword(_) => p.print_str("boolean"),
            Self::TSIntrinsicKeyword(_) => p.print_str("intrinsic"),
            Self::TSNeverKeyword(_) => p.print_str("never"),
            Self::TSNullKeyword(_) => p.print_str("null"),
            Self::TSNumberKeyword(_) => p.print_str("number"),
            Self::TSObjectKeyword(_) => p.print_str("object"),
            Self::TSStringKeyword(_) => p.print_str("string"),
            Self::TSSymbolKeyword(_) => p.print_str("symbol"),
            Self::TSThisType(_) => p.print_str("this"),
            Self::TSUndefinedKeyword(_) => p.print_str("undefined"),
            Self::TSUnknownKeyword(_) => p.print_str("unknown"),
            Self::TSVoidKeyword(_) => p.print_str("void"),
            Self::TSTemplateLiteralType(ty) => ty.print(p, ctx),
            Self::TSTypeLiteral(ty) => ty.print(p, ctx),
            Self::TSTypeOperatorType(ty) => ty.print(p, ctx),
            Self::TSTypePredicate(ty) => ty.print(p, ctx),
            Self::TSTypeQuery(ty) => ty.print(p, ctx),
            Self::TSTypeReference(ty) => ty.print(p, ctx),
            Self::JSDocNullableType(ty) => ty.print(p, ctx),
            Self::JSDocNonNullableType(ty) => ty.print(p, ctx),
            Self::JSDocUnknownType(_ty) => p.print_str("unknown"),
        }
    }
}

impl Gen for TSArrayType<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        self.element_type.print(p, ctx);
        p.print_str("[]");
    }
}

impl Gen for TSTupleType<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.print_str("[");
        p.print_list(&self.element_types, ctx);
        p.print_str("]");
    }
}

impl Gen for TSUnionType<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        let Some((first, rest)) = self.types.split_first() else {
            return;
        };
        first.print(p, ctx);
        for item in rest {
            p.print_soft_space();
            p.print_str("|");
            p.print_soft_space();
            item.print(p, ctx);
        }
    }
}

impl Gen for TSParenthesizedType<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.print_ascii_byte(b'(');
        self.type_annotation.print(p, ctx);
        p.print_ascii_byte(b')');
    }
}

impl Gen for TSIntersectionType<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        let Some((first, rest)) = self.types.split_first() else {
            return;
        };
        first.print(p, ctx);
        for item in rest {
            p.print_soft_space();
            p.print_str("&");
            p.print_soft_space();
            item.print(p, ctx);
        }
    }
}

impl Gen for TSConditionalType<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        self.check_type.print(p, ctx);
        p.print_str(" extends ");
        self.extends_type.print(p, ctx);
        p.print_str(" ? ");
        self.true_type.print(p, ctx);
        p.print_str(" : ");
        self.false_type.print(p, ctx);
    }
}

impl Gen for TSInferType<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.print_str("infer ");
        self.type_parameter.print(p, ctx);
    }
}

impl Gen for TSIndexedAccessType<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        self.object_type.print(p, ctx);
        p.print_str("[");
        self.index_type.print(p, ctx);
        p.print_str("]");
    }
}

impl Gen for TSMappedType<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.print_str("{");
        p.print_soft_space();
        match self.readonly {
            Some(TSMappedTypeModifierOperator::True) => p.print_str("readonly "),
            Some(TSMappedTypeModifierOperator::Plus) => p.print_str("+readonly "),
            Some(TSMappedTypeModifierOperator::Minus) => p.print_str("-readonly "),
            None => {}
        }
        p.print_str("[");
        self.type_parameter.name.print(p, ctx);
        if let Some(constraint) = &self.type_parameter.constraint {
            p.print_str(" in ");
            constraint.print(p, ctx);
        }
        if let Some(default) = &self.type_parameter.default {
            p.print_str(" = ");
            default.print(p, ctx);
        }
        if let Some(name_type) = &self.name_type {
            p.print_str(" as ");
            name_type.print(p, ctx);
        }
        p.print_str("]");
        match self.optional {
            Some(TSMappedTypeModifierOperator::True) => p.print_str("?"),
            Some(TSMappedTypeModifierOperator::Plus) => p.print_str("+?"),
            Some(TSMappedTypeModifierOperator::Minus) => p.print_str("-?"),
            None => {}
        }
        p.print_soft_space();
        if let Some(type_annotation) = &self.type_annotation {
            p.print_str(":");
            p.print_soft_space();
            type_annotation.print(p, ctx);
        }
        p.print_soft_space();
        p.print_str("}");
    }
}

impl Gen for TSQualifiedName<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        self.left.print(p, ctx);
        p.print_str(".");
        self.right.print(p, ctx);
    }
}

impl Gen for TSTypeOperator<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.print_str(self.operator.to_str());
        p.print_hard_space();
        self.type_annotation.print(p, ctx);
    }
}

impl Gen for TSTypePredicate<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        if self.asserts {
            p.print_str("asserts ");
        }
        match &self.parameter_name {
            TSTypePredicateName::Identifier(ident) => {
                ident.print(p, ctx);
            }
            TSTypePredicateName::This(_ident) => {
                p.print_str("this");
            }
        }
        if let Some(type_annotation) = &self.type_annotation {
            p.print_str(" is ");
            type_annotation.print(p, ctx);
        }
    }
}

impl Gen for TSTypeReference<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        self.type_name.print(p, ctx);
        if let Some(type_parameters) = &self.type_arguments {
            type_parameters.print(p, ctx);
        }
    }
}

impl Gen for JSDocNullableType<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        if self.postfix {
            self.type_annotation.print(p, ctx);
            p.print_str("?");
        } else {
            p.print_str("?");
            self.type_annotation.print(p, ctx);
        }
    }
}

impl Gen for JSDocNonNullableType<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        if self.postfix {
            self.type_annotation.print(p, ctx);
            p.print_str("!");
        } else {
            p.print_str("!");
            self.type_annotation.print(p, ctx);
        }
    }
}

impl Gen for TSTemplateLiteralType<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.print_str("`");
        for (index, item) in self.quasis.iter().enumerate() {
            if index != 0 {
                if let Some(types) = self.types.get(index - 1) {
                    p.print_str("${");
                    types.print(p, ctx);
                    p.print_str("}");
                }
            }
            p.print_str(item.value.raw.as_str());
        }
        p.print_str("`");
    }
}

impl Gen for TSTypeLiteral<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.print_curly_braces(self.span, self.members.is_empty(), |p| {
            for item in &self.members {
                p.print_leading_comments(item.span().start);
                p.print_indent();
                item.print(p, ctx);
                p.print_semicolon();
                p.print_soft_newline();
            }
        });
    }
}

impl Gen for TSTypeName<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        match self {
            Self::IdentifierReference(ident) => {
                ident.print(p, ctx);
            }
            Self::QualifiedName(decl) => {
                decl.left.print(p, ctx);
                p.print_str(".");
                decl.right.print(p, ctx);
            }
            Self::ThisExpression(e) => {
                e.print(p, ctx);
            }
        }
    }
}

impl Gen for TSLiteral<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        match self {
            Self::BooleanLiteral(decl) => decl.print(p, ctx),
            Self::NumericLiteral(decl) => decl.print_expr(p, Precedence::Lowest, ctx),
            Self::BigIntLiteral(decl) => decl.print_expr(p, Precedence::Lowest, ctx),
            Self::StringLiteral(decl) => decl.print(p, ctx),
            Self::TemplateLiteral(decl) => decl.print(p, ctx),
            Self::UnaryExpression(decl) => decl.print_expr(p, Precedence::Comma, ctx),
        }
    }
}

impl Gen for TSTypeParameter<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        if self.r#const {
            p.print_str("const ");
        }
        if self.r#in {
            p.print_str("in ");
        }
        if self.out {
            p.print_str("out ");
        }
        self.name.print(p, ctx);
        if let Some(constraint) = &self.constraint {
            p.print_str(" extends ");
            constraint.print(p, ctx);
        }
        if let Some(default) = &self.default {
            p.print_str(" = ");
            default.print(p, ctx);
        }
    }
}

impl Gen for TSFunctionType<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        if let Some(type_parameters) = &self.type_parameters {
            type_parameters.print(p, ctx);
        }
        p.print_str("(");
        if let Some(this_param) = &self.this_param {
            this_param.print(p, ctx);
            if !self.params.is_empty() || self.params.rest.is_some() {
                p.print_str(",");
                p.print_soft_space();
            }
        }
        self.params.print(p, ctx);
        p.print_str(")");
        p.print_soft_space();
        p.print_str("=>");
        p.print_soft_space();
        self.return_type.print(p, ctx);
    }
}

impl Gen for TSThisParameter<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.print_str("this");
        if let Some(type_annotation) = &self.type_annotation {
            p.print_str(": ");
            type_annotation.print(p, ctx);
        }
    }
}

impl Gen for TSSignature<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        match self {
            Self::TSIndexSignature(signature) => signature.print(p, ctx),
            Self::TSPropertySignature(signature) => signature.r#gen(p, ctx),
            Self::TSCallSignatureDeclaration(signature) => {
                if let Some(type_parameters) = signature.type_parameters.as_ref() {
                    type_parameters.print(p, ctx);
                }
                p.print_str("(");
                if let Some(this_param) = &signature.this_param {
                    this_param.print(p, ctx);
                    if !signature.params.is_empty() || signature.params.rest.is_some() {
                        p.print_str(",");
                        p.print_soft_space();
                    }
                }
                signature.params.print(p, ctx);
                p.print_str(")");
                if let Some(return_type) = &signature.return_type {
                    p.print_colon();
                    p.print_soft_space();
                    return_type.print(p, ctx);
                }
            }
            Self::TSConstructSignatureDeclaration(signature) => {
                p.print_str("new ");
                if let Some(type_parameters) = signature.type_parameters.as_ref() {
                    type_parameters.print(p, ctx);
                }
                p.print_str("(");
                signature.params.print(p, ctx);
                p.print_str(")");
                if let Some(return_type) = &signature.return_type {
                    p.print_colon();
                    p.print_soft_space();
                    return_type.print(p, ctx);
                }
            }
            Self::TSMethodSignature(signature) => {
                match signature.kind {
                    TSMethodSignatureKind::Method => {}
                    TSMethodSignatureKind::Get => p.print_str("get "),
                    TSMethodSignatureKind::Set => p.print_str("set "),
                }
                if signature.computed {
                    p.print_ascii_byte(b'[');
                    signature.key.print(p, ctx);
                    p.print_ascii_byte(b']');
                } else {
                    match &signature.key {
                        PropertyKey::StaticIdentifier(key) => {
                            key.print(p, ctx);
                        }
                        PropertyKey::PrivateIdentifier(key) => {
                            p.print_str(key.name.as_str());
                        }
                        PropertyKey::StringLiteral(key) => {
                            p.print_string_literal(key, false);
                        }
                        key => {
                            key.to_expression().print_expr(p, Precedence::Comma, ctx);
                        }
                    }
                }
                if signature.optional {
                    p.print_str("?");
                }
                if let Some(type_parameters) = &signature.type_parameters {
                    type_parameters.print(p, ctx);
                }
                p.print_str("(");
                if let Some(this_param) = &signature.this_param {
                    this_param.print(p, ctx);
                    if !signature.params.is_empty() || signature.params.rest.is_some() {
                        p.print_str(",");
                        p.print_soft_space();
                    }
                }
                signature.params.print(p, ctx);
                p.print_str(")");
                if let Some(return_type) = &signature.return_type {
                    p.print_colon();
                    p.print_soft_space();
                    return_type.print(p, ctx);
                }
            }
        }
    }
}

impl Gen for TSPropertySignature<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        if self.readonly {
            p.print_str("readonly ");
        }
        if self.computed {
            p.print_ascii_byte(b'[');
            self.key.print(p, ctx);
            p.print_ascii_byte(b']');
        } else {
            match &self.key {
                PropertyKey::StaticIdentifier(key) => {
                    key.print(p, ctx);
                }
                PropertyKey::PrivateIdentifier(key) => {
                    p.print_str(key.name.as_str());
                }
                PropertyKey::StringLiteral(key) => {
                    p.print_string_literal(key, false);
                }
                key => {
                    key.to_expression().print_expr(p, Precedence::Comma, ctx);
                }
            }
        }
        if self.optional {
            p.print_str("?");
        }
        if let Some(type_annotation) = &self.type_annotation {
            p.print_colon();
            p.print_soft_space();
            type_annotation.print(p, ctx);
        }
    }
}

impl Gen for TSTypeQuery<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.print_str("typeof ");
        self.expr_name.print(p, ctx);
        if let Some(type_params) = &self.type_arguments {
            type_params.print(p, ctx);
        }
    }
}

impl Gen for TSTypeQueryExprName<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        match self {
            match_ts_type_name!(Self) => self.to_ts_type_name().print(p, ctx),
            Self::TSImportType(decl) => decl.print(p, ctx),
        }
    }
}

impl Gen for TSImportType<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.print_str("import(");
        self.argument.print(p, ctx);
        if let Some(options) = &self.options {
            p.print_str(", ");
            options.print_expr(p, Precedence::Lowest, ctx);
        }
        p.print_str(")");
        if let Some(qualifier) = &self.qualifier {
            p.print_ascii_byte(b'.');
            qualifier.print(p, ctx);
        }
        if let Some(type_parameters) = &self.type_arguments {
            type_parameters.print(p, ctx);
        }
    }
}

impl Gen for TSTypeParameterInstantiation<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.print_str("<");
        p.print_list(&self.params, ctx);
        p.print_str(">");
    }
}

impl Gen for TSIndexSignature<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        if self.readonly {
            p.print_str("readonly ");
        }
        p.print_str("[");
        for (index, parameter) in self.parameters.iter().enumerate() {
            if index != 0 {
                p.print_str(",");
                p.print_soft_space();
            }
            p.print_str(parameter.name.as_str());
            p.print_colon();
            p.print_soft_space();
            parameter.type_annotation.print(p, ctx);
        }
        p.print_str("]");
        p.print_colon();
        p.print_soft_space();
        self.type_annotation.print(p, ctx);
    }
}

impl Gen for TSTupleElement<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        match self {
            match_ts_type!(TSTupleElement) => self.to_ts_type().print(p, ctx),
            TSTupleElement::TSOptionalType(ts_type) => {
                ts_type.type_annotation.print(p, ctx);
                p.print_str("?");
            }
            TSTupleElement::TSRestType(ts_type) => {
                p.print_str("...");
                ts_type.type_annotation.print(p, ctx);
            }
        }
    }
}

impl Gen for TSNamedTupleMember<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        self.label.print(p, ctx);
        if self.optional {
            p.print_str("?");
        }
        p.print_str(":");
        p.print_soft_space();
        self.element_type.print(p, ctx);
    }
}

impl Gen for TSModuleDeclaration<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        if self.declare {
            p.print_str("declare ");
        }
        p.print_str(self.kind.as_str());
        // If the kind is global, then the id is also `global`, so we don't need to print it
        if !self.kind.is_global() {
            p.print_space_before_identifier();
            self.id.print(p, ctx);
        }

        if let Some(body) = &self.body {
            let mut body = body;
            loop {
                match body {
                    TSModuleDeclarationBody::TSModuleDeclaration(b) => {
                        p.print_ascii_byte(b'.');
                        b.id.print(p, ctx);
                        if let Some(b) = &b.body {
                            body = b;
                        } else {
                            break;
                        }
                    }
                    TSModuleDeclarationBody::TSModuleBlock(body) => {
                        p.print_soft_space();
                        body.print(p, ctx);
                        break;
                    }
                }
            }
        } else {
            p.print_semicolon();
        }
        p.needs_semicolon = false;
    }
}

impl Gen for TSModuleDeclarationName<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        match self {
            Self::Identifier(ident) => ident.print(p, ctx),
            Self::StringLiteral(s) => p.print_string_literal(s, false),
        }
    }
}

impl Gen for TSModuleBlock<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        let is_empty = self.directives.is_empty() && self.body.is_empty();
        p.print_curly_braces(self.span, is_empty, |p| {
            p.print_directives_and_statements(&self.directives, &self.body, ctx);
        });
        p.needs_semicolon = false;
    }
}

impl Gen for TSTypeAliasDeclaration<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        if self.declare {
            p.print_str("declare ");
        }
        p.print_str("type");
        p.print_space_before_identifier();
        self.id.print(p, ctx);
        if let Some(type_parameters) = &self.type_parameters {
            type_parameters.print(p, ctx);
        }
        p.print_soft_space();
        p.print_str("=");
        p.print_soft_space();
        self.type_annotation.print(p, ctx);
    }
}

impl Gen for TSInterfaceDeclaration<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.print_str("interface");
        p.print_hard_space();
        self.id.print(p, ctx);
        if let Some(type_parameters) = &self.type_parameters {
            type_parameters.print(p, ctx);
        }
        if !self.extends.is_empty() {
            p.print_str(" extends ");
            p.print_list(&self.extends, ctx);
        }
        p.print_soft_space();
        p.print_curly_braces(self.body.span, self.body.body.is_empty(), |p| {
            for item in &self.body.body {
                p.print_leading_comments(item.span().start);
                p.print_indent();
                item.print(p, ctx);
                p.print_semicolon();
                p.print_soft_newline();
            }
        });
    }
}

impl Gen for TSInterfaceHeritage<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        self.expression.print_expr(p, Precedence::Call, ctx);
        if let Some(type_parameters) = &self.type_arguments {
            type_parameters.print(p, ctx);
        }
    }
}

impl Gen for TSEnumDeclaration<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.print_indent();
        if self.declare {
            p.print_str("declare ");
        }
        if self.r#const {
            p.print_str("const ");
        }
        p.print_space_before_identifier();
        p.print_str("enum ");
        self.id.print(p, ctx);
        p.print_space_before_identifier();
        self.body.print(p, ctx);
    }
}

impl Gen for TSEnumBody<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.print_curly_braces(self.span, self.members.is_empty(), |p| {
            for (index, member) in self.members.iter().enumerate() {
                p.print_leading_comments(member.span().start);
                p.print_indent();
                member.print(p, ctx);
                if index != self.members.len() - 1 {
                    p.print_comma();
                }
                p.print_soft_newline();
            }
        });
    }
}

impl Gen for TSEnumMember<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        match &self.id {
            TSEnumMemberName::Identifier(decl) => decl.print(p, ctx),
            TSEnumMemberName::String(decl) => p.print_string_literal(decl, false),
            TSEnumMemberName::ComputedString(decl) => {
                p.print_ascii_byte(b'[');
                p.print_string_literal(decl, false);
                p.print_ascii_byte(b']');
            }
            TSEnumMemberName::ComputedTemplateString(decl) => {
                let quasi = decl.quasis.first().unwrap();
                p.add_source_mapping(quasi.span);

                p.print_str("[`");
                p.print_str(quasi.value.raw.as_str());
                p.print_str("`]");
            }
        }

        if let Some(init) = &self.initializer {
            p.print_soft_space();
            p.print_equal();
            p.print_soft_space();
            init.print_expr(p, Precedence::Lowest, ctx);
        }
    }
}

impl Gen for TSConstructorType<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        if self.r#abstract {
            p.print_str("abstract ");
        }
        p.print_str("new ");
        if let Some(type_parameters) = &self.type_parameters {
            type_parameters.print(p, ctx);
        }
        p.print_str("(");
        self.params.print(p, ctx);
        p.print_str(")");
        p.print_soft_space();
        p.print_str("=>");
        p.print_soft_space();
        self.return_type.print(p, ctx);
    }
}

impl Gen for TSImportEqualsDeclaration<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        p.print_str("import ");
        self.id.print(p, ctx);
        p.print_str(" = ");
        self.module_reference.print(p, ctx);
    }
}

impl Gen for TSModuleReference<'_> {
    fn r#gen(&self, p: &mut Codegen, ctx: Context) {
        match self {
            Self::ExternalModuleReference(decl) => {
                p.print_str("require(");
                p.print_string_literal(&decl.expression, false);
                p.print_str(")");
            }
            match_ts_type_name!(Self) => self.to_ts_type_name().print(p, ctx),
        }
    }
}

impl GenExpr for V8IntrinsicExpression<'_> {
    fn gen_expr(&self, p: &mut Codegen, precedence: Precedence, ctx: Context) {
        let is_statement = p.start_of_stmt == p.code_len();
        let is_export_default = p.start_of_default_export == p.code_len();
        let mut wrap = precedence >= Precedence::New || ctx.intersects(Context::FORBID_CALL);
        if precedence >= Precedence::Postfix {
            wrap = true;
        }

        p.wrap(wrap, |p| {
            if is_export_default {
                p.start_of_default_export = p.code_len();
            } else if is_statement {
                p.start_of_stmt = p.code_len();
            }
            p.add_source_mapping(self.span);
            p.print_ascii_byte(b'%');
            self.name.print(p, Context::empty());
            p.print_arguments(self.span, &self.arguments, ctx);
            p.add_source_mapping_end(self.span);
        });
    }
}
