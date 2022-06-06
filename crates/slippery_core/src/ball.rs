use gdnative::api::*;
use gdnative::prelude::*;

use convert_case::{Case, Casing};

use sm_gd::*;

use crate::ui::debug_overlay;
use crate::system::fly::{
    states::OnSlide,
    machine::FlyMachine,
    resource::FlyResource
};

#[derive(NativeClass)]
#[inherit(RigidBody)]
pub struct Ball {
    #[property(default = 5.0)]
    rotate_speed: f32,

    #[property]
    ball_resource: Instance<FlyResource>,
    state_machine: FlyMachine,
}

#[methods]
impl Ball {
    fn new(_owner: &RigidBody) -> Self {
        Ball {
            rotate_speed: 5.0,
            ball_resource: Instance::<FlyResource, Unique>::new().into_shared(),
            // Set default state to OnSlide
            state_machine: FlyMachine::new(OnSlide),
        }
    }

    #[export]
    fn _ready(&mut self, owner: TRef<RigidBody>) {
        let resource = unsafe { self.ball_resource.assume_safe() };
        
        // Add a debug overlay
        let debug_overlay = debug_overlay::load_debug_overlay(owner.as_ref()).expect("Failed to get debug overlay instance");
    
        debug_overlay
            .map_mut(|d, _o| {
                d.visible = true;
                resource.map(|_r, o| {
                    for property_name in o.get_property_list().iter() {
                        let name = property_name.to::<Dictionary>().unwrap()
                            .get::<String>("name".to_string()).unwrap()
                            .to::<String>().unwrap();
                        d.add_monitor(name.to_case(Case::Title), owner, format!("Self:ball_resource:{}", name))
                    }
                }).unwrap()
            })
            .unwrap();

        // Activate physics process method
        owner.set_physics_process(true);

        // Activate contact monitor to receive collision signals
        owner.set_contact_monitor(true);

        // Initialize first state 
        let resource = unsafe { self.ball_resource.assume_safe() };
        resource.map_mut(|r, _o| self.state_machine.state.ready(owner.as_ref(), r)).unwrap();
    }

    #[export]
    fn _input(&mut self, owner: &RigidBody, event: Ref<InputEvent>) {
        let resource = unsafe { self.ball_resource.assume_safe() };
        resource.map_mut(|r, _o| {
            self.state_machine.state.input(owner, r, event).map(|new_state| {
                // Update current state
                self.state_machine.state(new_state);
                self.state_machine.state.init(owner, r)
            })
        }).unwrap();
    }

    #[export]
    fn _process(&mut self, owner: &RigidBody, delta: f32) {
        let resource = unsafe { self.ball_resource.assume_safe() };
        resource.map_mut(|r, _o| {
            self.state_machine.state.update(owner, r, delta).map(|new_state| {
                // Update current state
                self.state_machine.state(new_state);
                self.state_machine.state.init(owner, r)
            })
        }).unwrap();
    }

    #[export]
    fn _physics_process(&mut self, owner: &RigidBody, delta: f32) {
        let resource = unsafe { self.ball_resource.assume_safe() };
        resource.map_mut(|r, _o| {
            self.state_machine.state.physics_update(owner, r, delta).map(|new_state| {
                // Update current state
                self.state_machine.state(new_state);
                self.state_machine.state.init(owner, r)
            })
        }).unwrap();
    }

    #[export]
    fn _integrate_forces(&mut self, owner: &RigidBody, physics_state: Ref<PhysicsDirectBodyState>) {
        let physics_state = unsafe { physics_state.assume_safe() };
        let resource = unsafe { self.ball_resource.assume_safe() };
        resource.map_mut(|r, _o| self.state_machine.state.integrate_forces(owner, r, physics_state.step() as f32)).unwrap();
    }

    #[export]
    fn on_ball_body_entered(&self, _owner: &RigidBody, body: Ref<Node>) {
        let resource = unsafe { self.ball_resource.assume_safe() };
        resource.map_mut(|s, _o| s.body_collision = Some(body)).unwrap();
    }

    #[export]
    fn on_ball_body_existed(&self, _owner: &RigidBody, _body: Ref<Node>) {
        let resource = unsafe { self.ball_resource.assume_safe() };
        resource.map_mut(|s, _o| s.body_collision = None).unwrap();
    }
}
