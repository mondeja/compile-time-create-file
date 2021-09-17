use compile_time_create_file::create_file;

fn main() {
    create_file!(
        "foo/path-to-file-in-subdirectory.expect.txt",
        "Hello from subdirectory!"
    );
}
