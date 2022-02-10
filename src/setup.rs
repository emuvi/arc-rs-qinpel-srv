use serde_json::Value;

use std::path::Path;
use std::sync::atomic::Ordering;
use std::collections::HashMap;

use crate::QinServer;

static DEFAULT_HOST: &str = "localhost";
static DEFAULT_PORT: u64 = 5490;

#[derive(Debug)]
pub struct Head {
    pub debug: bool,
    pub verbose: bool,
    pub server_host: String,
    pub server_port: u64,
    pub serves_apps: bool,
    pub serves_cmds: bool,
    pub serves_sqls: bool,
    pub serves_dirs: bool,
    pub redirects: Option<HashMap<String, String>>
}

impl Head {
    pub fn load(qinpel_srv: QinServer) -> Self {
        let mut setup_debug = false;
        let mut setup_verbose = false;
        let mut setup_host = String::from(DEFAULT_HOST);
        let mut setup_port = DEFAULT_PORT;
        let path = Path::new("setup.json");
        if path.exists() {
            let file = std::fs::File::open(path).expect("Setup file exists but could not be open.");
            let setup_file: Value =
                serde_json::from_reader(file).expect("Setup file exists but could not be parsed.");
            match &setup_file["serverDebug"] {
                Value::Bool(server_debug) => {
                    setup_debug = *server_debug;
                }
                _ => {}
            };
            match &setup_file["serverVerbose"] {
                Value::Bool(server_verbose) => {
                    setup_verbose = *server_verbose;
                }
                _ => {}
            };
            match &setup_file["serverHost"] {
                Value::String(server_host) => {
                    setup_host = String::from(server_host);
                }
                _ => {}
            };
            match &setup_file["serverPort"] {
                Value::Number(server_port) => {
                    setup_port = server_port
                        .as_u64()
                        .expect("Could not parse the server port from setup file.");
                }
                _ => {}
            };
        }
        if let Some(debug) = qinpel_srv.debug  {
            setup_debug = debug;
        }
        if let Some(verbose) = qinpel_srv.verbose {
            setup_verbose = verbose;
        }
        if let Some(server_host) = qinpel_srv.server_host {
            setup_host = server_host;
        }
        if let Some(server_port) = qinpel_srv.server_port {
            setup_port = server_port;
        }
        if setup_debug {
            crate::DEBUG.store(true, Ordering::Relaxed);
        }
        if setup_verbose {
            crate::VERBOSE.store(true, Ordering::Relaxed);
        }
        Head {
            debug: setup_debug,
            verbose: setup_verbose,
            server_host: setup_host,
            server_port: setup_port,
            serves_apps: qinpel_srv.serves_apps,
            serves_cmds: qinpel_srv.serves_cmds,
            serves_sqls: qinpel_srv.serves_sqls,
            serves_dirs: qinpel_srv.serves_dirs,
            redirects: qinpel_srv.redirects,
        }
    }
}
