pub fn telemetry_key_validate(key: String) -> bool {
    log::debug!("Checking key: {:?} for validity", &key);
    if key.len() == 0 {
        log::debug!("Key do have a zero length");
        return false;
    }
    true
}
