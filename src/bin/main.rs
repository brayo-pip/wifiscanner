fn main() {
    let networks = wifi_scanner::scan().expect("Cannot scan network");
    for network in networks {
        println!(
            "{} {:15} {:10} {:4} {}",
            network.mac, network.ssid, network.channel, network.signal_level, network.security
        );
    }
}
