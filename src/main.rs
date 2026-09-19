/*
    Примеры запуска:

C:\Users\user\Job\Rust\CookBookProjects\c_0012_clap_derive>cargo run -- -h
Usage: c_0012_clap_derive.exe [OPTIONS] --server-addres <SERVER_ADDRES>

Options:
  -l, --long-level <LOG_LEVEL>
          Logging threshold [possible values: debug, error, warn, trace, info]
  -a, --server-addres <SERVER_ADDRES>
          The Backend Server Address
  -p, --server-port <SERVER_PORT>
          The Backend Server Port
  -s, --snapshot-freq <SNAPSHOT_FREQUENCY>
          Interval between database snapshots in seconds
  -h, --help
          Print help




C:\Users\user\Job\Rust\CookBookProjects\c_0012_clap_derive>cargo run -- -a 100.10.10.2 -l debug -p 8080 -s 3600
CliArgs { log_level: Some(Debug), server_addres: 100.10.10.2, server_port: Some(8080), snapshot_frequency: Some(3600) }
[src\main.rs:45:5] args = CliArgs {
    log_level: Some(
        Debug,
    ),
    server_addres: 100.10.10.2,
    server_port: Some(
        8080,
    ),
    snapshot_frequency: Some(
        3600,
    ),
}    

*/
use clap::{Parser, ValueEnum} ;
use std::net::IpAddr ;

#[derive(
    Debug, 
    Clone, 
    // derive macros, который позволяет использовать enum как значение аргумента 
    // командной строки
    ValueEnum 
)]
enum LogLevel {
    Debug,
    Error,
    Warn,
    Trace,
    Info
}

#[derive(Parser, Debug)]
struct CliArgs {
    /// Logging threshold
    #[arg(short='l', long = "long-level")]
    log_level: Option<LogLevel>,    // не обязательное поле

    /// The Backend Server Address
    #[arg(short = 'a',long = "server-addres")]
    server_addres: IpAddr,  // обязательное поле

    /// The Backend Server Port
    #[arg(short = 'p', long = "server-port")]
    server_port: Option<u16>,   // не обязательное поле

    /// Interval between database snapshots in seconds
    #[arg(short = 's', long = "snapshot-freq")]
    snapshot_frequency: Option<usize>   // не обязательное поле
}


fn main() {
    let args = CliArgs::parse() // Parse from std::env::args_os(), exit on error.
    ;

    println!("{:?}", args) ;

    dbg!(args) ;
}
