use gpui::{
    App, AppContext as _, Context, EventEmitter, FocusHandle, Focusable, ParentElement as _,
    Render, actions, div,
};
use workspace::{Item, Workspace};

actions!(
    workspace,
    [
        /// Open the willow pane/item
        OpenWillowUi,
    ]
);

pub fn init(cx: &mut App) {
    cx.observe_new(move |workspace: &mut Workspace, _window, _cx| {
        workspace.register_action(move |workspace, _: &OpenWillowUi, window, cx| {
            let willow_ui = cx.new(|cx| WillowUi::new(cx));
            workspace.add_item_to_active_pane(Box::new(willow_ui), None, true, window, cx)
        });
    })
    .detach();
}

pub struct WillowUi {
    focus_handle: FocusHandle,
}

impl WillowUi {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        Self { focus_handle }
    }
}

pub enum WillowEvent {
    //
}

impl Item for WillowUi {
    type Event = WillowEvent;

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> gpui::SharedString {
        "Willow".into()
    }
}

impl Focusable for WillowUi {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<WillowEvent> for WillowUi {}

impl Render for WillowUi {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        _cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        div().child("This is the willow UI")
    }
}

// pub fn init(app_state: Arc<AppState>, cx: &mut App) {
//     workspace::register_serializable_item::<ComponentPreview>(cx);

//     cx.observe_new(move |workspace: &mut Workspace, _window, cx| {
//         let app_state = app_state.clone();
//         let project = workspace.project().clone();
//         let weak_workspace = cx.entity().downgrade();

//         workspace.register_action(
//             move |workspace, _: &workspace::OpenComponentPreview, window, cx| {
//                 let app_state = app_state.clone();

//                 let language_registry = app_state.languages.clone();
//                 let user_store = app_state.user_store.clone();

//                 let component_preview = cx.new(|cx| {
//                     ComponentPreview::new(
//                         weak_workspace.clone(),
//                         project.clone(),
//                         language_registry,
//                         user_store,
//                         None,
//                         None,
//                         window,
//                         cx,
//                     )
//                     .expect("Failed to create component preview")
//                 });

//                 workspace.add_item_to_active_pane(
//                     Box::new(component_preview),
//                     None,
//                     true,
//                     window,
//                     cx,
//                 )
//             },
//         );
//     })
//     .detach();
// }
