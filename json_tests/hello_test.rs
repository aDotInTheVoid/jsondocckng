use rustdoc_types::*;

use crate::from_item::IntoKind;

pub(super) fn test(h: Crate) {
    let k = crate::TCrate { krate: h };

    assert_eq!(k.root_item().name.as_ref().unwrap(), "hello");
    assert_eq!(
        k.root_item().docs.as_ref().unwrap(),
        "A crate that can print frendly greetings"
    );

    assert_eq!(k.root().items.len(), 1);

    let hello_fun = k.load_root_item("hello");
    assert_eq!(
        hello_fun.docs.as_ref().unwrap(),
        "Display a frendly greeting"
    );

    assert_eq!(
        hello_fun.into_kind::<Function>().decl,
        FnDecl {
            inputs: vec![],
            output: None,
            c_variadic: false
        }
    );
}
