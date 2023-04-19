use clap::App;
use blkrs::run_lsblk;

fn main() {
    let matches = App::new("lsblk")
        .version("0.0.1")
        .author("Alfredo Deza")
        .about("lsblk in Rust")
        .arg(
            clap::Arg::with_name("device")
                .help("Device to query")
                .required(true)
                .index(1)
        )
        .get_matches();

    let device = matches.value_of("device").unwrap();
    let output = serde_json::to_string(&run_lsblk(&device)).unwrap();
    println!("{}", output);
}
