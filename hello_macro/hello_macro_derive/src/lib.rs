// custom derive macros must live in their own crate; the convention is that for the crate named `foo` the custom derive macro crate is named `foo_derive`
extern crate proc_macro;

use proc_macro::TokenStream; // `proc_macro` crate is the compiler’s API that allows us to read and manipulate Rust code from our code (basically writing compiler plugins)
use quote::quote; // `quote` crate turns `syn` data structures back into Rust code
use syn; // `syn` crate parses Rust code from a string into an abstract syntax tree (AST)

#[proc_macro_derive(HelloMacro)]
// `hello_macro_derive` function will be called when a user of our library specifies `#[derive(HelloMacro)]` on a type; this is possible because we’ve annotated `hello_macro_derive` function here with `proc_macro_derive` and specified the name, `HelloMacro`, which matches our trait name; this is the convention most procedural macros follow
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap(); // construct a representation of Rust code as a syntax tree that we can manipulate; it’s necessary for our procedural macro to panic on errors because `proc_macro_derive` functions must return `TokenStream` rather than `Result` to conform to the procedural macro API; in production code provide more specific error messages about what went wrong should be provided by using `panic!` or `expect`.

    impl_hello_macro(&ast) // build the trait implementation
}

// we’ve split the code into `hello_macro_derive` function, which is responsible for parsing the `TokenStream`, and `impl_hello_macro function`, which is responsible for transforming the syntax tree: this makes writing a procedural macro more convenient; the code in the outer function (`hello_macro_derive` in this case) will be the same for almost every procedural macro crate; the code specified in the body of the inner function (`impl_hello_macro` in this case) will be different depending on the procedural macro’s purpose.
fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name)); // `stringify!` macro used here is built into Rust; it takes a Rust expression, such as `1 + 2`, and at compile time turns the expression into a string literal, such as "1 + 2", without evaluating it
            }
        }
    };
    gen.into()
}
