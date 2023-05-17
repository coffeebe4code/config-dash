use config_dash::check_config;

pub static mut CONFIG_FIELD: usize = 0;
#[check_config(config.yaml)]
pub struct Config {
    field: usize,
    url: String,
}

fn main() {
    CONFIG_FIELD = 12;
    println!("Hello, world!");
}
