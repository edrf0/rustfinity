/*
The Plugin trait should include the following methods:
fn name(&self) -> &str; - Returns the name of the plugin.
fn execute(&self); - Executes the plugin's functionality.
The PluginManager should:
Have the following methods and associated functions:
new() -> Self - Creates a new PluginManager instance.
add_plugin - Adds a plugin to the list.
remove_plugin - Removes a plugin from the list and returns the removed plugin if found.
execute_all - Executes all registered plugins.
If a duplicate plugin is added (with the same name), it should panic with the message
"Plugin with name [name] already exists".
Make sure you make all relevant items public.*/
use std::collections::HashMap;

pub trait Plugin {
    // 1. Finish the trait
    fn name(&self) -> &str;
    fn execute(&self);
}

pub struct PluginManager {
    // 2. Finish the struct
    // Make fields public
    plugins: HashMap<&str,Box<dyn Plugin>>,
}

// 3. Implement the PluginManager
impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }
    pub fn add_plugin(&mut self, plugin: Box<dyn Plugin>) {
        if self.plugins.contains_key(plugin.name()) {
            panic!("Plugin with name {} already exists", plugin.name());
        }
        self.plugins.insert(plugin.name(),plugin);
    }
    pub fn remove_plugin(&mut self, plugin: Box<dyn Plugin>) -> Option<Box<dyn Plugin>> {
        self.plugins.remove(&plugin.name())
    }
    pub fn execute_all(&mut self) {
        self.plugins.iter().for_each(|plugin| plugin.execute());
    }
}


// Example usage
pub struct MyPlugin;

impl Plugin for MyPlugin {
    fn name(&self) -> &str {
        "MyPlugin"
    }
    fn execute(&self) {
        println!("Executing MyPlugin");
    }
}

impl MyPlugin {
    fn new() -> Self {
        Self
    }
}

pub fn main() {
    let mut manager = PluginManager::new();

    manager.add_plugin(Box::new(MyPlugin::new()));
    manager.execute_all();
}
