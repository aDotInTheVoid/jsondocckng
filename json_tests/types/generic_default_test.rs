use std::vec;

use guard::guard_unwrap;
use rustdoc_types::*;

pub(super) fn test(k: Crate) {
    let root = &k.index[&k.root];
    guard_unwrap!(let ItemEnum::Module(root) = &root.inner);

    let result = crate::load_by_name(&k, root, "Result");
    let myerror = crate::load_by_name(&k, root, "MyError");
    let myresult = crate::load_by_name(&k, root, "MyResult");

    guard_unwrap!(let ItemEnum::Typedef(myresult) = &myresult.inner);

    assert_eq!(
        myresult.generics,
        Generics {
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
                            id: myerror.id.clone(),
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
        }
    );
    assert_eq!(
        myresult.type_,
        Type::ResolvedPath {
            name: "Result".to_owned(),
            id: result.id.clone(),
            args: Some(Box::new(GenericArgs::AngleBracketed {
                args: vec![
                    GenericArg::Type(Type::Generic("T".to_owned())),
                    GenericArg::Type(Type::Generic("E".to_owned()))
                ],
                bindings: vec![]
            })),
            param_names: vec![]
        }
    )
}
