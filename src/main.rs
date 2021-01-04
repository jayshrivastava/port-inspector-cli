use std::process;

mod cli;

fn main() {
    // For all command line arg related errors, print the error and use exit code 1.
    let _args = cli::Arguments::new().unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
}

#[cfg(test)]
mod test_cli {
    use assert_cmd::Command;

    const BINARY_NAME: &str = env!("CARGO_PKG_NAME");

    #[test]
    fn test_missing_ip() {
        let mut cmd = Command::cargo_bin(BINARY_NAME).unwrap();
        cmd.assert()
            .failure()
            .code(1)
            .stderr("cli: missing ip address\n");
    }

    #[test]
    fn test_invalid_num_threads() {
        let mut cmd = Command::cargo_bin(BINARY_NAME).unwrap();
        cmd.arg("-t")
            .arg("'-1'")
            .assert()
            .failure()
            .code(1)
            .stderr("cli: failed to parse uint16 value from number of threads\n");
    }

    #[test]
    fn test_invalid_ip() {
        let mut cmd = Command::cargo_bin(BINARY_NAME).unwrap();
        cmd.arg("-i")
            .arg("127.0.0..")
            .assert()
            .failure()
            .code(1)
            .stderr("cli: invalid ip address\n");
    }

    #[test]
    fn test_valid_ip() {
        let mut cmd = Command::cargo_bin(BINARY_NAME).unwrap();
        cmd.arg("-i").arg("127.0.0.1").assert().success();
    }

    #[test]
    fn test_valid_with_threads() {
        let mut cmd = Command::cargo_bin(BINARY_NAME).unwrap();
        cmd.arg("-t")
            .arg("10")
            .arg("-i")
            .arg("127.0.0.1")
            .assert()
            .success();
    }
}
