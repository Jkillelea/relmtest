use std::process::*;

fn main() {
    Command::new("glib-compile-resources")
            .arg("data/resources.xml")
            .spawn()
            .expect("Command failed!");
}
