use rustdoc_types::*;

use crate::TCrate;

pub(super) fn test(k: TCrate) {
    let hidden: &Struct = k.load_root("HiddenFields");
    assert_eq!(hidden.fields, vec![]);
    assert_eq!(hidden.fields_stripped, true);

    let no_fields: &Struct = k.load_root("NoFields");
    assert_eq!(no_fields.fields, vec![]);
    assert_eq!(no_fields.fields_stripped, false);
}
