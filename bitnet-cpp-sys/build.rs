use cmake::Config;
use glob::glob;
use patch_apply::{apply, Patch};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use std::{env, fs};

macro_rules! debug_log {
    ($($arg:tt)*) => {
        if std::env::var("BUILD_DEBUG").is_ok() {
            println!("cargo:warning=[DEBUG] {}", format!($($arg)*));
        }
    };
}

// const CARGO_PKG_NAME: &str = env!("CARGO_PKG_NAME");
const CARGO_PKG_NAME: &str = "bitnet-cpp-sys"; // this has to be hardcoded to enable `bitnet-cpp` build
const CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
const BITNET_DIR: &str = "bitnet";
const LLAMA_CPP_DIR: &str = "bitnet/3rdparty/llama.cpp";
const PATCHES_DIR: &str = "patches";

#[cfg(target_os = "windows")]
const OS_EXTRA_ARGS: [(&str, &str); 1] = [("-T", "ClangCL")]; // these are cflags, so should be defined as .cflag("-foo")

#[cfg(target_os = "linux")]
const OS_EXTRA_ARGS: [(&str, &str); 2] = [
    ("CMAKE_C_COMPILER", "clang"),
    ("CMAKE_CXX_COMPILER", "clang++"),
];

#[cfg(target_arch = "aarch64")]
const COMPILER_EXTRA_ARGS: (&str, &str) = ("BITNET_ARM_TL1", "ON");

#[cfg(target_arch = "x86_64")]
const COMPILER_EXTRA_ARGS: (&str, &str) = ("BITNET_X86_TL2", "ON");

#[allow(dead_code)]
fn get_root_dir() -> String {
    match CARGO_MANIFEST_DIR.contains("/target/") {
        true => CARGO_MANIFEST_DIR.split("/target/").next().unwrap().into(),
        false => CARGO_MANIFEST_DIR.into(),
    }
}

#[allow(dead_code)]
fn get_src_dir() -> PathBuf {
    match CARGO_MANIFEST_DIR.contains("/target/") {
        true => {
            let path: PathBuf = CARGO_MANIFEST_DIR.split("/target/").next().unwrap().into();
            path.join(CARGO_PKG_NAME)
        }
        false => CARGO_MANIFEST_DIR.into(),
    }
}

#[allow(dead_code)]
fn get_out_dir() -> PathBuf {
    std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap())
}

#[allow(dead_code)]
fn run_shell(path: PathBuf) {
    let patches_dir = match CARGO_MANIFEST_DIR.contains("/target/") {
        true => {
            // when `cargo publish` the CARGO_MANIFEST_DIR returns `bitnet-cpp-rs/target/package/bitnet-cpp-sys-<version>`
            let dir = CARGO_MANIFEST_DIR.split("/target/").next().unwrap();
            format!("{dir}/{CARGO_PKG_NAME}")
        }
        false => CARGO_MANIFEST_DIR.into(),
    };
    let dir = std::path::PathBuf::from(patches_dir);
    let program = dir.join(path);
    // println!("cargo:warning=[DEBUG] {:?}", program);
    let mut child = Command::new(program).spawn().unwrap();
    child.wait().unwrap();
    sleep(Duration::from_secs(5));
}

fn get_cargo_target_dir() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let out_dir = get_out_dir();
    let profile = std::env::var("PROFILE")?;
    let mut target_dir = None;
    let mut sub_path = out_dir.as_path();
    while let Some(parent) = sub_path.parent() {
        if parent.ends_with(&profile) {
            target_dir = Some(parent);
            break;
        }
        sub_path = parent;
    }
    let target_dir = target_dir.ok_or("not found")?;
    Ok(target_dir.to_path_buf())
}

fn copy_folder(src: &Path, dst: &Path) {
    std::fs::create_dir_all(dst).expect("Failed to create dst directory");
    if cfg!(unix) {
        std::process::Command::new("cp")
            .arg("-rf")
            .arg(src)
            .arg(dst.parent().unwrap())
            .status()
            .expect("Failed to execute cp command");
    }

    if cfg!(windows) {
        std::process::Command::new("robocopy.exe")
            .arg("/e")
            .arg(src)
            .arg(dst)
            .status()
            .expect("Failed to execute robocopy command");
    }
}

