//
// EVERYTHING BELOW THIS POINT WAS AUTO-GENERATED DURING COMPILATION. DO NOT MODIFY.
//
#[doc=r#"The Continuous Integration platform detected during compilation."#]
#[allow(dead_code)]
pub const CI_PLATFORM: Option<&str> = None;
#[doc=r#"The full version."#]
#[allow(dead_code)]
pub const PKG_VERSION: &str = r"0.1.0";
#[doc=r#"The major version."#]
#[allow(dead_code)]
pub const PKG_VERSION_MAJOR: &str = r"0";
#[doc=r#"The minor version."#]
#[allow(dead_code)]
pub const PKG_VERSION_MINOR: &str = r"1";
#[doc=r#"The patch version."#]
#[allow(dead_code)]
pub const PKG_VERSION_PATCH: &str = r"0";
#[doc=r#"The pre-release version."#]
#[allow(dead_code)]
pub const PKG_VERSION_PRE: &str = r"";
#[doc=r#"A colon-separated list of authors."#]
#[allow(dead_code)]
pub const PKG_AUTHORS: &str = r"Lukas Lueg <lukas.lueg@gmail.com>";
#[doc=r#"The name of the package."#]
#[allow(dead_code)]
pub const PKG_NAME: &str = r"macro_railroad_ext";
#[doc=r#"The description."#]
#[allow(dead_code)]
pub const PKG_DESCRIPTION: &str = r"";
#[doc=r#"The homepage."#]
#[allow(dead_code)]
pub const PKG_HOMEPAGE: &str = r"";
#[doc=r#"The license."#]
#[allow(dead_code)]
pub const PKG_LICENSE: &str = r"";
#[doc=r#"The source repository as advertised in Cargo.toml."#]
#[allow(dead_code)]
pub const PKG_REPOSITORY: &str = r"";
#[doc=r#"The target triple that was being compiled for."#]
#[allow(dead_code)]
pub const TARGET: &str = r"wasm32-unknown-unknown";
#[doc=r#"The host triple of the rust compiler."#]
#[allow(dead_code)]
pub const HOST: &str = r"x86_64-unknown-linux-gnu";
#[doc=r#"`release` for release builds, `debug` for other builds."#]
#[allow(dead_code)]
pub const PROFILE: &str = r"release";
#[doc=r#"The compiler that cargo resolved to use."#]
#[allow(dead_code)]
pub const RUSTC: &str = r"rustc";
#[doc=r#"The documentation generator that cargo resolved to use."#]
#[allow(dead_code)]
pub const RUSTDOC: &str = r"rustdoc";
#[doc=r#"Value of OPT_LEVEL for the profile used during compilation."#]
#[allow(dead_code)]
pub const OPT_LEVEL: &str = r"s";
#[doc=r#"The parallelism that was specified during compilation."#]
#[allow(dead_code)]
pub const NUM_JOBS: u32 = 4;
#[doc=r#"Value of DEBUG for the profile used during compilation."#]
#[allow(dead_code)]
pub const DEBUG: bool = false;
#[doc=r#"The features that were enabled during compilation."#]
#[allow(dead_code)]
pub const FEATURES: [&str; 0] = [];
#[doc=r#"The features as a comma-separated string."#]
#[allow(dead_code)]
pub const FEATURES_STR: &str = r"";
#[doc=r#"The output of `rustc -V`"#]
#[allow(dead_code)]
pub const RUSTC_VERSION: &str = r"rustc 1.64.0 (a55dd71d5 2022-09-19)";
#[doc=r#"The output of `rustdoc -V`"#]
#[allow(dead_code)]
pub const RUSTDOC_VERSION: &str = r"rustdoc 1.64.0 (a55dd71d5 2022-09-19)";
#[doc=r#"An array of effective dependencies as documented by `Cargo.lock`."#]
#[allow(dead_code)]
pub const DEPENDENCIES: [(&str, &str); 58] = [("android_system_properties", "0.1.5"), ("autocfg", "1.1.0"), ("built", "0.5.1"), ("bumpalo", "3.11.0"), ("cargo-lock", "7.1.0"), ("cc", "1.0.73"), ("cfg-if", "1.0.0"), ("chrono", "0.4.22"), ("codespan-reporting", "0.11.1"), ("console_error_panic_hook", "0.1.7"), ("core-foundation-sys", "0.8.3"), ("cxx", "1.0.78"), ("cxx-build", "1.0.78"), ("cxxbridge-flags", "1.0.78"), ("cxxbridge-macro", "1.0.78"), ("form_urlencoded", "1.1.0"), ("htmlescape", "0.3.1"), ("iana-time-zone", "0.1.51"), ("iana-time-zone-haiku", "0.1.0"), ("idna", "0.3.0"), ("js-sys", "0.3.60"), ("libc", "0.2.135"), ("link-cplusplus", "1.0.7"), ("log", "0.4.17"), ("macro_railroad", "0.1.3"), ("macro_railroad_ext", "0.1.0"), ("num-integer", "0.1.45"), ("num-traits", "0.2.15"), ("once_cell", "1.15.0"), ("percent-encoding", "2.2.0"), ("proc-macro2", "1.0.46"), ("quote", "1.0.21"), ("railroad", "0.1.1"), ("scratch", "1.0.2"), ("semver", "1.0.14"), ("serde", "1.0.145"), ("serde_derive", "1.0.145"), ("syn", "1.0.102"), ("termcolor", "1.1.3"), ("time", "0.1.44"), ("tinyvec", "1.6.0"), ("tinyvec_macros", "0.1.0"), ("toml", "0.5.9"), ("unicode-bidi", "0.3.8"), ("unicode-ident", "1.0.5"), ("unicode-normalization", "0.1.22"), ("unicode-width", "0.1.10"), ("url", "2.3.1"), ("wasi", "0.10.0+wasi-snapshot-preview1"), ("wasm-bindgen", "0.2.83"), ("wasm-bindgen-backend", "0.2.83"), ("wasm-bindgen-macro", "0.2.83"), ("wasm-bindgen-macro-support", "0.2.83"), ("wasm-bindgen-shared", "0.2.83"), ("winapi", "0.3.9"), ("winapi-i686-pc-windows-gnu", "0.4.0"), ("winapi-util", "0.1.5"), ("winapi-x86_64-pc-windows-gnu", "0.4.0")];
#[doc=r#"The effective dependencies as a comma-separated string."#]
#[allow(dead_code)]
pub const DEPENDENCIES_STR: &str = r"android_system_properties 0.1.5, autocfg 1.1.0, built 0.5.1, bumpalo 3.11.0, cargo-lock 7.1.0, cc 1.0.73, cfg-if 1.0.0, chrono 0.4.22, codespan-reporting 0.11.1, console_error_panic_hook 0.1.7, core-foundation-sys 0.8.3, cxx 1.0.78, cxx-build 1.0.78, cxxbridge-flags 1.0.78, cxxbridge-macro 1.0.78, form_urlencoded 1.1.0, htmlescape 0.3.1, iana-time-zone 0.1.51, iana-time-zone-haiku 0.1.0, idna 0.3.0, js-sys 0.3.60, libc 0.2.135, link-cplusplus 1.0.7, log 0.4.17, macro_railroad 0.1.3, macro_railroad_ext 0.1.0, num-integer 0.1.45, num-traits 0.2.15, once_cell 1.15.0, percent-encoding 2.2.0, proc-macro2 1.0.46, quote 1.0.21, railroad 0.1.1, scratch 1.0.2, semver 1.0.14, serde 1.0.145, serde_derive 1.0.145, syn 1.0.102, termcolor 1.1.3, time 0.1.44, tinyvec 1.6.0, tinyvec_macros 0.1.0, toml 0.5.9, unicode-bidi 0.3.8, unicode-ident 1.0.5, unicode-normalization 0.1.22, unicode-width 0.1.10, url 2.3.1, wasi 0.10.0+wasi-snapshot-preview1, wasm-bindgen 0.2.83, wasm-bindgen-backend 0.2.83, wasm-bindgen-macro 0.2.83, wasm-bindgen-macro-support 0.2.83, wasm-bindgen-shared 0.2.83, winapi 0.3.9, winapi-i686-pc-windows-gnu 0.4.0, winapi-util 0.1.5, winapi-x86_64-pc-windows-gnu 0.4.0";
#[doc=r#"The build time in RFC2822, UTC."#]
#[allow(dead_code)]
pub const BUILT_TIME_UTC: &str = r"Mon, 10 Oct 2022 15:35:01 +0000";
#[doc=r#"The target architecture, given by `CARGO_CFG_TARGET_ARCH`."#]
#[allow(dead_code)]
pub const CFG_TARGET_ARCH: &str = r"wasm32";
#[doc=r#"The endianness, given by `CARGO_CFG_TARGET_ENDIAN`."#]
#[allow(dead_code)]
pub const CFG_ENDIAN: &str = r"little";
#[doc=r#"The toolchain-environment, given by `CARGO_CFG_TARGET_ENV`."#]
#[allow(dead_code)]
pub const CFG_ENV: &str = r"";
#[doc=r#"The OS-family, given by `CARGO_CFG_TARGET_FAMILY`."#]
#[allow(dead_code)]
pub const CFG_FAMILY: &str = r"wasm";
#[doc=r#"The operating system, given by `CARGO_CFG_TARGET_OS`."#]
#[allow(dead_code)]
pub const CFG_OS: &str = r"unknown";
#[doc=r#"The pointer width, given by `CARGO_CFG_TARGET_POINTER_WIDTH`."#]
#[allow(dead_code)]
pub const CFG_POINTER_WIDTH: &str = r"32";
//
// EVERYTHING ABOVE THIS POINT WAS AUTO-GENERATED DURING COMPILATION. DO NOT MODIFY.
//
