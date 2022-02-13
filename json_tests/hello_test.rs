use guard::guard_unwrap;
use rustdoc_types::*;

pub(super) fn test(h: Crate) {
    let hello = &h.index[&h.root];

    assert_eq!(hello.name.as_ref().unwrap(), "hello");
    assert_eq!(
        hello.docs.as_ref().unwrap(),
        "A crate that can print frendly greetings"
    );
    guard_unwrap!(let ItemEnum::Module(hmod) = &hello.inner);
    guard_unwrap!(let [hid] = &hmod.items[..]);
    let hello_fn_i = &h.index[&hid];
    assert_eq!(hello_fn_i.name.as_ref().unwrap(), "hello");
    assert_eq!(
        hello_fn_i.docs.as_ref().unwrap(),
        "Display a frendly greeting"
    );
    guard_unwrap!(let ItemEnum::Function(hfn) = &hello_fn_i.inner);
    assert_eq!(hfn.decl.inputs, []);
    assert_eq!(hfn.decl.output, None);
}
