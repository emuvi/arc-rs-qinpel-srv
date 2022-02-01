use clap::ArgMatches;
use serde_json::Value;
use std::path::Path;

static DEFAULT_HOST: &str = "localhost";
static DEFAULT_PORT: u64 = 5490;

#[derive(Debug)]
pub struct Head {
    pub debug: bool,
    pub host: String,
    pub port: u64,
}

impl Head {
    pub fn load(args: ArgMatches<'_>) -> Self {
        let mut setup_host = DEFAULT_HOST;
        let mut setup_port = DEFAULT_PORT;
        let setup_debug = args.is_present("debug");
        if args.is_present("host") {
            setup_host = args
                .value_of("host")
                .expect("Could not read the host argument.");
        }
        if args.is_present("port") {
            setup_port = args
                .value_of("port")
                .expect("Could not read the port argument.")
                .parse()
                .expect("Could not parse the port argument.");
        }
        let path = Path::new("setup.json");
        let setup_value: Value = if !path.exists() {
            Value::Null
        } else {
            let file = std::fs::File::open(path).expect("Setup file exists but could not be open.");
            serde_json::from_reader(file).expect("Setup file exists but could not be parsed.")
        };
        if !args.is_present("host") {
            let server_host = match &setup_value["serverHost"] {
                Value::String(server_host) => &server_host,
                Value::Null => DEFAULT_HOST,
                _ => panic!("Wrong type value for the host in the setup file."),
            };
            setup_host = server_host;
        }
        if !args.is_present("port") {
            let server_port = match &setup_value["serverPort"] {
                Value::Number(server_port) => server_port
                    .as_u64()
                    .expect("Could not parse the server port from setup file."),
                Value::Null => DEFAULT_PORT,
                _ => panic!("Wrong type value for the port in the setup file."),
            };
            setup_port = server_port;
        }
        Head {
            debug: setup_debug,
            host: String::from(setup_host),
            port: setup_port,
        }
    }
}
