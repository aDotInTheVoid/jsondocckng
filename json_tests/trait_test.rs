use std::collections::HashSet;

use rustdoc_types::{FnDecl, Generics, Method, Trait, Type};

use crate::{from_item::IntoKind, TCrate};

pub(super) fn test(k: TCrate) {
    let whamer = k.load_root::<Trait>("Whammer");
    assert_eq!(whamer.items.len(), 1);
    let wham = &whamer.items[0];
    let wham: &Method = k.krate.index[wham].into_kind();
    assert_eq!(
        wham,
        &Method {
            decl: FnDecl {
                inputs: vec![(
                    "self".to_owned(),
                    Type::BorrowedRef {
                        lifetime: None,
                        mutable: false,
                        type_: Box::new(Type::Generic("Self".to_owned()))
                    }
                )],
                output: None,
                c_variadic: false,
            },
            abi: "\"Rust\"".to_owned(),
            has_body: false,
            generics: Generics {
                params: vec![],
                where_predicates: vec![]
            },
            header: HashSet::new(),
        }
    )
}
