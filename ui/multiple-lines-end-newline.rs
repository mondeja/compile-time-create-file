use compile_time_create_file::create_file;

fn main() {
    create_file!("multiple-lines-end-newline.expect.txt", "foo\nbar\nbaz\n");
}