fn extract_lib_names(out_dir: &Path, build_shared_libs: bool) -> Vec<String> {
    let lib_pattern = if cfg!(windows) {
        "*.lib"
    } else if cfg!(target_os = "macos") {
        if build_shared_libs {
            "*.dylib"
        } else {
            "*.a"
        }
    } else {
        if build_shared_libs {
            "*.so"
        } else {
            "*.a"
        }
    };
    let libs_dir = out_dir.join("lib");
    let pattern = libs_dir.join(lib_pattern);
    debug_log!("Extract libs {}", pattern.display());

    let mut lib_names: Vec<String> = Vec::new();

    // Process the libraries based on the pattern
    for entry in glob(pattern.to_str().unwrap()).unwrap() {
        match entry {
            Ok(path) => {
                let stem = path.file_stem().unwrap();
                let stem_str = stem.to_str().unwrap();

                // Remove the "lib" prefix if present
                let lib_name = if stem_str.starts_with("lib") {
                    stem_str.strip_prefix("lib").unwrap_or(stem_str)
                } else {
                    stem_str
                };
                lib_names.push(lib_name.to_string());
            }
            Err(e) => println!("cargo:warning=error={}", e),
        }
    }
    lib_names
}

fn extract_lib_assets(out_dir: &Path) -> Vec<PathBuf> {
    let shared_lib_pattern = if cfg!(windows) {
        "*.dll"
    } else if cfg!(target_os = "macos") {
        "*.dylib"
    } else {
        "*.so"
    };

    let libs_dir = out_dir.join("lib");
    let pattern = libs_dir.join(shared_lib_pattern);
    debug_log!("Extract lib assets {}", pattern.display());
    let mut files = Vec::new();

    for entry in glob(pattern.to_str().unwrap()).unwrap() {
        match entry {
            Ok(path) => {
                files.push(path);
            }
            Err(e) => eprintln!("cargo:warning=error={}", e),
        }
    }

    files
}

fn macos_link_search_path() -> Option<String> {
    let output = Command::new("clang")
        .arg("--print-search-dirs")
        .output()
        .ok()?;
    if !output.status.success() {
        println!(
            "failed to run 'clang --print-search-dirs', continuing without a link search path"
        );
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        if line.contains("libraries: =") {
            let path = line.split('=').nth(1)?;
            return Some(format!("{}/lib/darwin", path));
        }
    }

    println!("failed to determine link search path, continuing without it");
    None
}

