use create_file_macro::create_file;

fn main() {
    create_file!(
        "foo/path-to-subdirectory.expect.txt",
        "Hello from subdirectory!"
    );
}
