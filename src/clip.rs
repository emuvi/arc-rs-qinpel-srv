use clap::{App, Arg, ArgMatches};

pub fn run<'a>() -> ArgMatches<'a> {
  App::new("QinpelSrv")
    .version(clap::crate_version!())
    .author("Éverton M. Vieira <everton.muvi@gmail.com>")
    .about("QinpelSrv - QinpelSrv - WebServer for Qinpel")
    .arg(
      Arg::with_name("host")
        .short("h")
        .long("host")
        .value_name("ADDRESS")
        .takes_value(true)
        .required(false)
        .help("What app or cmd should I install?"),
    )
    .arg(
      Arg::with_name("port")
        .short("p")
        .long("port")
        .value_name("NUMBER")
        .takes_value(true)
        .required(false)
        .help("How long should I wait before to execute?"),
    )
    .arg(
      Arg::with_name("run")
        .short("r")
        .long("run")
        .takes_value(false)
        .required(false)
        .help("What app or cmd should I install?"),
    )
    .get_matches()
}
