use proc_macro::TokenStream;
use std::env::current_dir;
use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::path::{is_separator, PathBuf};

use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_macro_input, LitStr, Result}; //Error

struct FilenameContent {
    filename: String,
    content: Option<String>,
}

impl Parse for FilenameContent {
    fn parse(input: ParseStream) -> Result<Self> {
        type Inner = Punctuated<LitStr, Comma>;
        let mut args = Inner::parse_terminated(input)?;

        let arg2 = args.pop();
        let arg1 = args.pop();
        if arg1.is_none() {
            Ok(Self {
                // arg2 really is the first and unique argument here
                filename: arg2.unwrap().into_value().value(),
                content: None,
            })
        } else {
            Ok(Self {
                filename: arg1.unwrap().into_value().value(),
                content: Some(arg2.unwrap().into_value().value()),
            })
        }
    }
}

#[proc_macro]
pub fn create_file(tokens: TokenStream) -> TokenStream {
    // parse arguments list
    let args = parse_macro_input!(tokens as FilenameContent);

    // build path
    let mut path: PathBuf = current_dir().unwrap();
    let path_to_join: PathBuf = PathBuf::from(args.filename);
    let path_to_join_parent = path_to_join.parent();

    // if passed filename contains a non existent directory, create it
    if path_to_join_parent.is_some() {
        create_dir_all(path_to_join_parent.unwrap()).ok();
    }
    path.push(&path_to_join);

    // don't overwrite existing files
    if !path.exists() {
        let last_path_character = path_to_join.to_str().unwrap().chars().last().unwrap();
        if is_separator(last_path_character) {
            // path is a directory, just create it
            create_dir_all(path_to_join).ok();
        } else {
            // create file without overwriting it
            let mut file = File::create(path).unwrap();
            if args.content.is_some() {
                file.write_all(args.content.unwrap().as_bytes()).unwrap();
            }
        }
    }

    TokenStream::new()
}

#[cfg(test)]
mod tests {
    #[test]
    fn ui() {
        let t = trybuild::TestCases::new();

        t.pass("ui/one-line.rs");
        t.pass("ui/multiple-lines.rs");
        t.pass("ui/multiple-lines-end-newline.rs");
        t.pass("ui/not-overwrite-file.rs");
        t.pass("ui/not-overwrite-directory.rs");
        t.pass("ui/relative-path.rs");
        t.pass("ui/path-to-file-in-subdirectory.rs");
        t.pass("ui/path-to-subdirectory.rs");
        t.pass("ui/path-to-absolute-file.rs");
        t.pass("ui/path-to-absolute-directory.rs");
        t.pass("ui/empty-file-one-arg.rs");
        t.pass("ui/empty-file-string-content.rs");
    }
}
