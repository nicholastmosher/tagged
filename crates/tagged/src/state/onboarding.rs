use zed::unstable::{
    gpui::{AppContext as _, Entity},
    ui::{Context, Window},
    workspace::Workspace,
};

use crate::{
    scenes::onboarding_item::OnboardingItem,
    state::{profile::Profile, space::Space},
};

/// State tracking a user's first interaction with the app
///
/// Initially this includes creating a first Profile and Space
///
/// There may be future needs like explaining the UI
#[non_exhaustive]
pub struct Onboarding {
    pub profile: Option<Profile>,
    pub space: Option<Space>,

    pub onboarding_item: Option<Entity<OnboardingItem>>,
    workspace: Entity<Workspace>,
}

impl Onboarding {
    pub fn new(workspace: Entity<Workspace>, _cx: &mut Context<Self>) -> Self {
        //
        Self {
            //
            profile: Default::default(),
            space: Default::default(),
            onboarding_item: Default::default(),
            workspace,
        }
    }

    pub fn open_onboarding(&self, window: &mut Window, cx: &mut Context<Self>) {
        // Grab the existing item, or create one
        let item = self
            .onboarding_item
            .clone()
            .unwrap_or_else(|| cx.new(|cx| OnboardingItem::new(cx)));

        self.workspace.update(cx, |workspace, cx| {
            workspace.add_item_to_active_pane(Box::new(item), Some(0), true, window, cx);
        });
    }
}
