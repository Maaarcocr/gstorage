fn main() {
    let google_cloud_cpp = vcpkg::find_package("google-cloud-cpp").unwrap();
    let mut compiler = cxx_build::bridge("src/lib.rs");
    compiler.cpp(true);
    compiler
        .file("gstorage-c/client.cc");
    for include_path in google_cloud_cpp.include_paths {
        compiler.include(include_path);
    }
    compiler.flag_if_supported("-std=c++14");
    compiler.include("gstorage-c");
    compiler.compile("gstorace-c");
    compiler.define("GOOGLE_CLOUD_CPP_STORAGE_ENABLE_GRPC", "ON");

    println!("cargo:rerun-if-changed=gstorage-c/client.hpp");
    println!("cargo:rerun-if-changed=gstorage-c/client.cc");
    println!("cargo:rustc-link-lib=framework=CoreFoundation");
    println!("cargo:rustc-link-lib=framework=SystemConfiguration");
    println!("cargo:rustc-link-lib=framework=Security");
    println!("cargo:rustc-link-lib=resolv");

}
