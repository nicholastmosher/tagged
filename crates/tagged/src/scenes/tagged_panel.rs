use std::path::PathBuf;

use zed::unstable::{
    gpui::{
        self, Action, AppContext as _, Entity, EventEmitter, FocusHandle, Focusable, actions, img,
    },
    ui::{
        ActiveTheme, App, Context, FluentBuilder as _, IconName, InteractiveElement as _,
        IntoElement, ListSeparator, ParentElement as _, Pixels, Render, StatefulInteractiveElement,
        Styled, Tooltip, Window, div, h_flex, px, v_flex,
    },
    workspace::{
        Panel, Workspace,
        dock::{DockPosition, PanelEvent},
    },
};

use crate::{
    components::{profile_bar::ProfileBar, space_header::SpaceHeader, space_icon::SpaceIcon},
    state::{profile::Profile, space::Space},
};

actions!(workspace, [ToggleTaggedPanel]);

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };

        let tagged_panel = cx.new(|cx| TaggedPanel::new(cx));
        workspace.add_panel(tagged_panel, window, cx);
        workspace.focus_panel::<TaggedPanel>(window, cx);
        workspace.register_action(|workspace, _: &ToggleTaggedPanel, window, cx| {
            workspace.toggle_panel_focus::<TaggedPanel>(window, cx);
        });
    })
    .detach();
}

pub struct TaggedPanel {
    active_profile: Option<Entity<Profile>>,
    active_space: Entity<Space>,
    focus_handle: FocusHandle,
    width: Option<Pixels>,
}

impl TaggedPanel {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let active_profile =
            cx.new(|cx| Profile::new("Myselfandi", cx).with_avatar(".assets/tagged.svg"));

        let active_space = cx.new(|cx| Space::new("Group's Space", cx));

        Self {
            //
            active_profile: None,
            // active_profile: Some(active_profile),
            active_space,
            focus_handle: cx.focus_handle(),
            width: None,
        }
    }
}

impl Render for TaggedPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .h_full()
            .w(self.width.unwrap_or(px(300.)) - px(1.))
            .map(|el| match &self.active_profile {
                None => {
                    //
                    el
                        //
                        .child(self.render_initial_panel(window, cx))
                }
                Some(profile) => {
                    //
                    el
                        //
                        .child(self.render_active_profile(profile.clone(), window, cx))
                }
            })
    }
}

impl TaggedPanel {
    fn render_initial_panel(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        v_flex()
            .debug()
            .size_full()
            //
            .p_2()
            .py_20()
            .child(
                v_flex()
                    .debug()
                    .flex_1()
                    //
                    .child(
                        div()
                            .text_center()
                            .child("Welcome! To get started, create a new Profile"),
                    )
                    .child(
                        //
                        h_flex()
                            .mx_auto()
                            //
                            .p_2()
                            .border_4()
                            .border_color(cx.theme().colors().border_selected)
                            .rounded_2xl()
                            .justify_center()
                            .child(
                                div()
                                    .id("create-profile")
                                    .hover(|style| {
                                        style
                                            //
                                            .bg(cx.theme().colors().ghost_element_hover)
                                    })
                                    .active(|style| {
                                        style
                                            //
                                            .bg(cx.theme().colors().ghost_element_active)
                                    })
                                    .rounded_xl()
                                    .child(
                                        //
                                        img(PathBuf::from(".assets/create-profile.svg"))
                                            //
                                            .size(px(96.))
                                            .rounded_xl(),
                                    ),
                            ),
                    ),
            )
            .child(
                v_flex()
                    .debug()
                    .flex_1()
                    //
                    .child("Then, you'll create your first Space")
                    .child(
                        //
                        h_flex()
                            .mx_auto()
                            //
                            .p_2()
                            .border_2()
                            .border_dashed()
                            .border_color(cx.theme().colors().border_disabled)
                            .rounded_2xl()
                            .justify_center()
                            .child(
                                div()
                                    .id("create-space")
                                    .hover(|style| {
                                        style
                                            //
                                            .bg(cx.theme().colors().ghost_element_hover)
                                    })
                                    .active(|style| {
                                        style
                                            //
                                            .bg(cx.theme().colors().ghost_element_active)
                                    })
                                    .rounded_xl()
                                    .child(
                                        //
                                        img(PathBuf::from(".assets/create-space.svg"))
                                            //
                                            .size(px(96.))
                                            .rounded_xl(),
                                    ),
                            ),
                    ),
            )
            .child(
                //
                h_flex()
                    .mx_auto()
                    //
                    .p_2()
                    .border_2()
                    .border_dashed()
                    .border_color(cx.theme().colors().border_disabled)
                    .rounded_2xl()
                    .justify_center()
                    .child(
                        div()
                            .id("create-profile-next")
                            .opacity(0.6)
                            .hover(|style| {
                                style
                                    //
                                    .opacity(1.)
                                    .bg(cx.theme().colors().ghost_element_hover)
                            })
                            .active(|style| {
                                style
                                    //
                                    .bg(cx.theme().colors().ghost_element_active)
                            })
                            .rounded_xl()
                            .child(
                                //
                                img(PathBuf::from(".assets/create-profile.svg"))
                                    //
                                    .size(px(96.))
                                    .rounded_xl(),
                            ),
                    ),
            )
    }

