use rustdoc_types::*;

pub(super) fn test(k: Crate) {
    let k = crate::TCrate { krate: k };

    k.load_root::<Enum>("Foo");
}