fn build() {
    let target = env::var("TARGET").unwrap();
    let out_dir = get_out_dir();
    let src_dir = get_src_dir();

    let target_dir = get_cargo_target_dir().unwrap();

    let bitnet_dst = out_dir.join(BITNET_DIR);
    let bitnet_src = Path::new(&CARGO_MANIFEST_DIR).join(BITNET_DIR);

    let patches_dst = out_dir.join(PATCHES_DIR);
    let patches_src = Path::new(&src_dir).join(PATCHES_DIR);

    let build_shared_libs = cfg!(feature = "cuda") || cfg!(feature = "dynamic-link");

    let build_shared_libs = std::env::var("LLAMA_BUILD_SHARED_LIBS")
        .map(|v| v == "1")
        .unwrap_or(build_shared_libs);
    let profile = env::var("LLAMA_LIB_PROFILE").unwrap_or("Release".to_string());
    let static_crt = env::var("LLAMA_STATIC_CRT")
        .map(|v| v == "1")
        .unwrap_or(false);

    debug_log!("TARGET: {}", target);
    debug_log!("CARGO_MANIFEST_DIR: {}", CARGO_MANIFEST_DIR);
    debug_log!("TARGET_DIR: {}", target_dir.display());
    debug_log!("OUT_DIR: {}", out_dir.display());
    debug_log!("BUILD_SHARED: {}", build_shared_libs);

    if !patches_dst.exists() {
        debug_log!(
            "Copy {} to {}",
            patches_src.display(),
            patches_dst.display()
        );
        copy_folder(&patches_src, &patches_dst);
    }

    if !bitnet_dst.exists() {
        debug_log!("Copy {} to {}", bitnet_src.display(), bitnet_dst.display());
        copy_folder(&bitnet_src, &bitnet_dst);
        // applies git patches when folder is copied, run `cargo clean` to run it again
        apply_patches();
    }

    // Speed up build
    env::set_var(
        "CMAKE_BUILD_PARALLEL_LEVEL",
        std::thread::available_parallelism()
            .unwrap()
            .get()
            .to_string(),
    );

    // Bindings
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate_comments(true)
        .clang_arg("-xc++")
        .clang_arg("-std=c++11")
        .clang_arg(format!("-I{}", bitnet_dst.join("include").display()))
        .clang_arg(format!(
            "-I{}",
            bitnet_dst.join("3rdparty/llama.cpp/include").display()
        ))
        .clang_arg(format!(
            "-I{}",
            bitnet_dst.join("3rdparty/llama.cpp/ggml/include").display()
        ))
        // .clang_arg("-std=c++14")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .derive_partialeq(true)
        .allowlist_function("ggml_.*")
        .allowlist_type("ggml_.*")
        .allowlist_function("llama_.*")
        .allowlist_type("llama_.*")
        .allowlist_item("LLAMA_.*")
        .use_core()
        .prepend_enum_name(false)
        .generate()
        .expect("Failed to generate bindings");

    // Write the generated bindings to an output file
    let bindings_path = out_dir.join("bindings.rs");
    bindings
        .write_to_file(bindings_path)
        .expect("Failed to write bindings");

    println!("cargo:rerun-if-changed=wrapper.h");

    debug_log!("Bindings Created");

    // Build with Cmake
    let mut config = Config::new(&bitnet_dst);

    config.out_dir(bitnet_dst.parent().unwrap());

    // Would require extra source files to pointlessly
    // be included in what's uploaded to and downloaded from
    // crates.io, so deactivating these instead
    config.define("LLAMA_BUILD_TESTS", "OFF");
    config.define("LLAMA_BUILD_EXAMPLES", "OFF");
    config.define("LLAMA_BUILD_SERVER", "OFF");

    #[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
    {
        config.define(COMPILER_EXTRA_ARGS.0, COMPILER_EXTRA_ARGS.1);
        debug_log!(
            "COMPILER_EXTRA_ARGS: {}={}",
            COMPILER_EXTRA_ARGS.0,
            COMPILER_EXTRA_ARGS.1
        );
    }

    config.define(
        "BUILD_SHARED_LIBS",
        if build_shared_libs { "ON" } else { "OFF" },
    );

    if cfg!(target_os = "macos") {
        config.define("GGML_BLAS", "OFF");
    }

    if cfg!(all(target_os = "macos", feature = "metal")) {
        config.define("GGML_METAL", "ON");
    } else {
        config.define("GGML_METAL", "OFF");
    }

    if cfg!(windows) {
        config.static_crt(static_crt);
    }

    if cfg!(feature = "vulkan") {
        config.define("GGML_VULKAN", "ON");
        if cfg!(windows) {
            let vulkan_path = env::var("VULKAN_SDK")
                .expect("Please install Vulkan SDK and ensure that VULKAN_SDK env variable is set");
            let vulkan_lib_path = Path::new(&vulkan_path).join("Lib");
            println!("cargo:rustc-link-search={}", vulkan_lib_path.display());
            println!("cargo:rustc-link-lib=vulkan-1");
        }

        if cfg!(target_os = "linux") {
            println!("cargo:rustc-link-lib=vulkan");
        }
    }

    if cfg!(feature = "cuda") {
        config.define("GGML_CUDA", "ON");
    }

    if cfg!(feature = "openmp") {
        config.define("GGML_OPENMP", "ON");
    } else {
        config.define("GGML_OPENMP", "OFF");
    }

    // General
    config
        .profile(&profile)
        .very_verbose(std::env::var("CMAKE_VERBOSE").is_ok()) // Not verbose by default
        .always_configure(false);

    // cmake --build build --config Release
    config.profile("Release");
    let build_dir = config.build(); // breaks

    // Search paths
    println!("cargo:rustc-link-search={}", out_dir.join("lib").display());
    println!("cargo:rustc-link-search={}", build_dir.display());

    // Link libraries
    let llama_libs_kind = if build_shared_libs { "dylib" } else { "static" };
    let llama_libs = extract_lib_names(&out_dir, build_shared_libs);

    for lib in llama_libs {
        debug_log!(
            "LINK {}",
            format!("cargo:rustc-link-lib={}={}", llama_libs_kind, lib)
        );
        println!(
            "{}",
            format!("cargo:rustc-link-lib={}={}", llama_libs_kind, lib)
        );
    }

    // OpenMP
    if cfg!(feature = "openmp") {
        if target.contains("gnu") {
            println!("cargo:rustc-link-lib=gomp");
        }
    }

    // Windows debug
    if cfg!(all(debug_assertions, windows)) {
        println!("cargo:rustc-link-lib=dylib=msvcrtd");
    }

    // macOS
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=framework=Foundation");
        println!("cargo:rustc-link-lib=framework=Metal");
        println!("cargo:rustc-link-lib=framework=MetalKit");
        println!("cargo:rustc-link-lib=framework=Accelerate");
        println!("cargo:rustc-link-lib=c++");
    }

    // Linux
    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    }

    if target.contains("apple") {
        // On (older) OSX we need to link against the clang runtime,
        // which is hidden in some non-default path.
        //
        // More details at https://github.com/alexcrichton/curl-rust/issues/279.
        if let Some(path) = macos_link_search_path() {
            println!("cargo:rustc-link-lib=clang_rt.osx");
            println!("cargo:rustc-link-search={}", path);
        }
    }

    // copy DLLs to target
    if build_shared_libs {
        let libs_assets = extract_lib_assets(&out_dir);
        for asset in libs_assets {
            let asset_clone = asset.clone();
            let filename = asset_clone.file_name().unwrap();
            let filename = filename.to_str().unwrap();
            let dst = target_dir.join(filename);
            debug_log!("HARD LINK {} TO {}", asset.display(), dst.display());
            if !dst.exists() {
                std::fs::hard_link(asset.clone(), dst).unwrap();
            }

            // Copy DLLs to examples as well
            if target_dir.join("examples").exists() {
                let dst = target_dir.join("examples").join(filename);
                debug_log!("HARD LINK {} TO {}", asset.display(), dst.display());
                if !dst.exists() {
                    std::fs::hard_link(asset.clone(), dst).unwrap();
                }
            }

            // Copy DLLs to target/profile/deps as well for tests
            let dst = target_dir.join("deps").join(filename);
            debug_log!("HARD LINK {} TO {}", asset.display(), dst.display());
            if !dst.exists() {
                std::fs::hard_link(asset.clone(), dst).unwrap();
            }
        }
    }
}

