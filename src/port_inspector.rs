use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::sync::mpsc::{channel, Sender};
use std::thread;

pub fn inspect(ip: IpAddr, n_threads: u16) {
    let (tx, rx) = channel();
    // Since port 0 is reserved, ports are scanned starting from 1.
    // Ports are distributed among threads. Ex. (assume 3 threads):
    // thread 0: ports 1, 4, 7, 10...
    // thread 1: ports 2, 5, 8, 11...
    // thread 2: ports 3, 6, 9, 12...
    for start_port in 0..n_threads {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            scan(tx_clone, start_port + 1, ip, n_threads);
        });
    }

    println!("{:?}", ip);
    // The channel closes when all the tx clones have dropped.
    // Thus, it is necessary to drop the original here.
    drop(tx);
    let mut out = vec![];
    for port in rx {
        out.push(port);
    }
    out.sort();
    println!("");
    println!("The following ports are open:");
    for port in out {
        println!("{}", port)
    }
}

const MAX_PORT: u16 = 65535;

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, n_threads: u16) {
    let mut port: u16 = start_port;
    loop {
        let connection = TcpStream::connect((addr, port));
        match connection {
            Ok(_) => {
                print!(".");
                // Since we are printing inside a thread, writing
                // to stdout must be synchronized using the line below.
                io::stdout().lock().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        };

        // Avoid u16 overflow.
        if port > MAX_PORT - n_threads {
            break;
        }
        port += n_threads;
    }
}
