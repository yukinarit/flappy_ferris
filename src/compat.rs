pub fn init_logger() {
    //console_log::init_with_level(log::Level::Debug).expect("Couldn't setup logger.");
}

//pub fn now() -> u64 {
//    #[cfg(target_arch = "wasm32")]
//    return js_sys::Date::now() as u64 * 1000;
//
//    #[cfg(not(target_arch = "wasm32"))]
//    return std::time::SystemTime::now()
//        .duration_since(std::time::SystemTime::UNIX_EPOCH)
//        .unwrap()
//        .as_secs();
//}
