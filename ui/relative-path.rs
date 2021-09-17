use compile_time_create_file::create_file;

fn main() {
    create_file!("./../relative-path.expect.txt", "foo bar baz qux");
}
