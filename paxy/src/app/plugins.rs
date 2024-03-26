use std::{
    fs::{write, File},
    path::{Path, PathBuf},
};

use bson::{doc, Document};
use extism::{Manifest, PluginBuilder, Wasm};

use crate::{actions::ensure_path, home};

#[allow(unused)]
pub(crate) fn plugin(manifest: Box<Path>) -> Wasm {
    let mut file = home!();
    file.push(".paxy");
    file.push("plugins");
    ensure_path(Some(&file));
    file.push("plugins.bson");
    let plugins = if !file.is_file() {
        let mut buf = vec![];
        let doc = doc! {"pax": "paxy.wasm"};
        doc.to_writer(&mut buf)
            .unwrap();
        write(file, buf).unwrap();
        doc
    } else {
        Document::from_reader(File::open(file).unwrap()).unwrap()
    };
    let plugin = plugins
        .get(
            manifest
                .extension()
                .expect("unknown manifest type")
                .to_str()
                .unwrap(),
        )
        .unwrap()
        .to_string();
    Wasm::file(plugin)
}

pub fn call_plugin(wasm: Wasm, pkg: Box<PathBuf>) {
    let mut tmp = home!();
    tmp.push(".paxy");
    tmp.push("tmp");
    ensure_path(Some(&tmp));
    let manifest = Manifest::new([wasm]).with_allowed_paths(
        [(tmp, PathBuf::from("/tmp")), (*pkg, PathBuf::from("/pkg"))]
            .iter()
            .cloned(),
    );
    let plugin = PluginBuilder::new(manifest).with_wasi(true);
    let mut plugin = plugin
        .build()
        .unwrap();
    plugin.call::<&str, &str>("process", "").unwrap();
}
