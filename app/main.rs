use teleport::*;

use std::path::Path;

use clap::{App, Arg};
use failure::Error;

fn main() -> Result<(), Error> {
    let matches = App::new("teleport-rs")
        .about("Teleport around your filesystem")
        .author("Thales Macedo Garitezi")
        .bin_name("tp")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("cfg")
                .help("portals configurations")
                .takes_value(true)
                .required(false),
        )
        .subcommand(
            App::new("add")
                .about("adds a new portal")
                .arg(Arg::with_name("portal name").required(true).index(1))
                .arg(Arg::with_name("destination").required(true).index(2)),
        )
        .subcommand(
            App::new("go")
                .about("teleport to a portal")
                .arg(Arg::with_name("portal").required(true)),
        )
        .subcommand(
            App::new("rm")
                .about("remove a portal")
                .arg(Arg::with_name("portal").required(true)),
        )
        .subcommand(App::new("list").about("lists known portals"))
        .get_matches();

    let mut mhome = dirs::home_dir();
    let cfg_file = if let Some(ref cfg_path) = matches.value_of("config") {
        cfg_path
    } else {
        let default_path: &str = match mhome {
            Some(ref mut pb) => {
                pb.push(".config");
                pb.push("tprc");
                pb.to_str().unwrap()
            }
            None => panic!("could not find user home"),
        };
        default_path
    };

    let mut cfg: PortalConfig = load_portals(Path::new(cfg_file))?;

    if let Some(matches) = matches.subcommand_matches("go") {
        let alias = matches.value_of("portal").unwrap();
        if let Some(path) = cfg.portals.get(alias) {
            println!("{}", path);
            std::process::exit(2);
        } else {
            eprintln!("no such portal: {}", alias);
            std::process::exit(1);
        };
    } else if let Some(matches) = matches.subcommand_matches("add") {
        let alias = matches.value_of("portal name").unwrap();
        if alias.is_empty() {
            eprintln!("portal alias must not be empty");
            std::process::exit(1);
        }
        let destination = Path::new(matches.value_of("destination").unwrap());
        if destination.exists() && destination.is_dir() {
            let destination = destination.canonicalize()?.to_str().unwrap().to_string();
            cfg.add_portal(alias, &destination);
            let new_blob = serde_json::to_string(&cfg)?;
            std::fs::write(cfg_file, new_blob)?;
        } else if !destination.exists() {
            eprintln!("destination {} does not exist!", destination.display());
            std::process::exit(1);
        } else if !destination.is_dir() {
            eprintln!("destination {} is not a directory!", destination.display());
            std::process::exit(1);
        }
    } else if let Some(matches) = matches.subcommand_matches("rm") {
        let alias = matches.value_of("portal").unwrap();
        cfg.del_portal(alias);
        let new_blob = serde_json::to_string(&cfg)?;
        std::fs::write(cfg_file, new_blob)?;
    } else if let Some(_) = matches.subcommand_matches("list") {
        let total_portals = cfg.portals.len();
        if total_portals == 0 {
            println!("No portals!");
        } else {
            println!("Known portals ({} total):", cfg.portals.len());
            let largest_alias = cfg.portals.keys().max_by_key(|a| a.len()).unwrap().len();

            for (alias, dst) in cfg.portals {
                println!("{:<width$}\t{}", alias, dst, width = largest_alias);
            }
        }
    };

    Ok(())
}
