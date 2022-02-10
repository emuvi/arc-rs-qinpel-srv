mod clip;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = clip::parse();
    if args.is_present("no-run") {
        std::process::exit(0);
    }
    let arg_debug = if args.is_present("debug") {
        Some(true)
    } else {
        None
    };
    let arg_verbose = if args.is_present("verbose") {
        Some(true)
    } else {
        None
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
    let server = qinpel_srv::QinServer::new(
        arg_debug,
        arg_verbose,
        arg_host,
        arg_port,
        true,
        true,
        true,
        true,
        None,
    );
    qinpel_srv::start(server).await
}
