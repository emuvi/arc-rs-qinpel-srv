use clap::{App, Arg, ArgMatches};

pub fn parse<'a>() -> ArgMatches<'a> {
    App::new("QinpelSrv")
    .version(env!("CARGO_PKG_VERSION"))
    .author("Éverton M. Vieira <everton.muvi@gmail.com>")
    .about("QinpelSrv ( Qinpel Server ) is a library and a command program that servers public files, graphical user interfaces, file system access with authorization, command programs dispatchs, databases queries and Liz functionality with scripts. It is the base of the Pointel platform and the backend of the Qinpel, the Quick Interface for Power Intelligence.")
    .arg(
      Arg::with_name("debug")
        .short("g")
        .long("debug")
        .takes_value(false)
        .required(false)
        .help("Should we print debug messages?"),
    )
    .arg(
      Arg::with_name("verbose")
        .short("v")
        .long("verbose")
        .takes_value(false)
        .required(false)
        .help("Should we print verbose messages?"),
    )
    .arg(
      Arg::with_name("name")
        .short("n")
        .long("name")
        .value_name("NAME")
        .takes_value(true)
        .required(false)
        .help("On behalf of what name should we serve?"),
    )
    .arg(
      Arg::with_name("host")
        .short("h")
        .long("host")
        .value_name("ADDRESS")
        .takes_value(true)
        .required(false)
        .help("On what host should we serve?"),
    )
    .arg(
      Arg::with_name("port")
        .short("p")
        .long("port")
        .value_name("NUMBER")
        .takes_value(true)
        .required(false)
        .help("On what port should we serve?"),
    )
    .arg(
      Arg::with_name("pubs")
        .short("u")
        .long("pubs")
        .takes_value(false)
        .required(false)
        .help("Should we serve applications?"),
    )
    .arg(
      Arg::with_name("apps")
        .short("a")
        .long("apps")
        .takes_value(false)
        .required(false)
        .help("Should we serve applications?"),
    )
    .arg(
      Arg::with_name("dirs")
        .short("d")
        .long("dirs")
        .takes_value(false)
        .required(false)
        .help("Should we serve directories?"),
    )
    .arg(
      Arg::with_name("cmds")
        .short("c")
        .long("cmds")
        .takes_value(false)
        .required(false)
        .help("Should we serve commands?"),
    )
    .arg(
      Arg::with_name("sqls")
        .short("s")
        .long("sqls")
        .takes_value(false)
        .required(false)
        .help("Should we serve SQL scripts?"),
    )
    .arg(
      Arg::with_name("lizs")
        .short("l")
        .long("lizs")
        .takes_value(false)
        .required(false)
        .help("Should we serve LIZ scripts?"),
    )
    .get_matches()
}
