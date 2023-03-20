use crate::plugin::Plugin;

pub struct PluginRegistry {
    inner: Vec<Box<dyn Plugin>>,
}
