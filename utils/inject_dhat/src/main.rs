use quote::quote;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::process;

fn inject_dhat_scoped_variable(syntax: &mut syn::File) {
    let dhat = syn::parse_str("let _dhat = Dhat::start_heap_profiling();")
        .expect("Unable to parse the dhat string");

    // Find the main function.
    for item in syntax.items.iter_mut() {
        match item {
            syn::Item::Fn(ref mut fn_item) => {
                if fn_item.sig.ident.to_string() == "main" {
                    // Create a new vector with the injected statement at the beginning.
                    let mut new_stmts = vec![dhat];
                    for stmt in fn_item.block.stmts.drain(..) {
                        new_stmts.push(stmt);
                    }
                    fn_item.block.stmts = new_stmts;
                    return;
                }
            }
            _ => {}
        }
    }

    panic!("Unable to find the main function.");
}

fn inject_allocator_declaration(syntax: &mut syn::File) {
    let use_code = "use dhat::{Dhat, DhatAlloc};";
    let allocator_code = "
        #[global_allocator]
        static ALLOCATOR: DhatAlloc = DhatAlloc;
    ";
    let mut new_items = vec![
        syn::parse_str(use_code).expect("Unable to parse the dhat use string"),
        syn::parse_str(allocator_code).expect("Unable to parse the allocator string"),
    ];

    for item in syntax.items.drain(..) {
        new_items.push(item);
    }
    syntax.items = new_items;
}

fn run_injection(filename: &PathBuf) -> String {
    let mut file = File::open(&filename).expect("Unable to open file");

    let mut src = String::new();
    file.read_to_string(&mut src).expect("Unable to read file");

    let mut syntax = syn::parse_file(&src).expect("Unable to parse file");

    inject_allocator_declaration(&mut syntax);
    inject_dhat_scoped_variable(&mut syntax);

    quote!(#syntax).to_string()
}

fn main() {
    let mut args = env::args();

    let filename = match (args.next(), args.next(), args.next()) {
        (Some(_executable), Some(filename), None) => PathBuf::from(filename),
        _ => {
            eprintln!("Usage: cargo run --package inject_dhat -- path/to/filename.rs");
            process::exit(1);
        }
    };

    // The code is output to stdout.
    println!("{}", run_injection(&filename));
}

#[cfg(test)]
mod test {
    use super::*;
    use std::env;

    #[test]
    fn test_dhat_injection() {
        let path = {
            // Get the path to: "$CARGO_MANIFEST_DIR/utils/inject_dhat/tests/fixtures/code.rs"
            // $CARGO_MANIFEST_DIR always points to the root of the workspace.
            let root_dir = &env::var("CARGO_MANIFEST_DIR").expect("$CARGO_MANIFEST_DIR");
            let mut path = PathBuf::from(root_dir);
            path.push("tests/fixtures/code.rs");
            path
        };

        // The result ends up on one line of text which is a bit hard to read.
        // Split it up on the semi-colons to improve readability.
        let result: Vec<String> = run_injection(&path).split(";").map(String::from).collect();

        assert_eq!(
            result,
            [
                // Injected code:
                "use dhat :: { Dhat , DhatAlloc } ",
                " # [global_allocator] static ALLOCATOR : DhatAlloc = DhatAlloc ",
                // Original code:
                " use std :: env ",
                " fn do_something () { println ! (\"It has another function in it.\") ",
                " println ! (\"It uses an import. {}\" , env ! (\"CARGO_PKG_VERSION\") ",
                ") ",
                // Injected code:
                " } fn main () { let _dhat = Dhat :: start_heap_profiling () ",
                //               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
                // Original:
                " println ! (\"This is a test fixture\") ",
                " }",
            ],
        );
    }
}
