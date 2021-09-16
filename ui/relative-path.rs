use create_file_macro::create_file;

fn main() {
    create_file!("./../relative-path.expect.txt", "foo bar baz qux");
}
