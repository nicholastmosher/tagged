use gpui::{
    App, AppContext as _, Context, EventEmitter, FocusHandle, Focusable, ParentElement as _,
    Render, SharedString, actions, div, *,
};
use workspace::ui::{
    ActiveTheme, Clickable, Color, Icon, IconButton, IconName, IconSize, Label, LabelCommon,
    LabelSize, ListItem, ListItemSpacing, h_flex, v_flex,
};
use workspace::{Item, Workspace};

actions!(
    workspace,
    [
        /// Open the willow filesystem browser
        OpenWillowUi,
        /// Toggle namespace expansion
        ToggleNamespace,
        /// Toggle subspace expansion
        ToggleSubspace,
        /// Navigate to path
        NavigateToPath,
        /// Create new folder
        CreateFolder,
        /// Upload file
        UploadFile,
    ]
);

#[derive(Debug, Clone, PartialEq)]
pub enum NamespaceType {
    Shared,
    Private,
}

#[derive(Debug, Clone)]
pub struct FileSystemItem {
    pub name: String,
    pub path: String,
    pub is_directory: bool,
    pub size: Option<u64>,
    pub modified: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Subspace {
    pub id: String,
    pub name: String,
    pub owner: String,
    pub items: Vec<FileSystemItem>,
    pub expanded: bool,
}

#[derive(Debug, Clone)]
pub struct Namespace {
    pub id: String,
    pub name: String,
    pub namespace_type: NamespaceType,
    pub subspaces: Vec<Subspace>,
    pub expanded: bool,
}

#[derive(Debug, Clone)]
pub struct BreadcrumbItem {
    pub name: String,
    pub path: String,
}

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
    namespaces: Vec<Namespace>,
    current_path: Vec<BreadcrumbItem>,
    selected_namespace: Option<String>,
    selected_subspace: Option<String>,
}

impl WillowUi {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();

        // Initialize with sample data
        let namespaces = vec![
            Namespace {
                id: "family".to_string(),
                name: "Family".to_string(),
                namespace_type: NamespaceType::Shared,
                expanded: false,
                subspaces: vec![
                    Subspace {
                        id: "alice".to_string(),
                        name: "Alice".to_string(),
                        owner: "alice@family.com".to_string(),
                        expanded: false,
                        items: vec![
                            FileSystemItem {
                                name: "Documents".to_string(),
                                path: "/family/alice/Documents".to_string(),
                                is_directory: true,
                                size: None,
                                modified: Some("2 days ago".to_string()),
                            },
                            FileSystemItem {
                                name: "Photos".to_string(),
                                path: "/family/alice/Photos".to_string(),
                                is_directory: true,
                                size: None,
                                modified: Some("1 week ago".to_string()),
                            },
                        ],
                    },
                    Subspace {
                        id: "bob".to_string(),
                        name: "Bob".to_string(),
                        owner: "bob@family.com".to_string(),
                        expanded: false,
                        items: vec![FileSystemItem {
                            name: "Music".to_string(),
                            path: "/family/bob/Music".to_string(),
                            is_directory: true,
                            size: None,
                            modified: Some("3 days ago".to_string()),
                        }],
                    },
                ],
            },
            Namespace {
                id: "work".to_string(),
                name: "Work".to_string(),
                namespace_type: NamespaceType::Private,
                expanded: false,
                subspaces: vec![Subspace {
                    id: "projects".to_string(),
                    name: "Projects".to_string(),
                    owner: "me@work.com".to_string(),
                    expanded: false,
                    items: vec![
                        FileSystemItem {
                            name: "willow-fs".to_string(),
                            path: "/work/projects/willow-fs".to_string(),
                            is_directory: true,
                            size: None,
                            modified: Some("1 hour ago".to_string()),
                        },
                        FileSystemItem {
                            name: "presentation.pdf".to_string(),
                            path: "/work/projects/presentation.pdf".to_string(),
                            is_directory: false,
                            size: Some(2048576),
                            modified: Some("Yesterday".to_string()),
                        },
                    ],
                }],
            },
            Namespace {
                id: "photos".to_string(),
                name: "Photos".to_string(),
                namespace_type: NamespaceType::Shared,
                expanded: false,
                subspaces: vec![Subspace {
                    id: "vacation_2024".to_string(),
                    name: "Vacation 2024".to_string(),
                    owner: "shared@photos.com".to_string(),
                    expanded: false,
                    items: vec![
                        FileSystemItem {
                            name: "beach.jpg".to_string(),
                            path: "/photos/vacation_2024/beach.jpg".to_string(),
                            is_directory: false,
                            size: Some(5242880),
                            modified: Some("2 weeks ago".to_string()),
                        },
                        FileSystemItem {
                            name: "mountains.jpg".to_string(),
                            path: "/photos/vacation_2024/mountains.jpg".to_string(),
                            is_directory: false,
                            size: Some(4194304),
                            modified: Some("2 weeks ago".to_string()),
                        },
                    ],
                }],
            },
        ];

