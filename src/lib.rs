use custard_use::{dylib_management::safe_library::core_crate::CompositionFunctionReturn, utils::files::get_maybe_const_string};

#[no_mangle]
pub fn composition() -> CompositionFunctionReturn {
	let ret = get_maybe_const_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/composition.ron"), include_str!("composition.ron"));
	println!("fetched composition with status {}: {}", ret.1, ret.0);
	ret.0
}
