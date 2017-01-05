/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::env;
use std::path::{Path, PathBuf};
use std::io::prelude::*;
use std::fs::{canonicalize, read_dir, File};

fn write_shaders(glsl_files: Vec<PathBuf>, shader_file_path: &Path) {
    let mut shader_file = File::create(shader_file_path).unwrap();

    write!(shader_file, "/// AUTO GENERATED BY build.rs\n\n").unwrap();
    write!(shader_file, "use std::collections::HashMap;\n").unwrap();
    write!(shader_file, "lazy_static! {{\n").unwrap();
    write!(shader_file, "  pub static ref SHADERS: HashMap<&'static str, &'static str> = {{\n").unwrap();
    write!(shader_file, "    let mut h = HashMap::with_capacity({});\n", glsl_files.len()).unwrap();
    for glsl in glsl_files {
        let shader_name = glsl.file_name().unwrap().to_str().unwrap();
        // strip .glsl
        let shader_name = shader_name.replace(".glsl", "");
        let full_path = canonicalize(&glsl).unwrap();
        let full_name = full_path.as_os_str().to_str().unwrap();
        // if someone is building on a network share, I'm sorry.
        let full_name = full_name.replace("\\\\?\\", "");
        let full_name = full_name.replace("\\", "/");
        write!(shader_file, "    h.insert(\"{}\", include_str!(\"{}\"));\n",
               shader_name, full_name).unwrap();
    }
    write!(shader_file, "    h\n").unwrap(); 
    write!(shader_file, "  }};\n").unwrap(); 
    write!(shader_file, "}}\n").unwrap(); 
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap_or("out".to_owned());

    let shaders_file = Path::new(&out_dir).join("shaders.rs");
    let mut glsl_files = vec![];

    println!("cargo:rerun-if-changed=res");
    let res_dir = Path::new("res");
    for entry in read_dir(res_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if entry.file_name().to_str().unwrap().ends_with(".glsl") {
            glsl_files.push(path.to_owned());
        }
    }

    write_shaders(glsl_files, &shaders_file);
}
