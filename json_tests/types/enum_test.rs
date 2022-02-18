use rustdoc_types::*;

use crate::TCrate;

pub(super) fn test(k: TCrate) {
    k.load_root::<Enum>("Foo");
}
