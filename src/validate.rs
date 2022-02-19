// TODO: Don't
#![allow(unused_variables)]

use rustdoc_types::*;

use crate::TCrate;

impl TCrate {
    pub(super) fn validate_item(&self, item: &Item) {
        match &item.inner {
            ItemEnum::Constant(i) => self.validate_constant(i),
            ItemEnum::Enum(i) => self.validate_enum(i),
            ItemEnum::Function(i) => self.validate_function(i),
            ItemEnum::Impl(i) => self.validate_impl(i),
            ItemEnum::Import(i) => self.validate_import(i),
            ItemEnum::Macro(i) => self.validate_macro(i),
            ItemEnum::Method(i) => self.validate_method(i),
            ItemEnum::Module(i) => self.validate_module(i),
            ItemEnum::OpaqueTy(i) => self.validate_opaque_ty(i),
            ItemEnum::PrimitiveType(i) => self.validate_primitive_type(i),
            ItemEnum::ProcMacro(i) => self.validate_proc_macro(i),
            ItemEnum::Static(i) => self.validate_static(i),
            ItemEnum::Struct(i) => self.validate_struct(i),
            ItemEnum::StructField(i) => self.validate_struct_field(i),
            ItemEnum::Trait(i) => self.validate_trait(i),
            ItemEnum::TraitAlias(i) => self.validate_trait_alias(i),
            ItemEnum::Typedef(i) => self.validate_typedef(i),
            ItemEnum::Union(i) => self.validate_union(i),
            ItemEnum::Variant(i) => self.validate_variant(i),

            ItemEnum::ForeignType => todo!(),
            ItemEnum::ExternCrate { name, rename } => todo!(),
            ItemEnum::AssocConst { type_, default } => todo!(),
            ItemEnum::AssocType { bounds, default } => todo!(),
        }
    }

    fn validate_type(&self, t: &Type) {
        // TODO
    }

    fn validate_generics(&self, g: &Generics) {
        // TODO
    }

    fn validate_decl(&self, d: &FnDecl) {}

    fn validate_generic_bound(&self, b: &GenericBound) {}

    // --- ItemEnum Variants ---
    fn validate_module(&self, m: &Module) {
        for i in &m.items {
            self.check_in_index(i)
        }
    }

    fn validate_struct(&self, i: &Struct) {
        self.validate_generics(&i.generics);
        for i in &i.fields {
            self.check_in_index(i)
        }
        for i in &i.impls {
            self.check_in_index(i)
        }
    }
    fn validate_struct_field(&self, i: &Type) {
        self.validate_type(i);
    }

    fn validate_enum(&self, i: &Enum) {
        self.validate_generics(&i.generics);
        self.checks_in_index(&i.variants);
        self.checks_in_index(&i.impls);
    }

    fn validate_variant(&self, i: &Variant) {
        match i {
            Variant::Plain => {}
            Variant::Tuple(tys) => {
                for t in tys {
                    self.validate_type(t)
                }
            }
            Variant::Struct(s) => self.checks_in_index(s),
        }
    }

    fn validate_function(&self, i: &Function) {
        self.validate_generics(&i.generics);
        self.validate_decl(&i.decl);
    }
    fn validate_method(&self, i: &Method) {
        self.validate_generics(&i.generics);
        self.validate_decl(&i.decl);
    }

    // TODO assoc_const is the same
    fn validate_constant(&self, i: &Constant) {
        self.validate_type(&i.type_);
    }
    fn validate_static(&self, i: &Static) {
        self.validate_type(&i.type_);
    }

    fn validate_typedef(&self, i: &Typedef) {
        self.validate_type(&i.type_);
        self.validate_generics(&i.generics);
    }

    fn validate_opaque_ty(&self, i: &OpaqueTy) {
        self.validate_generics(&i.generics);
        self.validate_generic_bounds(&i.bounds);
    }
    fn validate_trait_alias(&self, i: &TraitAlias) {
        self.validate_generics(&i.generics);
        self.validate_generic_bounds(&i.params);
    }

    fn validate_trait(&self, i: &Trait) {
        self.checks_in_index(&i.implementors);
        // TODO: Re-enable when it doesnt panic
        // self.checks_in_index(&i.items);
        self.validate_generics(&i.generics);
        self.validate_generic_bounds(&i.bounds);
    }

    fn validate_impl(&self, i: &Impl) {
        self.validate_generics(&i.generics);
        if let Some(trait_) = &i.trait_ {
            // TODO: Check is trait
            self.validate_type(trait_);
        }
        if let Some(blanket) = &i.blanket_impl {
            self.validate_type(blanket);
        }
        self.validate_type(&i.for_);
        self.checks_in_index(&i.items);
    }

    fn validate_import(&self, i: &Import) {
        todo!()
    }

    fn validate_macro(&self, i: &String) {
        todo!()
    }

    fn validate_primitive_type(&self, i: &String) {
        todo!()
    }

    fn validate_proc_macro(&self, i: &ProcMacro) {
        todo!()
    }

    fn validate_union(&self, i: &Union) {
        todo!()
    }

    // --- Helpers ---
    fn check_in_index(&self, i: &Id) {
        // TODO: Check that the item has the right type
        assert!(self.krate.index.contains_key(i), "Key {:?} not found", i);
    }

    // --- Loop Helpers ---
    fn checks_in_index(&self, is: &[Id]) {
        for i in is {
            self.check_in_index(i);
        }
    }

    fn validate_generic_bounds(&self, bs: &[GenericBound]) {
        for b in bs {
            self.validate_generic_bound(b);
        }
    }
}
