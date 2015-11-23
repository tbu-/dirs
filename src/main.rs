extern crate foobar;

fn main() {
    let dirs = foobar::Directories::with_prefix("foobar", "Foobar").unwrap();
    println!("config_home: {:?}", dirs.config_home());
    println!("cache_home:  {:?}", dirs.cache_home());
    println!("config_dirs: {:?}", dirs.config_dirs());
}
