fn main() {
    print_config_max(None);
    print_config_max(Some(3u8));
}

fn print_config_max(config_max: Option<u8>) {
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max)
    }
}
