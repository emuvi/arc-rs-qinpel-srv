use qinpel_srv::QinServer;

mod clip;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = clip::parse();
    let arg_verbose = if args.is_present("verbose") {
        Some(true)
    } else {
        None
    };
    let arg_archive = if args.is_present("archive") {
        Some(true)
    } else {
        None
    };
    let arg_name = if args.is_present("name") {
        Some(String::from(
            args.value_of("name")
                .expect("Could not read the name argument."),
        ))
    } else {
        Some("QinpelSrv".into())
    };
    let arg_host = if args.is_present("host") {
        Some(String::from(
            args.value_of("host")
                .expect("Could not read the host argument."),
        ))
    } else {
        None
    };
    let arg_port: Option<u64> = if args.is_present("port") {
        Some(
            args.value_of("port")
                .expect("Could not read the port argument.")
                .parse()
                .expect("Could not parse the port argument."),
        )
    } else {
        None
    };
    let arg_pubs = if args.is_present("pubs") {
        Some(true)
    } else {
        None
    };
    let arg_apps = if args.is_present("apps") {
        Some(true)
    } else {
        None
    };
    let arg_dirs = if args.is_present("dirs") {
        Some(true)
    } else {
        None
    };
    let arg_cmds = if args.is_present("cmds") {
        Some(true)
    } else {
        None
    };
    let arg_regs = if args.is_present("regs") {
        Some(true)
    } else {
        None
    };
    let arg_sqls = if args.is_present("sqls") {
        Some(true)
    } else {
        None
    };
    let arg_lizs = if args.is_present("lizs") {
        Some(true)
    } else {
        None
    };
    let server = QinServer {
        verbose: arg_verbose,
        archive: arg_archive,
        server_name: arg_name,
        server_host: arg_host,
        server_port: arg_port,
        serves_pubs: arg_pubs,
        serves_apps: arg_apps,
        serves_dirs: arg_dirs,
        serves_cmds: arg_cmds,
        serves_regs: arg_regs,
        serves_sqls: arg_sqls,
        serves_lizs: arg_lizs,
        redirects: None,
    };
    qinpel_srv::start(server).await
}
