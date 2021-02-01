use rustc_ast::ast::DUMMY_NODE_ID;
use rustc_ast::ptr::P;
use rustc_ast::{
    AngleBracketedArg, AngleBracketedArgs, AssocItem, AssocItemKind, Const, Defaultness, FnHeader,
    FnRetTy, FnSig, GenericArg, GenericArgs, Generics, ImplPolarity, ItemKind, MetaItem, Path,
    PathSegment, Unsafe, VariantData, Visibility, VisibilityKind,
};
use rustc_errors::struct_span_err;
use rustc_expand::base::{Annotatable, ExtCtxt};
use rustc_span::symbol::{sym, Ident};
use rustc_span::{Span, DUMMY_SP};

macro_rules! invalid_derive {
    ($cx:ident, $span:ident, $reason:expr) => {
        struct_span_err!(
            &$cx.sess.parse_sess.span_diagnostic,
            $span,
            FIXME,
            "`Into` can only be derived for {}",
            $reason
        )
        .emit()
    };
}

pub fn expand_deriving_into(
    cx: &mut ExtCtxt<'_>,
    span: Span,
    _mitem: &MetaItem,
    item: &Annotatable,
    push: &mut dyn FnMut(Annotatable),
) {
    match *item {
        Annotatable::Item(ref annitem) => match annitem.kind {
            ItemKind::Enum(ref def, _) => {
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
                    invalid_derive!(cx, span, "enums with an explicit integer representation");
                    return;
                }

                let has_data = def.variants.iter().any(|variant| match variant.data {
                    VariantData::Struct(..) | VariantData::Tuple(..) => true,
                    VariantData::Unit(..) => false,
                });

                if has_data {
                    invalid_derive!(cx, span, "data-free enums");
                    return;
                }

                let repr_ident = Ident { name: reprs[0], span: DUMMY_SP };
                let repr_ty = cx.ty_ident(DUMMY_SP, repr_ident);

                let ty = cx.ty_ident(DUMMY_SP, annitem.ident);

                let param_ident = Ident::from_str("value");

                let decl = cx.fn_decl(
                    vec![cx.param(DUMMY_SP, param_ident, ty)],
                    FnRetTy::Ty(repr_ty.clone()),
                );

                let fn_item = P(AssocItem {
                    attrs: vec![cx.attribute(cx.meta_word(span, sym::inline))],
                    id: DUMMY_NODE_ID,
                    span: DUMMY_SP,
                    vis: Visibility { kind: VisibilityKind::Inherited, span, tokens: None },
                    ident: Ident::from_str("from"),

                    kind: AssocItemKind::Fn(
                        Defaultness::Final,
                        FnSig { header: FnHeader::default(), decl, span: DUMMY_SP },
                        Generics::default(),
                        Some(cx.block_expr(cx.expr_cast(
                            DUMMY_SP,
                            cx.expr_path(Path::from_ident(param_ident)),
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
                    ident: Ident { name: sym::From, span: DUMMY_SP },
                    id: DUMMY_NODE_ID,
                    args: Some(P(GenericArgs::AngleBracketed(AngleBracketedArgs {
                        span: DUMMY_SP,
                        args: vec![AngleBracketedArg::Arg(GenericArg::Type(
                            cx.ty_ident(DUMMY_SP, annitem.ident),
                        ))],
                    }))),
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
                        self_ty: repr_ty,
                        items: vec![fn_item],
                    },
                ));

                push(trait_item);
            }
            _ => invalid_derive!(cx, span, "enums"),
        },
        _ => invalid_derive!(cx, span, "enums"),
    }
}
