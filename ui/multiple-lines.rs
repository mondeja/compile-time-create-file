use create_file_macro::create_file;

fn main() {
    create_file!("multiple-lines.expect.txt", "foo\nbar\nbaz");
}
