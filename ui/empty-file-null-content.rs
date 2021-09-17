use compile_time_create_file::create_file;

fn main() {
    create_file!("empty-file-null-content.expect.txt", None);
}
