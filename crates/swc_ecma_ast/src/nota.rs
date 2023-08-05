use is_macro::Is;
use swc_atoms::JsWord;
use swc_common::{ast_node, EqIgnoreSpan, Span};

#[ast_node("NotaTemplate")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct NotaTemplate {
    pub span: Span,
    pub elems: NotaElems,
}

/// An expression in markup, math or code.
#[ast_node]
#[derive(Eq, Hash, Is, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub enum NotaElem {
    /// Plain text without markup.
    #[tag("TextElem")]
    Text(NotaText),

    /// Whitespace in markup or math. Has at most one newline in markup, as more
    /// indicate a paragraph break.
    #[tag("SpaceElem")]
    Space(NotaSpace),

    /// A paragraph break, indicated by one or multiple blank lines.
    #[tag("ParbreakElem")]
    Parbreak(NotaParbreak),

    /// Strong content: `*Strong*`.
    #[tag("StrongElem")]
    Strong(NotaStrong),

    /// An item in a bullet list: `- ...`.
    #[tag("ListElem")]
    List(NotaListItem),
}

pub type NotaElems = Vec<NotaElem>;

#[ast_node("TextElem")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct NotaText {
    pub span: Span,

    #[cfg_attr(any(feature = "rkyv-impl"), with(swc_atoms::EncodeJsWord))]
    pub value: JsWord,
}

#[ast_node("SpaceElem")]
#[derive(Eq, Hash, Copy, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct NotaSpace {
    pub span: Span,
}

#[ast_node("ParbreakElem")]
#[derive(Eq, Hash, Copy, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct NotaParbreak {
    pub span: Span,
}

#[ast_node("StrongElem")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct NotaStrong {
    pub span: Span,

    pub elems: NotaElems,
}

#[ast_node("ListElem")]
#[derive(Eq, Hash, EqIgnoreSpan)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
pub struct NotaListItem {
    pub span: Span,

    pub elems: NotaElems,
}
