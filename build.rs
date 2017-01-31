extern crate gcc;

fn main() {
    gcc::Config::new().file("objc/WBWindow.m").flag("-fmodules").compile("libwbwindow.a");
}
