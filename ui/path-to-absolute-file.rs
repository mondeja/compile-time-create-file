use compile_time_create_file::create_file;

fn main() {
    create_file!(
        "/tmp/path-to-absolute-file.expect.txt",
        "Hello from absolute file!"
    );
}
