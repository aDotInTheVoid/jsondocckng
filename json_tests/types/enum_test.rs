use guard::guard_unwrap;
use rustdoc_types::*;

pub(super) fn test(k: Crate) {
    let root = &k.index[&k.root];
    guard_unwrap!(let ItemEnum::Module(root) = &root.inner);
    guard_unwrap!(let [eid] = &root.items[..]);
    let eitem = &k.index[&eid];
    assert_eq!(eitem.name.as_ref().unwrap(), "Foo");
    guard_unwrap!(let ItemEnum::Enum(_) = &eitem.inner);
}
