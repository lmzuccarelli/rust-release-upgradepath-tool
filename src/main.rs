use clap::Parser;
use custom_logger::*;
use semver::Version;
use std::fs;
use std::path::Path;

mod api;
mod calculate;
mod isc;
mod request;

use api::schema::*;
use calculate::upgradepath::*;
use isc::generate::*;
use request::graphdata::*;

// main entry point (use async)
#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let args_check: Vec<_> = std::env::args().collect();
    // 4 args from-version to-version channel arch
    if args_check.len() < 4 {
        eprintln!("Usage: rust-release-upgradepath-tool --help");
        std::process::exit(1);
    }

    let from_version = args.from_version.to_string();
    let to_version = args.to_version.to_string();

    if !Version::parse(&from_version).is_ok() || !Version::parse(&to_version).is_ok() {
        eprintln!("ensure from-version and to-version are valid semver versions");
        std::process::exit(1);
    }

    let arch = args.arch.to_string();
    let channel = args.channel.to_string();
    let level = args.loglevel.unwrap().to_string();

    // convert to enum
    let res_log_level = match level.as_str() {
        "info" => Level::INFO,
        "debug" => Level::DEBUG,
        "trace" => Level::TRACE,
        _ => Level::INFO,
    };

    // setup logging
    let log = &Logging {
        log_level: res_log_level,
    };

    log.info(&format!("from_version: {}", from_version));
    log.info(&format!("to_version: {}", to_version));
    log.info(&format!("arch: {:#?}", arch.clone()));

    let file_name = format!("{}_{}.json", channel, arch);
    let json_data: String;

    // first check if we has this json on disk
    if Path::new(&file_name.clone()).exists() {
        json_data =
            fs::read_to_string(format!("{}_{}.json", channel, arch)).expect("unable to read file");
    } else {
        let url = format!(
        "https://api.openshift.com/api/upgrades_info/v1/graph?arch={}&channel={}&id=dfb7d530-e876-425b-80b7-374ba5800525&version={}",arch,channel,from_version);

        // setup the request interface
        let g_con = ImplUpgradePathInterface {};
        json_data = g_con.get_graphdata(url.clone()).await.unwrap();

        // we can now save the json to file
        fs::write(file_name, json_data.clone()).expect("unable to write file");
    }

    // parse and calculate the upgrade path
    let graphdata = parse_json_graphdata(json_data.clone()).unwrap();
    let images = get_upgrade_path(log, from_version, to_version, graphdata);
    let v2_yml = IscV2Alpha1::new().to_yaml(channel, images.clone());
    log.debug(&format!("{}", v2_yml.clone()));
    let v3_yml = IscV3Alpha1::new().to_yaml(images.clone());
    log.debug(&format!("{}", v3_yml.clone()));
}
