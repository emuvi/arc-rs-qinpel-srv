use clap::{App, Arg, ArgMatches};

pub fn parse<'a>() -> ArgMatches<'a> {
  App::new("QinpelSrv")
    .version(clap::crate_version!())
    .author("Éverton M. Vieira <everton.muvi@gmail.com>")
    .about("QinpelSrv - WebServer for Qinpel")
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
        .short("nr")
        .long("no-run")
        .takes_value(false)
        .required(false)
        .help("Should I exit without serving?"),
    )
    .get_matches()
}
