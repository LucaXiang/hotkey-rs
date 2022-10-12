#[cfg(not(target_os = "linux"))]
fn main() {
    println!("please run this example on linux");
}

#[cfg(target_os = "linux")]
fn main() {
    println!("Hello World!");
}
