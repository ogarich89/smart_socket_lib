use std::path::Path;

#[cfg(target_os = "linux")]
pub fn lib_path() -> &'static Path {
    Path::new("target/release/libsmart_socket.so")
}

#[cfg(target_os = "windows")]
pub fn lib_path() -> &'static Path {
    Path::new("target/release/libsmart_socket.dll")
}

#[cfg(target_os = "macos")]
pub fn lib_path() -> &'static Path {
    Path::new("target/release/libsmart_socket_lib.dylib")
}