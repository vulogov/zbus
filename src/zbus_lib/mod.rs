use rhai::{Engine, Scope};

pub fn initscope(scope: &mut Scope) {
    log::debug!("Initializing ZBUS RHAI library");
    scope.push("ANSWER", 42_i64);
}

pub fn initlib(_engine: &mut Engine)  {
    log::debug!("Initializing ZBUS RHAI library");

}
