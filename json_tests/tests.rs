#[macro_export]
macro_rules! json_tests {
    ($($name:ident)*) => {
        paste::paste!{
        $(
            mod [<$name _test>];
            #[test]
            fn $name() {

                // [<$name>] will convert r#enum to enum
                let path = camino::Utf8PathBuf::from(file!()).parent().unwrap().join(stringify!([<$name>])).with_extension("rs");

                let krate = crate::load_json(
                    crate::Version::Nightly, &path
                ).unwrap();
                [<$name _test>]::test(krate);
            }
        )*
    }
    }
}

mod types;

json_tests! {
    hello
}
