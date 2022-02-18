use guard::guard_unwrap;
use rustdoc_types::*;

pub(super) fn test(c: Crate) {
    let root = &c.index[&c.root];
    guard_unwrap!(let ItemEnum::Module(root) = &root.inner);

    let hidden = crate::load_by_name(&c, &root, "HiddenFields");
    let no_fields = crate::load_by_name(&c, &root, "NoFields");
    guard_unwrap!(let ItemEnum::Struct(hidden) = &hidden.inner);
    guard_unwrap!(let ItemEnum::Struct(no_fields) = &no_fields.inner);

    assert_eq!(hidden.fields.len(), 0);
    assert_eq!(no_fields.fields.len(), 0);
    assert_eq!(hidden.fields_stripped, true);
    assert_eq!(no_fields.fields_stripped, false);
}