fn apply_patch(patch_name: &str, output_dir: &str) {
    // let src_dir = get_src_dir();
    let out_dir = get_out_dir();

    // let patches_dir = src_dir.join("patches");
    let patches_dir = out_dir.join(PATCHES_DIR);

    let content = fs::read_to_string(patches_dir.join(patch_name)).unwrap();
    // uncomment this if you want to see atomic commits in the main git repo
    // let root = src_dir.join(output_dir);
    let root = out_dir.join(output_dir);

    let patches: Vec<Patch<'_>> = Patch::from_multiple(&content).unwrap();
    patches.iter().for_each(|patch| {
        let path = patch.new.path.to_string().replace("b/", ""); // "b/ggml/CMakeLists.txt" -> "ggml/CMakeLists.txt"
        let path = root.join(path);
        // println!("cargo:warning=[DEBUG] {:?}", path);
        let old_content = match fs::read_to_string(path.clone()) {
            Ok(content) => content,
            Err(_) => "".into(),
        };
        let patched_content = apply(old_content, patch.clone());
        fs::write(path, patched_content).unwrap();
    });
}

fn apply_patches() {
    apply_patch("llama.cpp.patch", LLAMA_CPP_DIR);
    apply_patch("bitnet.patch", BITNET_DIR);
}

fn main() {
    // TODO: apply patches on the features level of architecture and quantization type
    // run_shell("patches/apply.sh".into());
    // apply_patches();
    build();
    // run_shell("patches/clean.sh".into());
}
