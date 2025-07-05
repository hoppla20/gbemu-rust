use poll_promise::Promise;

#[cfg(not(target_arch = "wasm32"))]
pub fn execute<F: std::future::Future<Output = Option<Vec<u8>>> + Send + 'static>(
    f: F,
) -> Promise<Option<Vec<u8>>> {
    Promise::spawn_async(f)
}

#[cfg(target_arch = "wasm32")]
pub fn execute<F: std::future::Future<Output = Option<Vec<u8>>> + 'static>(
    f: F,
) -> Promise<Option<Vec<u8>>> {
    Promise::spawn_local(f)
}
