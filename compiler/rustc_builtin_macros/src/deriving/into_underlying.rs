use rustc_ast::ast::DUMMY_NODE_ID;
use rustc_ast::ptr::P;
use rustc_ast::{
    AssocItem, AssocItemKind, AttrVec, Const, Defaultness, FnHeader, FnRetTy, FnSig, Generics,
    ImplPolarity, ItemKind, MetaItem, Mutability, Param, Path, PathSegment, SelfKind, Unsafe,
    Visibility, VisibilityKind,
};
use rustc_errors::struct_span_err;
use rustc_expand::base::{Annotatable, ExtCtxt};
use rustc_span::source_map::dummy_spanned;
use rustc_span::symbol::{kw::SelfLower, sym, Ident};
use rustc_span::{Span, DUMMY_SP};

macro_rules! invalid_derive {
    ($cx:ident, $span:ident) => {
        struct_span_err!(
            &$cx.sess.parse_sess.span_diagnostic,
            $span,
            FIXME,
            "`IntoUnderlying` can only be derived for enums with an explicit integer representation"
        )
        .emit();
    };
}

pub fn expand_deriving_into_underlying(
    cx: &mut ExtCtxt<'_>,
    span: Span,
    _mitem: &MetaItem,
    item: &Annotatable,
    push: &mut dyn FnMut(Annotatable),
) {
    match *item {
        Annotatable::Item(ref annitem) => match annitem.kind {
            ItemKind::Enum(_, _) => {
                let reprs: Vec<_> = annitem
                    .attrs
                    .iter()
                    .filter_map(|attr| {
                        for r in rustc_attr::find_repr_attrs(&cx.sess, attr) {
                            use rustc_attr::*;
                            match r {
                                ReprInt(rustc_attr::IntType::UnsignedInt(int_type)) => {
                                    return Some(int_type.name());
                                }
                                ReprInt(rustc_attr::IntType::SignedInt(int_type)) => {
                                    return Some(int_type.name());
                                }
                                ReprC | ReprPacked(..) | ReprSimd | ReprTransparent
                                | ReprAlign(..) | ReprNoNiche => {}
                            }
                        }
                        None
                    })
                    .collect();
                if reprs.len() != 1 {
                    invalid_derive!(cx, span);
                    return;
                }

                let repr_ident = Ident { name: reprs[0], span: DUMMY_SP };
                let repr_ty = cx.ty_ident(DUMMY_SP, repr_ident);

                let ty = cx.ty_ident(DUMMY_SP, annitem.ident);

                let self_ident = Ident::with_dummy_span(SelfLower).with_span_pos(DUMMY_SP);

                let decl = cx.fn_decl(
                    vec![Param::from_self(
                        AttrVec::default(),
                        dummy_spanned(SelfKind::Value(Mutability::Not)),
                        self_ident,
                    )],
                    FnRetTy::Ty(repr_ty.clone()),
                );

                let fn_item = P(AssocItem {
                    attrs: vec![cx.attribute(cx.meta_word(span, sym::inline))],
                    id: DUMMY_NODE_ID,
                    span: DUMMY_SP,
                    vis: Visibility { kind: VisibilityKind::Inherited, span, tokens: None },
                    ident: Ident::from_str("into_underlying"),

                    kind: AssocItemKind::Fn(
                        Defaultness::Final,
                        FnSig { header: FnHeader::default(), decl, span: DUMMY_SP },
                        Generics::default(),
                        Some(cx.block_expr(cx.expr_cast(
                            DUMMY_SP,
                            cx.expr_path(Path::from_ident(self_ident)),
                            repr_ty.clone(),
                        ))),
                    ),
                    tokens: None,
                });

                let mut trait_path = Path {
                    span: DUMMY_SP,
                    segments: cx
                        .std_path(&[sym::convert])
                        .into_iter()
                        .map(PathSegment::from_ident)
                        .collect(),
                    tokens: None,
                };
                trait_path.segments.push(PathSegment {
                    ident: Ident { name: sym::IntoUnderlying, span: DUMMY_SP },
                    id: DUMMY_NODE_ID,
                    args: None,
                });

                let associated_type = P(AssocItem {
                    attrs: vec![],
                    id: DUMMY_NODE_ID,
                    span: DUMMY_SP,
                    vis: Visibility { kind: VisibilityKind::Inherited, span, tokens: None },
                    ident: Ident::from_str("Underlying"),
                    kind: AssocItemKind::TyAlias(
                        Defaultness::Final,
                        Generics::default(),
                        vec![],
                        Some(repr_ty),
                    ),
                    tokens: None,
                });

                let trait_item = Annotatable::Item(cx.item(
                    DUMMY_SP,
                    Ident::invalid(),
                    Vec::new(),
                    ItemKind::Impl {
                        unsafety: Unsafe::No,
                        polarity: ImplPolarity::Positive,
                        defaultness: Defaultness::Final,
                        constness: Const::No,
                        generics: Generics::default(),
                        of_trait: Some(cx.trait_ref(trait_path)),
                        self_ty: ty,
                        items: vec![associated_type, fn_item],
                    },
                ));

                push(trait_item);
            }
            _ => invalid_derive!(cx, span),
        },
        _ => invalid_derive!(cx, span),
    }
}
