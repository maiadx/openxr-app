use openxr as xr;

pub struct ActionSet {
    pub action_set: xr::ActionSet,
    pub right_hand_action: xr::Action<xr::Posef>,
    pub left_hand_action: xr::Action<xr::Posef>,
}

impl ActionSet {
    pub fn new(xr_instance: &xr::Instance) -> xr::Result<Self> {
        // Create an action set
        let action_set = xr_instance.create_action_set("input", "Input Pose Information", 0)?;

        // Create actions
        let right_hand_action = action_set.create_action::<xr::Posef>("right_hand", "Right Hand Controller", &[])?;
        let left_hand_action = action_set.create_action::<xr::Posef>("left_hand", "Left Hand Controller", &[])?;

        // Suggest interaction profile bindings
        xr_instance.suggest_interaction_profile_bindings(
            xr_instance.string_to_path("/interaction_profiles/khr/simple_controller")?,
            &[
                xr::Binding::new(&right_hand_action, xr_instance.string_to_path("/user/hand/right/input/grip/pose")?),
                xr::Binding::new(&left_hand_action, xr_instance.string_to_path("/user/hand/left/input/grip/pose")?),
            ],
        )?;

        Ok(Self {
            action_set,
            right_hand_action,
            left_hand_action,
        })
    }

    pub fn attach(&self, session: &xr::Session<xr::Vulkan>) -> xr::Result<()> {
        session.attach_action_sets(&[&self.action_set])
    }

    pub fn create_action_spaces(
        &self,
        session: &xr::Session<xr::Vulkan>,
    ) -> xr::Result<(xr::Space, xr::Space)> {
        let right_space = self.right_hand_action.create_space(session.clone(), xr::Path::NULL, xr::Posef::IDENTITY)?;
        let left_space = self.left_hand_action.create_space(session.clone(), xr::Path::NULL, xr::Posef::IDENTITY)?;

        Ok((right_space, left_space))
    }
}