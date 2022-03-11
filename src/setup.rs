use serde_json::Value;

use std::collections::HashMap;
use std::path::Path;

use crate::QinServer;

static DEFAULT_NAME: &str = "QinpelSrv";
static DEFAULT_HOST: &str = "localhost";
static DEFAULT_PORT: u64 = 5490;

#[derive(Debug)]
pub struct Head {
    pub verbose: bool,
    pub archive: bool,
    pub server_name: String,
    pub server_host: String,
    pub server_port: u64,
    pub serves_pubs: bool,
    pub serves_apps: bool,
    pub serves_dirs: bool,
    pub serves_cmds: bool,
    pub serves_sqls: bool,
    pub serves_lizs: bool,
    pub redirects: Option<HashMap<String, String>>,
}

impl Head {
    pub fn load(qinpel_srv: QinServer) -> Self {
        let mut setup_verbose = false;
        let mut setup_archive = false;
        let mut setup_name = String::from(DEFAULT_NAME);
        let mut setup_host = String::from(DEFAULT_HOST);
        let mut setup_port = DEFAULT_PORT;
        let mut setup_pubs = false;
        let mut setup_apps = false;
        let mut setup_dirs = false;
        let mut setup_cmds = false;
        let mut setup_sqls = false;
        let mut setup_lizs = false;
        let mut setup_redirects: Option<HashMap<String, String>> = None;
        let path = Path::new("setup.json");
        if path.exists() {
            let file = std::fs::File::open(path).expect("Setup file exists but could not be open.");
            let setup_file: Value =
                serde_json::from_reader(file).expect("Setup file exists but could not be parsed.");
            match &setup_file["serverVerbose"] {
                Value::Bool(server_verbose) => {
                    setup_verbose = *server_verbose;
                }
                _ => {}
            };
            match &setup_file["serverArchive"] {
                Value::Bool(server_archive) => {
                    setup_archive = *server_archive;
                }
                _ => {}
            };
            match &setup_file["serverName"] {
                Value::String(server_name) => {
                    setup_name = String::from(server_name);
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
            match &setup_file["servesPUBs"] {
                Value::Bool(serves_pubs) => {
                    setup_pubs = *serves_pubs;
                }
                _ => {}
            };
            match &setup_file["servesAPPs"] {
                Value::Bool(serves_apps) => {
                    setup_apps = *serves_apps;
                }
                _ => {}
            };
            match &setup_file["servesDIRs"] {
                Value::Bool(serves_dirs) => {
                    setup_dirs = *serves_dirs;
                }
                _ => {}
            };
            match &setup_file["servesCMDs"] {
                Value::Bool(serves_cmds) => {
                    setup_cmds = *serves_cmds;
                }
                _ => {}
            };
            match &setup_file["servesSQLs"] {
                Value::Bool(serves_sqls) => {
                    setup_sqls = *serves_sqls;
                }
                _ => {}
            };
            match &setup_file["servesLIZs"] {
                Value::Bool(serves_lizs) => {
                    setup_lizs = *serves_lizs;
                }
                _ => {}
            };
            match &setup_file["serverRedirects"] {
                Value::Object(server_redirects) => {
                    for (key, value) in server_redirects {
                        match value {
                            Value::String(destiny) => {
                                let origin = String::from(key);
                                let destiny = String::from(destiny);
                                if let Some(ref mut redirects) = setup_redirects {
                                    redirects.insert(origin, destiny);
                                } else {
                                    let mut redirects: HashMap<String, String> = HashMap::new();
                                    redirects.insert(origin, destiny);
                                    setup_redirects = Some(redirects);
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            };
        }
        if let Some(verbose) = qinpel_srv.verbose {
            setup_verbose = verbose;
        }
        if setup_verbose {
            liz::liz_debug::put_verbose();
        }
        if let Some(archive) = qinpel_srv.archive {
            setup_archive = archive;
        }
        if setup_archive {
            liz::liz_debug::put_archive();
        }
        if let Some(server_name) = qinpel_srv.server_name {
            setup_name = server_name;
        }
        if let Some(server_host) = qinpel_srv.server_host {
            setup_host = server_host;
        }
        if let Some(server_port) = qinpel_srv.server_port {
            setup_port = server_port;
        }
        if let Some(serves_pubs) = qinpel_srv.serves_pubs {
            setup_pubs = serves_pubs;
        }
        if let Some(serves_apps) = qinpel_srv.serves_apps {
            setup_apps = serves_apps;
        }
        if let Some(serves_dirs) = qinpel_srv.serves_dirs {
            setup_dirs = serves_dirs;
        }
        if let Some(serves_cmds) = qinpel_srv.serves_cmds {
            setup_cmds = serves_cmds;
        }
        if let Some(serves_sqls) = qinpel_srv.serves_sqls {
            setup_sqls = serves_sqls;
        }
        if let Some(serves_lizs) = qinpel_srv.serves_lizs {
            setup_lizs = serves_lizs;
        }
        if let Some(server_redirects) = qinpel_srv.redirects {
            if let Some(ref mut redirects) = setup_redirects {
                for (origin, destiny) in server_redirects {
                    redirects.insert(origin, destiny);
                }
            } else {
                setup_redirects = Some(server_redirects);
            }
        }
        Head {
            verbose: setup_verbose,
            archive: setup_archive,
            server_name: setup_name,
            server_host: setup_host,
            server_port: setup_port,
            serves_pubs: setup_pubs,
            serves_apps: setup_apps,
            serves_dirs: setup_dirs,
            serves_cmds: setup_cmds,
            serves_sqls: setup_sqls,
            serves_lizs: setup_lizs,
            redirects: setup_redirects,
        }
    }
}
