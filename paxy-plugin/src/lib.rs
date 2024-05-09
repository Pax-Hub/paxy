use std::{fs, process::Command};

use extism_pdk::{plugin_fn, FnResult, Json};
use semver::{Version, VersionReq};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Pkg {
    name: String,
    repo: String,
    version: Vec<Ver>,
}

#[derive(Debug, Deserialize)]
struct Ver {
    ver: Version,
    dependencies: Vec<VersionReq>,
    src: String,
    install_inst: String,
}

fn parse_file() -> Pkg {
    toml::from_str(
        fs::read_to_string("/pkg")
            .unwrap()
            .as_str(),
    )
    .expect("invalid file")
}

fn exec_inst(commands: String) {
    for stmt in commands.split(';') {
        let mut args = stmt
            .split(' ')
            .collect::<Vec<&str>>();
        let mut cmd = Command::new(
            args.get(0)
                .expect("malformed command"),
        );
        args.remove(0);
        cmd.args(args);
        let mut handle = match cmd.spawn() {
            Ok(c) => c,
            Err(_) => panic!("Illegal expression: {stmt}"),
        };
        handle
            .wait()
            .unwrap();
    }
}

#[plugin_fn]
pub fn prep(req: Json<Version>) -> FnResult<String> {
    let pkg = parse_file();
    let ver;
    let req = req.0;
    for version in pkg.version {
        if version.ver == req {
            ver = version;
            break;
        }
    }
    todo!()
}
