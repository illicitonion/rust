// use crate::deriving::generic::ty::*;
// use crate::deriving::generic::*;
// use crate::deriving::path_std;

// use rustc_ast::ptr::P;
// use rustc_ast::{self as ast, Expr, GenericArg, Generics, ItemKind, MetaItem, VariantData};
use rustc_ast::{ItemKind, MetaItem};
use rustc_expand::base::{Annotatable, ExtCtxt};
// use rustc_span::symbol::{kw, sym, Ident, Symbol};
use rustc_span::Span;

pub fn expand_deriving_into(
    cx: &mut ExtCtxt<'_>,
    span: Span,
    _mitem: &MetaItem,
    item: &Annotatable,
    _push: &mut dyn FnMut(Annotatable),
) {
    match *item {
        Annotatable::Item(ref annitem) => match annitem.kind {
            ItemKind::Enum(_, _) => {
                cx.span_bug(span, "DWH: enum")
            },
            _ => {
                cx.span_bug(span, "DWH: not enum")
            },
        },
        _ => {
            cx.span_bug(span, "DWH: not Item")
        },
    }

    // let inline = cx.meta_word(span, sym::inline);
    // let attrs = vec![cx.attribute(inline)];
    // let trait_def = TraitDef {
    //     span,
    //     attributes: Vec::new(),
    //     path: path_std!(into::Into),
    //     additional_bounds: Bounds::empty(),
    //     generics: Bounds::empty(),
    //     is_unsafe: false,
    //     supports_unions: false,
    //     methods: vec![MethodDef {
    //         name: sym::into,
    //         generics: Bounds::empty(),
    //         explicit_self: borrowed_explicit_self(),
    //         args: Vec::new(),
    //         ret_ty: repr_ty,
    //         attributes: attrs,
    //         is_unsafe: false,
    //         unify_fieldless_variants: false,
    //         combine_substructure: substructure,
    //     }],
    //     associated_types: Vec::new(),
    // };
    //
    // trait_def.expand_ext(cx, mitem, item, push, is_shallow)
}