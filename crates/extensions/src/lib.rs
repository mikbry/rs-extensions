include!("generated.rs");

pub fn init() {
    println!("Init Plugins");
    init_all();
    println!("{}", message());
}
