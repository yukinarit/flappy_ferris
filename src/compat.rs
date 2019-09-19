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

pub fn rand() -> f32 {
    rand::random()
}

pub fn randrng(min: f32, max: f32) -> f32 {
    min + (max - min) * rand()
}
