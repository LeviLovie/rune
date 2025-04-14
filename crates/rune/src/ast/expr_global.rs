use crate::ast::prelude::*;

#[test]
#[cfg(not(miri))]
fn ast_parse() {
    rt::<ast::ExprGlobal>("let x = 1");
    rt::<ast::ExprGlobal>("#[attr] let a = f()");
}

/// A global expression.
///
/// * `global <name> = <expr>`
#[derive(Debug, TryClone, PartialEq, Eq, ToTokens, Spanned)]
#[non_exhaustive]
pub struct ExprGlobal {
    /// The attributes for the let expression
    #[rune(iter)]
    pub attributes: Vec<ast::Attribute>,
    /// The `global` token.
    pub global_token: T![global],
    /// The `mut` token.
    #[rune(iter)]
    pub mut_token: Option<T![mut]>,
    /// The name of the binding.
    pub pat: ast::Pat,
    /// The equality token.
    pub eq: T![=],
    /// The expression the binding is assigned to.
    pub expr: Box<ast::Expr>,
}

impl ExprGlobal {
    /// Parse with the given meta.
    pub(crate) fn parse_with_meta(
        parser: &mut Parser<'_>,
        attributes: Vec<ast::Attribute>,
    ) -> Result<Self> {
        Ok(Self {
            attributes,
            global_token: parser.parse()?,
            mut_token: parser.parse()?,
            pat: parser.parse()?,
            eq: parser.parse()?,
            expr: Box::try_new(ast::Expr::parse_without_eager_brace(parser)?)?,
        })
    }

    /// Parse a let expression without eager bracing.
    pub(crate) fn parse_without_eager_brace(parser: &mut Parser<'_>) -> Result<Self> {
        Ok(Self {
            attributes: Vec::new(),
            global_token: parser.parse()?,
            mut_token: parser.parse()?,
            pat: parser.parse()?,
            eq: parser.parse()?,
            expr: Box::try_new(ast::Expr::parse_without_eager_brace(parser)?)?,
        })
    }
}

expr_parse!(Global, ExprGlobal, "global expression");
