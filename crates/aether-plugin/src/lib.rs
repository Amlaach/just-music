pub mod sandbox;

pub use sandbox::PluginSandbox;

pub fn plugin_engine_init() {
    tracing::info!("Aether WebAssembly Plugin Sandbox Engine ready");
}
