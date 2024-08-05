use vmprotect::protected;

fn main() {
    let data = protected!(cstr "hello");
    println!("{}", data);
}