        Self {
            focus_handle,
            namespaces,
            current_path: vec![],
            selected_namespace: None,
            selected_subspace: None,
        }
    }

    fn toggle_namespace(&mut self, namespace_id: &str, cx: &mut Context<Self>) {
        if let Some(namespace) = self.namespaces.iter_mut().find(|n| n.id == namespace_id) {
            namespace.expanded = !namespace.expanded;
            if namespace.expanded {
                self.selected_namespace = Some(namespace_id.to_string());
                self.selected_subspace = None;
            }
            cx.notify();
        }
    }

    fn toggle_subspace(&mut self, namespace_id: &str, subspace_id: &str, cx: &mut Context<Self>) {
        if let Some(namespace) = self.namespaces.iter_mut().find(|n| n.id == namespace_id) {
            if let Some(subspace) = namespace.subspaces.iter_mut().find(|s| s.id == subspace_id) {
                subspace.expanded = !subspace.expanded;
                if subspace.expanded {
                    self.selected_namespace = Some(namespace_id.to_string());
                    self.selected_subspace = Some(subspace_id.to_string());
                    self.current_path = vec![
                        BreadcrumbItem {
                            name: namespace.name.clone(),
                            path: format!("/{}", namespace.id),
                        },
                        BreadcrumbItem {
                            name: subspace.name.clone(),
                            path: format!("/{}/{}", namespace.id, subspace.id),
                        },
                    ];
                }
                cx.notify();
            }
        }
    }

    fn render_breadcrumbs(&self, cx: &mut Context<Self>) -> impl gpui::IntoElement {
        h_flex()
            .gap_2()
            .items_center()
            .p_2()
            .bg(cx.theme().colors().surface_background)
            .border_b_1()
            .border_color(cx.theme().colors().border_variant)
            .child(
                Icon::new(IconName::File)
                    .size(IconSize::Small)
                    .color(Color::Muted),
            )
            .children(self.current_path.iter().enumerate().map(|(i, item)| {
                let mut flex = h_flex().gap_1();
                if i > 0 {
                    flex = flex.child(
                        Icon::new(IconName::ChevronRight)
                            .size(IconSize::Small)
                            .color(Color::Muted),
                    );
                }
                flex.child(Label::new(&item.name).size(LabelSize::Small).color(
                    if i == self.current_path.len() - 1 {
                        Color::Default
                    } else {
                        Color::Muted
                    },
                ))
            }))
    }

    fn render_namespace(
        &self,
        namespace: &Namespace,
        namespace_idx: usize,
        cx: &mut Context<Self>,
    ) -> impl gpui::IntoElement {
        let namespace_id = namespace.id.clone();
        let namespace_icon = match namespace.namespace_type {
            NamespaceType::Shared => IconName::Person,
            NamespaceType::Private => IconName::Eye,
        };

        let mut namespace_container = v_flex().w_full().child(
            ListItem::new(("namespace", namespace_idx))
                .spacing(ListItemSpacing::Sparse)
                .start_slot(
                    h_flex()
                        .gap_2()
                        .child(
                            IconButton::new(
                                ("expand-ns", namespace_idx),
                                if namespace.expanded {
                                    IconName::ChevronDown
                                } else {
                                    IconName::ChevronRight
                                },
                            )
                            .on_click(cx.listener(
                                move |this, _event, _window, cx| {
                                    this.toggle_namespace(&namespace_id, cx);
                                },
                            )),
                        )
                        .child(Icon::new(namespace_icon).size(IconSize::Small).color(
                            match namespace.namespace_type {
                                NamespaceType::Shared => Color::Accent,
                                NamespaceType::Private => Color::Warning,
                            },
                        )),
                )
                .child(
                    Label::new(&namespace.name)
                        .size(LabelSize::Default)
                        .color(Color::Default),
                )
                .end_slot(
                    Label::new(match namespace.namespace_type {
                        NamespaceType::Shared => "Shared",
                        NamespaceType::Private => "Private",
                    })
                    .size(LabelSize::Small)
                    .color(Color::Muted),
                ),
        );

        if namespace.expanded {
            let mut subspace_container = v_flex().ml_6();
            for (subspace_idx, subspace) in namespace.subspaces.iter().enumerate() {
                subspace_container = subspace_container.child(self.render_subspace(
                    &namespace.id,
                    subspace,
                    subspace_idx,
                    cx,
                ));
            }
            namespace_container = namespace_container.child(subspace_container);
        }

        namespace_container
    }

    fn render_subspace(
        &self,
        namespace_id: &str,
        subspace: &Subspace,
        subspace_idx: usize,
        cx: &mut Context<Self>,
    ) -> impl gpui::IntoElement {
        let subspace_id = subspace.id.clone();
        let namespace_id_clone = namespace_id.to_string();

        let mut subspace_container = v_flex().w_full().child(
            ListItem::new(("subspace", subspace_idx))
                .spacing(ListItemSpacing::Sparse)
                .start_slot(
                    h_flex()
                        .gap_2()
                        .child(
                            IconButton::new(
                                ("expand-ss", subspace_idx),
                                if subspace.expanded {
                                    IconName::ChevronDown
                                } else {
                                    IconName::ChevronRight
                                },
                            )
                            .on_click(cx.listener(
                                move |this, _event, _window, cx| {
                                    this.toggle_subspace(&namespace_id_clone, &subspace_id, cx);
                                },
                            )),
                        )
                        .child(
                            Icon::new(IconName::Person)
                                .size(IconSize::Small)
                                .color(Color::Accent),
                        ),
                )
                .child(
                    v_flex()
                        .child(
                            Label::new(&subspace.name)
                                .size(LabelSize::Default)
                                .color(Color::Default),
                        )
                        .child(
                            Label::new(&subspace.owner)
                                .size(LabelSize::Small)
                                .color(Color::Muted),
                        ),
                ),
        );

        if subspace.expanded {
            let mut items_container = v_flex().ml_6();
            for (item_idx, item) in subspace.items.iter().enumerate() {
                items_container = items_container.child(self.render_file_item(item, item_idx, cx));
            }
            subspace_container = subspace_container.child(items_container);
        }

        subspace_container
    }

    fn render_file_item(
        &self,
        item: &FileSystemItem,
        item_idx: usize,
        _cx: &mut Context<Self>,
    ) -> impl gpui::IntoElement {
        let icon = if item.is_directory {
            IconName::Folder
        } else {
            match item.name.split('.').last().unwrap_or("") {
                "jpg" | "jpeg" | "png" | "gif" | "bmp" => IconName::Image,
                "pdf" => IconName::File,
                "mp3" | "wav" | "flac" => IconName::File,
                "mp4" | "avi" | "mkv" => IconName::File,
                _ => IconName::File,
            }
        };

        ListItem::new(("file", item_idx))
            .spacing(ListItemSpacing::Sparse)
            .start_slot(
                Icon::new(icon)
                    .size(IconSize::Small)
                    .color(if item.is_directory {
                        Color::Accent
                    } else {
                        Color::Muted
                    }),
            )
            .child(
                h_flex()
                    .justify_between()
                    .w_full()
                    .child(
                        Label::new(&item.name)
                            .size(LabelSize::Default)
                            .color(Color::Default),
                    )
                    .child(
                        h_flex()
                            .gap_4()
                            .children(if let Some(size) = item.size {
                                Some(
                                    Label::new(Self::format_file_size(size))
                                        .size(LabelSize::Small)
                                        .color(Color::Muted),
                                )
                            } else {
                                None
                            })
                            .children(if let Some(modified) = &item.modified {
                                Some(
                                    Label::new(modified)
                                        .size(LabelSize::Small)
                                        .color(Color::Muted),
                                )
                            } else {
                                None
                            }),
                    ),
            )
    }

    fn format_file_size(bytes: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        let mut size = bytes as f64;
        let mut unit_index = 0;

        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }

        if unit_index == 0 {
            format!("{} {}", size as u64, UNITS[unit_index])
        } else {
            format!("{:.1} {}", size, UNITS[unit_index])
        }
    }

    fn render_toolbar(&self, cx: &mut Context<Self>) -> impl gpui::IntoElement {
        h_flex()
            .w_full()
            .justify_between()
            .p_2()
            .border_b_1()
            .border_color(cx.theme().colors().border_variant)
            .bg(cx.theme().colors().toolbar_background)
            .child(
                Label::new("Willow Filesystem")
                    .size(LabelSize::Default)
                    .color(Color::Default),
            )
            .child(
                h_flex()
                    .gap_2()
                    .child(IconButton::new("create-folder", IconName::Folder))
                    .child(IconButton::new("upload-file", IconName::File))
                    .child(IconButton::new("refresh", IconName::ArrowCircle)),
            )
    }
}

pub enum WillowEvent {
    //
}

impl Item for WillowUi {
    type Event = WillowEvent;

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "Willow FS".into()
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
        cx: &mut gpui::Context<Self>,
    ) -> impl gpui::IntoElement {
        v_flex()
            .size_full()
            .bg(cx.theme().colors().panel_background)
            .on_action(
                cx.listener(|_this, _action: &ToggleNamespace, _window, cx| {
                    // Handle toggle namespace action
                    cx.notify();
                }),
            )
            .on_action(cx.listener(|_this, _action: &ToggleSubspace, _window, cx| {
                // Handle toggle subspace action
                cx.notify();
            }))
            .child(self.render_toolbar(cx))
            .child(self.render_breadcrumbs(cx))
            .child(div().flex_1().overflow_hidden().p_3().child({
                let mut flex = v_flex().gap_1();
                for (namespace_idx, namespace) in self.namespaces.iter().enumerate() {
                    flex = flex.child(self.render_namespace(namespace, namespace_idx, cx));
                }
                flex
            }))
    }
}
