use clap::{App, Arg, ArgMatches};

pub fn get_cli_matches() -> ArgMatches<'static> {
    App::new("Solana Info Database")
        .version("0.1.2")
        .author("Emin Mammadov")
        .about("Provides information about the Solana network")
        .arg(
            Arg::with_name("validators")
                .long("validators")
                .help("Number of validators"),
        )
        .arg(
            Arg::with_name("stakes")
                .long("stakes")
                .help("Number of stakes and total stake amount"),
        )
        .arg(
            Arg::with_name("top-validators")
                .long("top-validators")
                .help("Top 10 validators with the highest stakes"),
        )
        .arg(
            Arg::with_name("rpc-url")
                .long("rpc-url")
                .value_name("URL")
                .help("Specify the RPC URL to use")
                .takes_value(true),
        )
        .get_matches()
}