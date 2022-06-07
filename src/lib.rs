#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
use linux::DeviceVFileBuilder;

#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
use windows::DeviceVFileBuilder;

use std::sync::{Arc};

use tap::config_schema;
use tap::plugin;
use tap::plugin::{PluginInfo, PluginInstance, PluginConfig, PluginArgument, PluginResult, PluginEnvironment};
use tap::tree::{TreeNodeIdSchema};
use tap::node::Node;
use tap::tree::TreeNodeId;
use tap::value::Value;

use serde::{Serialize, Deserialize};
use schemars::{JsonSchema};

plugin!("device", "Input", "Mount a device", Device, Arguments);


#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Arguments
{
  path : String,
  #[schemars(with = "TreeNodeIdSchema")] 
  mount_point : TreeNodeId,
}

#[derive(Debug, Serialize, Deserialize,Default)]
pub struct Results
{
}

#[derive(Default)]
pub struct Device 
{
}

impl Device 
{
  fn run(&mut self, args : Arguments, env : PluginEnvironment) -> anyhow::Result<Results>
  {
    let (device_name, vfile_builder) = DeviceVFileBuilder::new(args.path.clone())?;
    let node = Node::new(device_name);
    node.value().add_attribute(self.name(), None, None);
    node.value().add_attribute("data", Value::VFileBuilder(Arc::new(vfile_builder)), None);

    let _node_id = env.tree.add_child(args.mount_point, node)?;

    Ok(Results{})
  }
}

