//! 19.5. Macros

mod custom_derive_macro {

    use hello_macro::HelloMacro;
    use hello_macro_derive::HelloMacro;

    #[derive(HelloMacro)]
    struct Pancakes;

    pub fn custom_derive_macro_main() {
        Pancakes::hello_macro();
    }

}

fn main() {

    use custom_derive_macro as cust_der_macro;

    cust_der_macro::custom_derive_macro_main()

}
