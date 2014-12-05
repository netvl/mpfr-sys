#![feature(if_let)]

use std::os;
use std::io::{mod, fs, Command, BufReader};
use std::io::process::InheritFd;
use std::io::fs::PathExtensions;

const MPFR_NAME: &'static str = "libmpfr";
const MPFR_VERSION: &'static str = "3.1.2";

#[cfg(unix)]
fn check_library(name: &str) -> bool {
    // First check whether ldconfig utility is available (if we're on linux)
    if let Ok(po) = Command::new("ldcoig").arg("-p").output() {
        let target = os::getenv("TARGET").unwrap();
        let is_64bit = target.contains("x86_64");
        if po.output.len() > 0 {
            let mut br = BufReader::new(&*po.output);
            return br.lines().map(|l| l.unwrap())
                .any(|l| l.contains(name) && if is_64bit { l.contains("x86-64") }
                                             else { true })
        }
    }

    // If it fails, then check common system libraries directories
    for &dir in ["/lib", "/usr/lib", "/usr/local/lib"].iter() {
        let p = Path::new(dir).join(format!("{}.so", MPFR_NAME));
        if p.exists() { return true; }
    }

    // Nothing found, build the lib from scratch
    false
}

// Windows does not have predefined locations with libraries, sorry
#[cfg(windows)]
fn check_library(name: &str) -> bool {
    false
}

fn main() {
    // MPFR does not support pkg-config :(
    // Try to guess its presence manually
    if check_library(MPFR_NAME) { return; }

    // Bind some useful paths

    let project_src_root = Path::new(os::getenv("CARGO_MANIFEST_DIR").unwrap());
    let mpfr_src_root = project_src_root.join([MPFR_NAME, "-", MPFR_VERSION].concat());

    let out_dir = Path::new(os::getenv("OUT_DIR").unwrap());

    let mpfr_build_dir = out_dir.join("build");

    let mpfr_out_dir = out_dir.join("out");
    let mpfr_out_lib_dir = mpfr_out_dir.join("lib");
    let mpfr_out_include_dir = mpfr_out_dir.join("include");

    // Do not rebuild libmpfr if it had already been built
    
    if !(mpfr_out_lib_dir.exists() && mpfr_out_lib_dir.join("libmpfr.a").exists() &&
         mpfr_out_include_dir.exists() && mpfr_out_include_dir.join("mpfr.h").exists()) {
        run_build(&mpfr_src_root, &mpfr_build_dir, 
                  &mpfr_out_dir, &mpfr_out_lib_dir, &mpfr_out_include_dir);
    }

    // TODO: Regenerate and update source file if we have bindgen, otherwise copy prebuilt source

    // Emit cargo config
    emit_cargo_config(&mpfr_out_lib_dir, &mpfr_out_include_dir);
}

fn run_build(mpfr_src_root: &Path,
             mpfr_build_dir: &Path,
             mpfr_out_dir: &Path,
             mpfr_out_lib_dir: &Path,
             mpfr_out_include_dir: &Path) {
    // let windows = target.contains("windows") || target.contains("mingw");
    //
    let target = os::getenv("TARGET").unwrap();

    let mut ldflags = os::getenv("LDFLAGS").unwrap_or(String::new());
    if let Some(gmp_libdir) = os::getenv("DEP_GMP_LIBDIR") {
        ldflags.push_str("-L");
        ldflags.push_str(&*gmp_libdir);
    }

    let mut cflags = os::getenv("CFLAGS").unwrap_or(String::new());
    cflags.push_str(" -ffunction-sections -fdata-sections");
    if target.contains("i686") {
        cflags.push_str(" -m32");
    } else if target.as_slice().contains("x86_64") {
        cflags.push_str(" -m64");
    }
    if !target.contains("i686") {
        cflags.push_str(" -fPIC");
    }    
    if let Some(gmp_include) = os::getenv("DEP_GMP_INCLUDE") {
        cflags.push_str("-I");
        cflags.push_str(&*gmp_include);
    }

    let _ = fs::rmdir_recursive(mpfr_build_dir);
    let _ = fs::rmdir_recursive(mpfr_out_dir);

    let _ = fs::mkdir_recursive(mpfr_out_lib_dir, io::USER_DIR);
    let _ = fs::mkdir_recursive(mpfr_out_include_dir, io::USER_DIR);
    fs::mkdir(mpfr_build_dir, io::USER_DIR).unwrap();

    let config_opts = vec![
        "--enable-shared=no".into_string() // TODO: why?
    ];

    // Run configure
    run(Command::new("sh")
                .env("CFLAGS", cflags)
                .tap_mut(|c| if !ldflags.is_empty() { c.env("LDFLAGS", &*ldflags); })
                .cwd(mpfr_build_dir)
                .arg("-c")
                .arg(format!(
                    "{} {}", 
                    mpfr_src_root.join("configure").display(),
                    config_opts.connect(" ")
                ).replace("C:\\", "/c/").replace("\\", "/")));

    // Run make
    run(Command::new(make())
       .arg(format!("-j{}", os::getenv("NUM_JOBS").unwrap()))
       .cwd(mpfr_build_dir));

    // Copy the library archive file
    let p1 = mpfr_build_dir.join("src/.libs/libmpfr.a");
    let p2 = mpfr_build_dir.join("src/.libs/libmpfr.lib");
    if p1.exists() {
        fs::rename(&p1, &mpfr_out_lib_dir.join("libmpfr.a")).unwrap();
    } else {
        fs::rename(&p2, &mpfr_out_lib_dir.join("libmpfr.a")).unwrap();
    }
    
    // Copy the single include file
    fs::copy(&mpfr_build_dir.join("src/mpfr.h"), &mpfr_out_include_dir.join("mpfr.h")).unwrap();
    fs::copy(&mpfr_build_dir.join("src/mpf2mpfr.h"), &mpfr_out_include_dir.join("mpf2mpfr.h")).unwrap();
}

fn emit_cargo_config(lib_dir: &Path, include_dir: &Path) {
    println!("cargo:rustc-flags=-L {} -l mpfr:static", lib_dir.display());
    println!("cargo:libdir={}", lib_dir.display());
    println!("cargo:include={}", include_dir.display());
}

fn make() -> &'static str {
    if cfg!(target_os = "freebsd") {"gmake"} else {"make"}
}

fn run(cmd: &mut Command) {
    println!("running: {}", cmd);
    assert!(cmd.stdout(InheritFd(1))
            .stderr(InheritFd(2))
            .status()
            .unwrap()
            .success());
}

trait Tap {
    fn tap(self, f: |Self| -> Self) -> Self;
    fn tap_mut(&mut self, f: |&mut Self|) -> &mut Self;
}

impl<T> Tap for T {
    #[inline]
    fn tap(self, f: |T| -> T) -> T { f(self) }

    #[inline]
    fn tap_mut(&mut self, f: |&mut T|) -> &mut T {
        f(self);
        self
    }
}
