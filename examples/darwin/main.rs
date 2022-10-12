#[cfg(not(target_os = "darwin"))]
fn main() {
    println!("please run this example on darwin");
}

#[cfg(target_os = "linux")]
fn main() {
    println!("Hello World!");
}
