use clap::{Arg, ArgMatches, Command};

pub fn parse() -> ArgMatches {
    Command::new("QinpelSrv")
    .version(clap::crate_version!())
    .about("QinpelSrv (Qinpel Server) is a library and a command program that servers public files, graphical user interfaces, file system access with authorization, command programs dispatchs, databases queries and Liz functionality with scripts. It is the base of the Pointel platform and the backend of the Qinpel, the Quick Interface for Power Intelligence.")
    .author("Éverton M. Vieira <everton.muvi@gmail.com>")
    .arg(
      Arg::new("verbose")
        .short('v')
        .long("verbose")
        .takes_value(false)
        .required(false)
        .help("Should we print verbose messages?"),
    )
    .arg(
      Arg::new("archive")
        .short('a')
        .long("archive")
        .takes_value(false)
        .required(false)
        .help("Should we archive all the messages?"),
    )
    .arg(
      Arg::new("name")
        .short('n')
        .long("name")
        .value_name("NAME")
        .takes_value(true)
        .required(false)
        .help("On behalf of what name should we serve?"),
    )
    .arg(
      Arg::new("host")
        .short('h')
        .long("host")
        .value_name("ADDRESS")
        .takes_value(true)
        .required(false)
        .help("On what host should we serve?"),
    )
    .arg(
      Arg::new("port")
        .short('p')
        .long("port")
        .value_name("NUMBER")
        .takes_value(true)
        .required(false)
        .help("On what port should we serve?"),
    )
    .arg(
      Arg::new("pubs")
        .short('u')
        .long("pubs")
        .takes_value(false)
        .required(false)
        .help("Should we serve applications?"),
    )
    .arg(
      Arg::new("apps")
        .short('a')
        .long("apps")
        .takes_value(false)
        .required(false)
        .help("Should we serve applications?"),
    )
    .arg(
      Arg::new("dirs")
        .short('d')
        .long("dirs")
        .takes_value(false)
        .required(false)
        .help("Should we serve directories?"),
    )
    .arg(
      Arg::new("cmds")
        .short('c')
        .long("cmds")
        .takes_value(false)
        .required(false)
        .help("Should we serve commands?"),
    )
    .arg(
      Arg::new("sqls")
        .short('s')
        .long("sqls")
        .takes_value(false)
        .required(false)
        .help("Should we serve SQL scripts?"),
    )
    .arg(
      Arg::new("lizs")
        .short('l')
        .long("lizs")
        .takes_value(false)
        .required(false)
        .help("Should we serve LIZ scripts?"),
    )
    .get_matches()
}
