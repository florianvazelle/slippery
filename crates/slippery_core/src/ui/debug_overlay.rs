use gdnative::api::{RigidBody, Engine, OS};
use gdnative::prelude::*;

use std::cmp::max;

use crate::utils::humanize_size;

/// A simple debug overlay to monitor variables
#[derive(NativeClass)]
#[inherit(CanvasLayer)]
pub struct DebugOverlay {
    monitors: Vec<(GodotString, Ref<Node, Shared>, String)>,
    pub visible: bool,
    max_length: usize,
}

#[methods]
impl DebugOverlay {
    fn new(_owner: &CanvasLayer) -> Self {
        DebugOverlay {
            monitors: Vec::new(),
            visible: false,
            max_length: 13,
        }
    }

    pub fn add_monitor(&mut self, label: impl Into<GodotString>, caller: impl AsArg<Node>, target: impl Into<NodePath>) {
        let label = label.into();
        let target_node = unsafe { caller.to_arg_variant().to_object::<Node>().unwrap() };
        let property_path = format!(":{}", target.into().get_concatenated_subnames());
        self.max_length = max(self.max_length, label.len());
        self.monitors.push((label, target_node, property_path));
    }

    #[export]
    fn _process(&self, owner: &CanvasLayer, _delta: f32) {
        if self.visible {
            let template = GodotString::from("{0}: {1}");
            
            let engine = Engine::godot_singleton();
            let os = OS::godot_singleton();

            let mut label_text = GodotString::new();

            // Add fps
            label_text += self.add_text(&GodotString::from("FPS"), engine.get_frames_per_second());

            // Add static memory
            label_text += GodotString::from("\n") + self.add_text(&GodotString::from("Static Memory"), humanize_size(os.get_static_memory_usage() as f64));

            // Add all monitors
            for (label, node, property_path) in self.monitors.iter() {
                let node = unsafe { node.assume_safe() };
                let property = node.get_indexed(property_path);

                label_text += GodotString::from("\n") + self.add_text(label, property);
            }

            // Update label
            let label = unsafe { owner.get_node_as::<Label>("Label").unwrap() };
            label.set_text(label_text);
        }
    }

    fn add_text(&self, label: &GodotString, property: impl ToVariant) -> GodotString {
        let template = GodotString::from("{0}{1}: {2}");

        let data = VariantArray::new();
        data.push(" ".repeat(self.max_length - label.len()));
        data.push(label);
        data.push(property);

        template.format(&data.into_shared().to_variant())
    }
}

pub fn load_debug_overlay(node: &RigidBody) -> Option<Instance<DebugOverlay, Unique>> {
    let tree = node.get_tree()?;
    let tree = unsafe { tree.assume_safe() };

    let root = tree.root()?;
    let root = unsafe { root.assume_safe() };

    let debug_overlay_node = unsafe { root.get_node_as::<CanvasLayer>("./DebugOverlay")? };
    let debug_overlay_node = unsafe { debug_overlay_node.assume_unique() };

    Instance::<DebugOverlay, _>::from_base(debug_overlay_node)
}