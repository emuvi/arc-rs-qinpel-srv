use clap::{App, Arg, ArgMatches};

pub fn parse<'a>() -> ArgMatches<'a> {
  App::new("QinpelSrv")
    .version(clap::crate_version!())
    .author("Éverton M. Vieira <everton.muvi@gmail.com>")
    .about("Qinpel Server - Web server of graphical user and command line interfaces, file system and database source operations for the Qinpel (Quick Interface for Pointel) Platform.")
    .arg(
      Arg::with_name("host")
        .short("h")
        .long("host")
        .value_name("ADDRESS")
        .takes_value(true)
        .required(false)
        .help("On what host should I serve?"),
    )
    .arg(
      Arg::with_name("port")
        .short("p")
        .long("port")
        .value_name("NUMBER")
        .takes_value(true)
        .required(false)
        .help("On what port should I serve?"),
    )
    .arg(
      Arg::with_name("no-run")
        .short("x")
        .long("no-run")
        .takes_value(false)
        .required(false)
        .help("Should I exit without serving?"),
    )
    .get_matches()
}
