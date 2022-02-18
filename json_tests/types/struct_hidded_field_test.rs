use rustdoc_types::*;

pub(super) fn test(c: Crate) {
    let k = crate::TCrate { krate: c };

    let hidden: &Struct = k.load_root("HiddenFields");
    assert_eq!(hidden.fields, vec![]);
    assert_eq!(hidden.fields_stripped, true);

    let no_fields: &Struct = k.load_root("NoFields");
    assert_eq!(no_fields.fields, vec![]);
    assert_eq!(no_fields.fields_stripped, false);
}