    fn render_active_profile(
        &mut self,
        profile: Entity<Profile>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        v_flex()
            .h_full()
            .w(self.width.unwrap_or(px(300.)) - px(1.))
            // Profile space?
            .child(
                h_flex()
                    .h_full()
                    .pb_20()
                    // Spaces bar
                    .child(
                        //
                        self.render_spaces_column(window, cx),
                    )
                    .child(
                        div()
                            .h_full()
                            .w_0()
                            .mt_2()
                            .border_1()
                            .border_color(cx.theme().colors().border),
                    )
                    // Active space content
                    .child(
                        //
                        self.render_active_space(window, cx),
                    ),
            )
            // Profile bar/selector
            .child(self.render_profile_bar(profile, window, cx))
    }

    fn render_profile_bar(
        &mut self,
        profile: Entity<Profile>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        h_flex()
            .w_full()
            .absolute()
            .bottom_0()
            //
            // .mt_auto()
            .p_2()
            .child(ProfileBar::new(profile))
    }

    fn render_spaces_column(
        &mut self,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        v_flex()
            .id("spaces-column")
            .h_full()
            .pt_2()
            .px_2()
            .gap_1()
            .overflow_y_scroll()
            // TODO: Children, one per space for active profile
            .child(SpaceIcon::new("space-icon-1", ".assets/tagged.svg").size(px(48.)))
            .child(SpaceIcon::new("space-icon-2", ".assets/tagged.svg").size(px(48.)))
            .child(SpaceIcon::new("space-icon-3", ".assets/tagged.svg").size(px(48.)))
            .child(SpaceIcon::new("space-icon-4", ".assets/tagged.svg").size(px(48.)))
            .child(SpaceIcon::new("space-icon-5", ".assets/tagged.svg").size(px(48.)))
            .child(SpaceIcon::new("space-icon-6", ".assets/tagged.svg").size(px(48.)))
            .child(SpaceIcon::new("space-icon-7", ".assets/tagged.svg").size(px(48.)))
            .child(SpaceIcon::new("space-icon-8", ".assets/tagged.svg").size(px(48.)))
            .child(SpaceIcon::new("space-icon-9", ".assets/tagged.svg").size(px(48.)))
            .child(SpaceIcon::new("space-icon-10", ".assets/tagged.svg").size(px(48.)))
            .child(SpaceIcon::new("space-icon-11", ".assets/tagged.svg").size(px(48.)))
            .child(div().flex_grow())
            // TODO: Tools like create space (+)
            .child(
                div()
                    //
                    .id("create-space")
                    .bg(cx.theme().colors().editor_background)
                    .rounded_xl()
                    .child(
                        SpaceIcon::new("space-icon-12", ".assets/create-space.svg")
                            .size(px(48.))
                            .tooltip(Tooltip::text("Create Space")),
                    ),
            )
    }

    fn render_active_space(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        // Container, no flex
        v_flex()
            //
            .p_2()
            .size_full()
            .child(SpaceHeader::new(self.active_space.clone()))
            .child(ListSeparator)
    }
}

impl EventEmitter<PanelEvent> for TaggedPanel {}
impl Focusable for TaggedPanel {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Panel for TaggedPanel {
    fn persistent_name() -> &'static str {
        "TaggedPanel"
    }

    fn panel_key() -> &'static str {
        "tagged-panel"
    }

    fn position(&self, _window: &Window, _cx: &App) -> DockPosition {
        DockPosition::Left
    }

    fn position_is_valid(&self, _position: DockPosition) -> bool {
        true
    }

    fn set_position(
        &mut self,
        _position: DockPosition,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
    }

    fn size(&self, _window: &Window, _cx: &App) -> Pixels {
        self.width.unwrap_or(px(300.))
    }

    fn set_size(&mut self, size: Option<Pixels>, _window: &mut Window, _cx: &mut Context<Self>) {
        self.width = size;
    }

    fn icon(&self, _window: &Window, _cx: &App) -> Option<IconName> {
        Some(IconName::Hash)
    }

    fn icon_tooltip(&self, _window: &Window, _cx: &App) -> Option<&'static str> {
        Some("Tagged")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleTaggedPanel)
    }

    fn activation_priority(&self) -> u32 {
        0
    }
}
