use clap::{App, ArgMatches};

pub fn get_cli_matches() -> ArgMatches<'static> {
    App::new("Solana Info Database")
        .version("0.1.2")
        .author("Emin Mammadov")
        .about("Provides information about the Solana network")
        .arg(
            clap::Arg::with_name("validators")
                .long("validators")
                .help("Number of validators"),
        )
        .arg(
            clap::Arg::with_name("stakes")
                .long("stakes")
                .help("Number of stakes and total stake amount"),
        )
        .arg(
            clap::Arg::with_name("top-validators")
                .long("top-validators")
                .help("Top 10 validators with the highest stakes"),
        )
        .get_matches()
}