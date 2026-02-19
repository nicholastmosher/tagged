use std::marker::PhantomData;

use zed::unstable::{
    gpui::{self, Action, AppContext as _, Entity, EventEmitter, FocusHandle, Focusable, actions},
    ui::{
        ActiveTheme as _, App, Context, IconName, IntoElement, ParentElement as _, Pixels, Render,
        Styled as _, Window, div, px,
    },
    workspace::{
        Panel,
        dock::{DockPosition, PanelEvent},
    },
};

actions!(workspace, [ToggleContactsPanel]);

// #[derive(WillowModel)]
#[derive(JsonSchema)]
pub struct Contact {
    // #[willow(path = "/apps/contacts/entities/name")]
    name: String,
    // #[willow(path = "/apps/contacts/entities/key")]
    key: String,
}

impl Contact {
    //
    pub fn new(name: String, key: String, cx: &mut Context<Self>) -> Self {
        //
        Self {
            //
            name,
            key,
        }
    }
}

/// A Willow Entity is a handle representing an object with a well-known type
///
/// To be a somewhat complete and well-addressed handle, a WillowEntity includes
/// information about the namespace and subspace of the underlying Entry.
///
/// So an Entity is like an address/handle for an Area, so it's defined by its
/// namespace, subspace, and path prefix (directory). The definition of a Willow
/// Area also includes a time range, I want to think about how to represent time
/// in a dedicated brainstorm.
///
/// - Area in the spec has `subspace_id: SubspaceId | any`, which implies an
///   arbitrary restriction in the expressiveness of the API. I think it should
///   easily be possible to specify a list of subspaces we're interested in.
struct WillowEntity<T: WillowModel> {
    _phantom: PhantomData<T>,
}

struct WillowContext<T> {
    _phantom: PhantomData<T>,
}

impl<T: WillowModel> WillowEntity<T> {
    fn read(&self, cx: &mut WillowContext<T>) -> Option<&T> {
        None
    }
}

// WillowComponent?
// WillowSpec
// WillowArea
// WillowModel <-- expresses paths to multiple files, typed extractors
// - Model would refer to a multi-"file" data construction which is located
//   at a path and described by the set of files the model refers to, as well
//   as the types of those files.
use schemars::JsonSchema;
trait WillowModel: Sized + JsonSchema {
    // uhhh, does it matter that Sized was required to be here? fuzzy on Sized details...

    fn model() -> Schema<Self>;
}
impl WillowModel for () {
    fn model() -> Schema<Self> {
        SchemaBuilder::new().finish()
    }
}
impl WillowModel for String {
    fn model() -> Schema<Self> {
        SchemaBuilder::new().finish()
    }
}
impl WillowModel for Contact {
    fn model() -> Schema<Self> {
        // TODO: derive trait impl
        SchemaBuilder::new()
            .field::<String>("name")
            .field::<String>("key")
            .finish()
    }
}

/// Schema of a `T`
struct Schema<T: WillowModel = ()> {
    _phantom: PhantomData<T>,
}
struct SchemaBuilder {
    //
}
impl SchemaBuilder {
    fn new() -> Self {
        SchemaBuilder {}
    }
    fn field<T: WillowModel>(mut self, name: &str) -> Self {
        self
    }

    fn finish<T: WillowModel>(&self) -> Schema<T> {
        Schema {
            //
            _phantom: PhantomData,
        }
    }
}
struct AnySchema {} // Type-erased schema
struct Entry; // stand-in for Willow Entry (as per spec)
trait FromEntry {
    fn from_entry(entry: Entry) -> Self;
}

impl Render for Contact {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            //
            .p_2()
            .border_1()
            .border_color(cx.theme().colors().border.opacity(0.6))
            .rounded_md()
            .flex()
            .flex_col()
            // Name
            .child(
                //
                div().child(format!("Name: {}", self.name)),
            )
            // key
            .child(
                //
                div().child(format!("Key: {}", self.key)),
            )
    }
}

/// Display a feed of contacts for the active Profile or Profiles
pub struct Contacts {
    contacts: Vec<Entity<Contact>>,
    focus_handle: FocusHandle,
    width: Option<Pixels>,
}

impl Contacts {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();

        let contacts = vec![
            cx.new(|cx| Contact::new("Apple".to_string(), "applekey".to_string(), cx)),
            cx.new(|cx| Contact::new("Banana".to_string(), "bananakey".to_string(), cx)),
            cx.new(|cx| Contact::new("Cranberry".to_string(), "cranberrykey".to_string(), cx)),
        ];

        Self {
            contacts,
            // contacts: Default::default(),
            focus_handle,
            width: None,
        }
    }
}

impl Render for Contacts {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .debug()
            .p_2()
            .flex()
            .flex_col()
            .children(self.contacts.iter().map(|contact| {
                //
                contact.clone()
            }))
    }
}

impl Focusable for Contacts {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}
impl EventEmitter<PanelEvent> for Contacts {}
impl Panel for Contacts {
    fn persistent_name() -> &'static str {
        "Contacts"
    }

    fn panel_key() -> &'static str {
        "contacts"
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
        Some(IconName::Person)
    }

    fn icon_tooltip(&self, _window: &Window, _cx: &App) -> Option<&'static str> {
        Some("Contacts")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleContactsPanel)
    }

    fn activation_priority(&self) -> u32 {
        0
    }
}
