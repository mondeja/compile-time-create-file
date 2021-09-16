use create_file_macro::create_file;

fn main() {
    create_file!("multiple-lines-end-newline.expect.txt", "foo\nbar\nbaz\n");
}
