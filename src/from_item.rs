use rustdoc_types::*;

pub(crate) trait FromItem {
    fn from_item(i: &Item) -> &Self;
}

macro_rules! impl_from_item {
    ($($name:ident)*) => {
        paste::paste! {
            $(
                impl FromItem for $name {
                    fn from_item(i: &Item) -> &Self {
                        match &i.inner {
                            ItemEnum::$name(x) => x,
                            _ => panic!(concat!("Expected ItemEnum::", stringify!($name), " but got {:?}"), i.inner),
                        }
                    }
                }
            )*
        }
    };
}

impl_from_item! {
    Module
    Import
    Union
    Struct
    Enum
    Variant
    // TODO: StructField(Type)
    Function
    Trait
    TraitAlias
    Method
    Impl
    Typedef
    OpaqueTy
    Constant
    Static
    // TODO: Macro(String)
    // TODO: ExternCrate {..}
    // TODO: ForeinType
    ProcMacro
    // TODO: PrimitiveType
    // TODO: AssocConst {..}
    // TODO: AssocType {..}
}

pub(crate) trait IntoKind {
    fn into_kind<T: FromItem>(&self) -> &T;
}

impl IntoKind for Item {
    fn into_kind<T: FromItem>(&self) -> &T {
        T::from_item(self)
    }
}
