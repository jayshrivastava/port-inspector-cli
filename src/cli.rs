use clap::{App, Arg};
use std::net::IpAddr;

// Arguments is the final struct constructed by the
pub struct Arguments {
    pub ip_addr: IpAddr,
    pub n_threads: u16,
}

impl Arguments {
    // new() parses command line flags into an Arugments struct.
    pub fn new() -> Result<Arguments, &'static str> {
        // Defines flags for this program. Note that the following flags
        // are implicitly added:
        //     -h, --help       Prints help information
        //     -V, --version    Prints version information
        let matches = App::new(env!("CARGO_PKG_NAME"))
            .about(env!("CARGO_PKG_DESCRIPTION"))
            .author(env!("CARGO_PKG_AUTHORS"))
            .version(env!("CARGO_PKG_VERSION"))
            .arg(
                Arg::with_name(NUM_THREADS)
                    .short("t")
                    .long("threads")
                    .takes_value(true)
                    .help("The number of threads with which to inspect the specified IP address")
                    .default_value(DEFAULT_NUM_THREADS),
            )
            .arg(
                Arg::with_name(IP_ADDR)
                    .short("i")
                    .long("ip")
                    .takes_value(true)
                    .help("The IP address to inspect"),
            )
            .get_matches();

        let n_threads: u16 = match {
            match matches.value_of(NUM_THREADS) {
                None => return Err("cli: missing argument for number of threads"),
                Some(s) => match s.parse::<u16>() {
                    Ok(n) => Ok(n),
                    Err(_) => Err("cli: failed to parse uint16 value from number of threads"),
                },
            }
        } {
            Ok(n) => n,
            Err(e) => return Err(&e),
        };

        let ip_string: &str = match matches.value_of(IP_ADDR) {
            None => return Err("cli: missing ip address"),
            Some(s) => s,
        };

        let ip_addr = match ip_string.parse() {
            Ok(ip) => ip,
            Err(_) => return Err("cli: invalid ip address"),
        };

        return Ok(Arguments { ip_addr, n_threads });
    }
}

// Flags
const NUM_THREADS: &str = "num threads";
const IP_ADDR: &str = "ip address";

// Flag Defaults
const DEFAULT_NUM_THREADS: &str = "1";
