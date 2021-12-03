fn main() {
    // will crash if is not defined at compile time
	env!("EMBEDDED_FILES_PATH");
	
	// rerun this build script if EMBEDDED_FILEs_PATH has changed since last build
    println!("cargo:rerun-if-env-changed=EMBEDDED_FILES_PATH");
}