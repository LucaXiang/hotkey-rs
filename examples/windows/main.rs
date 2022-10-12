#[cfg(not(target_os = "windows"))]
fn main() {
    println!("please run this example on windows");
}

#[cfg(target_os = "windows")]
fn main() {
    println!("Hello World!");
}
