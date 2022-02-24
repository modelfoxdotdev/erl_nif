fn main() {
	let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
	if target_os == "macos" {
		macos_setup();
	}
}

fn macos_setup() {
	println!("cargo:rustc-cdylib-link-arg=-undefined");
	println!("cargo:rustc-cdylib-link-arg=dynamic_lookup");
}
