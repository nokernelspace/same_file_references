// Copyright 2025 Andros Yang
// All Rights Reserved

use std::boxed::Box;
use std::fs;
use syn::Item;
use syn::*;

fn main() {
    let src = fs::read_to_string("/Users/lyoko/Projects/builder/src/utils.rs").unwrap();
    let ast = syn::parse_file(&src).unwrap();
    let shebang = ast.shebang;
    for attr in ast.attrs {}

    //https://docs.rs/syn/2.0.98/syn/enum.Item.html
    for item in ast.items {
        match item {
            // Data Structures
            Item::Const(ItemConst {
                attrs,
                vis,
                const_token,
                ident,
                generics,
                colon_token,
                ty,
                eq_token,
                expr,
                semi_token,
            }) => {}
            Item::Enum(ItemEnum {
                attrs,
                vis,
                enum_token,
                ident,
                generics,
                brace_token,
                variants,
            }) => {}
            Item::Struct(ItemStruct {
                attrs,
                vis,
                struct_token,
                ident,
                generics,
                fields,
                semi_token,
            }) => {}
            // Function Modules
            Item::Impl(ItemImpl {
                attrs,
                defaultness,
                unsafety,
                impl_token,
                generics,
                trait_,
                self_ty,
                brace_token,
                items,
            }) => {}
            Item::Mod(ItemMod {
                attrs,
                vis,
                unsafety,
                mod_token,
                ident,
                content,
                semi,
            }) => {}
            Item::Use(ItemUse {
                attrs,
                vis,
                use_token,
                leading_colon,
                tree,
                semi_token,
            }) => {}
            Item::Fn(ItemFn {
                attrs,
                vis,
                sig,
                block,
            }) => {
                let mut inner_file_refs = 0;
                for cmd in block.stmts {
                    match cmd {
                        Stmt::Item(item) => parse_item(item),
                        Stmt::Expr(expr, _) => {
                            if is_same_file_function_call(expr) {
                                inner_file_refs += 1;
                            }
                        }
                        Stmt::Macro(_) => {}
                        Stmt::Local(local) => parse_local(local),
                    }
                }
                println!("fn {} => {}", sig.ident.to_string(), inner_file_refs);
            }
            _ => {}
        }
    }
}

fn parse_item(item: Item) {
    match item {
        Item::Const(_const) => println!("{:?}", _const),
        Item::Enum(_enum) => println!("{:?}", _enum),
        Item::ExternCrate(_crate) => {
            println!("{:?}", _crate)
        }
        Item::Fn(_fn) => {
            println!("{:?}", _fn)
        }
        Item::ForeignMod(_mod) => {
            println!("{:?}", _mod)
        }
        Item::Impl(_impl) => {
            println!("{:?}", _impl)
        }
        Item::Macro(_macro) => {
            println!("{:?}", _macro)
        }
        Item::Mod(_mod) => println!("{:?}", _mod),
        Item::Static(_static) => {}
        Item::Struct(_sruct) => {}
        Item::Trait(_trait) => {}
        Item::TraitAlias(_trait) => {}
        Item::Type(_type) => {}
        Item::Union(_union) => {}
        Item::Use(_use) => {}
        Item::Verbatim(toks) => {
            println!("{:?}", toks);
        }
        _ => {}
    }
}

fn parse_local(local: Local) {
    //println!("{:?}", local);
}

fn is_same_file_function_call(expr: Expr) -> bool {
    match expr {
        Expr::Call(_call) => {
            return true;
        }

        _ => false,
    }
}
