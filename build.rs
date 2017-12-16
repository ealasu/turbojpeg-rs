use std::env;
use std::fs;
use std::process::Command;
use std::path::Path;

fn main() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let out_dir = env::var("OUT_DIR").unwrap();
    let build_dir = Path::new(&out_dir).join("turbojpeg");
    fs::create_dir_all(&build_dir).unwrap();
    let target = env::var("TARGET").unwrap();
    println!("target: {}", target);
    let src_dir = Path::new(manifest_dir).join("libjpeg-turbo");

    let lib_file = Path::new(&build_dir).join(".libs").join("libturbojpeg.a");
    //if lib_file.exists() {
    if true {
        println!("already built.");
    } else { 
        let status = Command::new("autoreconf")
            .arg("-fiv")
            .current_dir(&src_dir)
            .status()
            .expect("failed to run autoreconf");
        assert!(status.success());;

        let mut configure = Command::new("sh");
        configure.arg(src_dir.join("configure"));
        configure.current_dir(&build_dir);
        let mut cflags = "".to_string();

        let mut make = Command::new("make");
        make.current_dir(&build_dir);

        if target == "armv7-unknown-linux-gnueabihf" {
            let toolchain = "arm-linux-gnueabihf";

            configure.arg(format!("--host={}", toolchain));
            cflags.push_str(" -march=armv7-a -mfloat-abi=hard -fprefetch-loop-arrays");

            configure.env("CPP", format!("{}-cpp", toolchain));
            configure.env("AR", format!("{}-ar", toolchain));
            configure.env("NM", format!("{}-nm", toolchain));
            configure.env("CC", format!("{}-gcc", toolchain));
            configure.env("LD", format!("{}-ld", toolchain));
            configure.env("RANLIB", format!("{}-ranlib", toolchain));
            configure.env("OBJDUMP", format!("{}-objdump", toolchain));
            configure.env("STRIP", format!("{}-strip", toolchain));


            make.env("CPP", format!("{}-cpp", toolchain));
            make.env("AR", format!("{}-ar", toolchain));
            make.env("NM", format!("{}-nm", toolchain));
            make.env("CC", format!("{}-gcc", toolchain));
            make.env("LD", format!("{}-ld", toolchain));
            make.env("RANLIB", format!("{}-ranlib", toolchain));
            make.env("OBJDUMP", format!("{}-objdump", toolchain));
            make.env("STRIP", format!("{}-strip", toolchain));
        }
        if target == "x86_64-apple-darwin" {
            configure.arg(format!("--host={}", target));
            configure.arg("NASM=/usr/local/bin/nasm");
        }

        println!("cflags: {}", cflags);
        configure.arg(format!("CFLAGS={} -O3 -fPIE", cflags));
        configure.arg(format!("CPPFLAGS={}", cflags));
        configure.arg(format!("LDFLAGS={} -pie", cflags));
        configure.arg("--with-simd");
        configure.arg("--with-pic");
        let status = configure.status().expect("failed to run configure");
        assert!(status.success());

        let status = make.status().expect("failed to run make");
        assert!(status.success());
    }

    print_lib_dir();
    //println!("cargo:rustc-link-search={}/.libs", build_dir.to_string_lossy());
    println!("cargo:rustc-link-lib=static=turbojpeg");
}

#[cfg(not(target_os = "macos"))]
fn print_lib_dir() {
}

#[cfg(target_os = "macos")]
fn print_lib_dir() {
    let lib_dir = Command::new("brew")
        .arg("--prefix")
        .arg("libjpeg-turbo")
        //.arg("libtiff")
        .output()
        .ok()
        .into_iter()
        .filter(|output| output.status.success())
        .flat_map(|output| String::from_utf8(output.stdout).ok())
        .map(|output| format!("{}/lib", output.trim()))
        .next()
        .unwrap();
    println!("cargo:rustc-link-search={}", lib_dir);
}
