use std::vec;

use rustdoc_types::*;

pub(super) fn test(krate: Crate) {
    let k = crate::TCrate { krate };

    let my_result: &Typedef = k.load_root("MyResult");

    assert_eq!(
        my_result,
        &Typedef {
            generics: Generics {
                params: vec![
                    GenericParamDef {
                        name: "T".to_owned(),
                        kind: GenericParamDefKind::Type {
                            bounds: vec![],
                            default: None
                        },
                    },
                    GenericParamDef {
                        name: "E".to_owned(),
                        kind: GenericParamDefKind::Type {
                            bounds: vec![],
                            default: Some(Type::ResolvedPath {
                                name: "MyError".to_owned(),
                                id: k.load_root_id("MyError"),
                                args: Some(Box::new(GenericArgs::AngleBracketed {
                                    args: vec![],
                                    bindings: vec![]
                                })),
                                param_names: vec![]
                            })
                        }
                    }
                ],
                where_predicates: vec![],
            },
            type_: Type::ResolvedPath {
                name: "Result".to_owned(),
                id: k.load_root_id("Result"),
                args: Some(Box::new(GenericArgs::AngleBracketed {
                    args: vec![
                        GenericArg::Type(Type::Generic("T".to_owned())),
                        GenericArg::Type(Type::Generic("E".to_owned()))
                    ],
                    bindings: vec![]
                })),
                param_names: vec![]
            }
        }
    );
}
