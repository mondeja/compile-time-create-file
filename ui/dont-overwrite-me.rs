use create_file_macro::create_file;

fn main() {
    create_file!(
        "../dont-overwrite-me.expect.txt",
        "Sadly, I've been overwritten :(\n"
    );
}
