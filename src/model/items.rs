use std::fmt::{self, Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::num::NonZeroUsize;

use comemo::Tracked;
use once_cell::sync::OnceCell;

use super::{Content, NodeId, StyleChain};
use crate::diag::SourceResult;
use crate::frame::Frame;
use crate::geom::{Abs, Dir};
use crate::util::{hash128, EcoString};
use crate::World;

/// Global storage for lang items.
#[doc(hidden)]
pub static LANG_ITEMS: OnceCell<LangItems> = OnceCell::new();

/// Set the lang items. This is a hack :(
///
/// Passing the lang items everywhere they are needed (especially the text node
/// related things) is very painful. By storing them globally, in theory, we
/// break incremental, but only when different sets of lang items are used in
/// the same program. For this reason, if this function is called multiple
/// times, the items must be the same.
pub(crate) fn set_lang_items(items: LangItems) {
    if LANG_ITEMS.set(items).is_err() {
        let first = hash128(LANG_ITEMS.get().unwrap());
        let second = hash128(&items);
        assert_eq!(first, second, "set differing lang items");
    }
}

/// Access a lang item.
macro_rules! item {
    ($name:ident) => {
        $crate::model::LANG_ITEMS.get().unwrap().$name
    };
}

/// Definition of certain standard library items the language is aware of.
#[derive(Copy, Clone)]
pub struct LangItems {
    /// The root layout function.
    pub layout: fn(
        content: &Content,
        world: Tracked<dyn World>,
        styles: StyleChain,
    ) -> SourceResult<Vec<Frame>>,
    /// Access the em size.
    pub em: fn(StyleChain) -> Abs,
    /// Access the text direction.
    pub dir: fn(StyleChain) -> Dir,
    /// Whitespace.
    pub space: fn() -> Content,
    /// A forced line break: `\`.
    pub linebreak: fn(justify: bool) -> Content,
    /// Plain text without markup.
    pub text: fn(text: EcoString) -> Content,
    /// The id of the text node.
    pub text_id: NodeId,
    /// Get the string if this is a text node.
    pub text_str: fn(&Content) -> Option<&str>,
    /// A smart quote: `'` or `"`.
    pub smart_quote: fn(double: bool) -> Content,
    /// A paragraph break.
    pub parbreak: fn() -> Content,
    /// Strong content: `*Strong*`.
    pub strong: fn(body: Content) -> Content,
    /// Emphasized content: `_Emphasized_`.
    pub emph: fn(body: Content) -> Content,
    /// Raw text with optional syntax highlighting: `` `...` ``.
    pub raw: fn(text: EcoString, tag: Option<EcoString>, block: bool) -> Content,
    /// A hyperlink: `https://typst.org`.
    pub link: fn(url: EcoString) -> Content,
    /// A reference: `@target`.
    pub ref_: fn(target: EcoString) -> Content,
    /// A section heading: `= Introduction`.
    pub heading: fn(level: NonZeroUsize, body: Content) -> Content,
    /// An item in an unordered list: `- ...`.
    pub list_item: fn(body: Content) -> Content,
    /// An item in an enumeration (ordered list): `+ ...` or `1. ...`.
    pub enum_item: fn(number: Option<usize>, body: Content) -> Content,
    /// An item in a description list: `/ Term: Details`.
    pub desc_item: fn(term: Content, body: Content) -> Content,
    /// A mathematical formula: `$x$`, `$ x^2 $`.
    pub math: fn(children: Vec<Content>, display: bool) -> Content,
    /// An atom in a formula: `x`, `+`, `12`.
    pub math_atom: fn(atom: EcoString) -> Content,
    /// A base with optional sub- and superscripts in a formula: `a_1^2`.
    pub math_script:
        fn(base: Content, sub: Option<Content>, sup: Option<Content>) -> Content,
    /// A fraction in a formula: `x/2`.
    pub math_frac: fn(num: Content, denom: Content) -> Content,
    /// An alignment indicator in a formula: `&`, `&&`.
    pub math_align: fn(count: usize) -> Content,
}

impl Debug for LangItems {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.pad("LangItems { .. }")
    }
}

impl Hash for LangItems {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.layout as usize).hash(state);
        (self.em as usize).hash(state);
        (self.dir as usize).hash(state);
        self.space.hash(state);
        self.linebreak.hash(state);
        self.text.hash(state);
        self.smart_quote.hash(state);
        self.parbreak.hash(state);
        self.strong.hash(state);
        self.emph.hash(state);
        self.raw.hash(state);
        self.link.hash(state);
        self.ref_.hash(state);
        self.heading.hash(state);
        self.list_item.hash(state);
        self.enum_item.hash(state);
        self.desc_item.hash(state);
        self.math.hash(state);
        self.math_atom.hash(state);
        self.math_script.hash(state);
        self.math_frac.hash(state);
        self.math_align.hash(state);
    }
}
