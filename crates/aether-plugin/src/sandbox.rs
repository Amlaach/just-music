use aether_core::{AetherError, Result};
use std::path::Path;
use wasmtime::*;

pub struct PluginSandbox {
    engine: Engine,
    store: Store<()>,
}

impl PluginSandbox {
    pub fn new() -> Result<Self> {
        let mut config = Config::new();
        config.wasm_component_model(true);
        config.async_support(true);

        let engine = Engine::new(&config)
            .map_err(|e| AetherError::Plugin(format!("Failed to create Wasm engine: {}", e)))?;

        let store = Store::new(&engine, ());

        Ok(Self { engine, store })
    }

    pub fn load_module<P: AsRef<Path>>(&mut self, path: P) -> Result<Module> {
        Module::from_file(&self.engine, path)
            .map_err(|e| AetherError::Plugin(format!("Failed to load WASM module: {}", e)))
    }
}
