use proc_macro::TokenStream;
use std::env::current_dir;
use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::path::PathBuf;

use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_macro_input, Error, LitStr, Result};

struct FilenameContent {
    filename: String,
    content: String,
}

impl Parse for FilenameContent {
    fn parse(input: ParseStream) -> Result<Self> {
        type Inner = Punctuated<LitStr, Comma>;
        let mut args = Inner::parse_terminated(input)?;
        if args.len() != 2 {
            return Err(Error::new(
                input.cursor().span(),
                "wrong number of arguments: 2 expected",
            ));
        }

        let content = args.pop();
        if content.is_none() {
            return Err(Error::new(
                input.cursor().span(),
                "missing required argument: content",
            ));
        };
        let filename = args.pop();
        if filename.is_none() {
            return Err(Error::new(
                input.cursor().span(),
                "missing required argument: filename",
            ));
        };

        Ok(Self {
            filename: filename.unwrap().into_value().value(),
            content: content.unwrap().into_value().value(),
        })
    }
}

#[proc_macro]
pub fn create_file(tokens: TokenStream) -> TokenStream {
    // parse arguments list
    let args = parse_macro_input!(tokens as FilenameContent);

    // build path
    let mut path: PathBuf = current_dir().unwrap();
    // if passed filename contains a non existent directory, create it
    let path_to_join: PathBuf = PathBuf::from(args.filename);
    let path_to_join_parent = path_to_join.parent();
    if path_to_join_parent.is_some() {
        create_dir_all(path_to_join_parent.unwrap()).ok();
    }
    path.push(path_to_join);

    // create file without overwriting it
    if !path.exists() {
        let mut file = File::create(path).unwrap();
        file.write_all(args.content.as_bytes()).unwrap();
    }

    TokenStream::new()
}

#[cfg(test)]
mod tests {
    #[test]
    fn ui() {
        let t = trybuild::TestCases::new();
        t.compile_fail("ui/zero-arguments.rs");
        t.compile_fail("ui/one-argument.rs");
        t.compile_fail("ui/three-arguments.rs");

        t.pass("ui/one-line.rs");
        t.pass("ui/multiple-lines.rs");
        t.pass("ui/multiple-lines-end-newline.rs");
        t.pass("ui/dont-overwrite-me.rs");
        t.pass("ui/relative-path.rs");
        t.pass("ui/path-to-subdirectory.rs")
    }
}
