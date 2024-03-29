use std::fs::remove_dir_all;
use npm_rs::*;

fn main() {
	std::fs::remove_dir_all("spa/build");

	NpmEnv::default()
		.init()
		.install(None)
		.run("build")
		.exec()
		.expect("Failed to run npm build script");

	println!("cargo:rerun-if-changed=svelte/");
}