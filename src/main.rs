use std::env;
use std::net::ipAddr;

struct arguments {
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

impl Arguments {
    fn new(args: &Vec<String>) -> Result<arguments, &'static str> {
        // The minimum number of arguments must be 2. For example,
        // the name of the binary + the `-h` flag.
        if args.len() < 2 {
            return Err("not enough arguments");
        // The maximum number of arguments is 4. For example,
        // The name of the binary + `-t` `{num-threads}` `ip-addr`
        } else if args.len() > 4 {
            return Err("too many arguments");
        }
        return Ok()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    for i in &args {
        println!("{}", i);
    }
    println!("{:?}", args);

    yeet();

    let a = args[0].clone();


}
