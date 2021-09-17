use compile_time_create_file::create_file;

fn main() {
    create_file!("one-line.expect.txt", "foo bar baz");
}
