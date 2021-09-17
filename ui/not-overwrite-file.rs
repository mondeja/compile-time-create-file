use compile_time_create_file::create_file;

fn main() {
    create_file!(
        "../not-overwrite-file.expect.txt",
        "Sadly, I've been overwritten :(\n"
    );
}
