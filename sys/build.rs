fn main() {
	let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
	match target_os.as_str() {
		"macos" => {
			macos_setup();
		}
		_ => {}
	};
}

fn macos_setup() {
	println!("cargo:rustc-cdylib-link-arg=-Wl");
	println!("cargo:rustc-cdylib-link-arg=-undefined");
	println!("cargo:rustc-cdylib-link-arg=dynamic_lookup");
}

