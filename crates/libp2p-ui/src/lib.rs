use std::collections::BTreeSet;

use libp2p::{PeerId, mdns::Event as MdnsEvent};
use libp2p_swarm::SwarmEvent;
use zed::unstable::{
    db::smol::stream::StreamExt,
    gpui::{self, Action, AppContext, EventEmitter, FocusHandle, Focusable, actions, rgb},
    ui::{
        App, Context, IconName, IntoElement, ListItem, ParentElement, Pixels, Render, Styled,
        Window, div, px,
    },
    workspace::{
        Panel, Workspace,
        dock::{DockPosition, PanelEvent},
    },
};

use crate::p2p::{PeerieBehaviour, PeerieBehaviourEvent};

pub mod p2p;

actions!(workspace, [ToggleLibp2pPanel]);

pub fn init(cx: &mut App) {
    let libp2p_ui = cx.new(|cx| Libp2pUi::new(cx));

    cx.spawn({
        let libp2p_ui = libp2p_ui.clone();
        async move |cx| {
            let mut swarm = PeerieBehaviour::try_init_swarm()?;
            while let Some(event) = swarm.next().await {
                tracing::info!(?event, "Emitting SwarmEvent");
                libp2p_ui.update(cx, |_ui, cx| {
                    cx.emit(event);
                })?;
            }
            tracing::warn!("Ending Swarm task");
            anyhow::Ok(())
        }
    })
    .detach_and_log_err(cx);

    cx.observe_new(move |workspace: &mut Workspace, window, cx| {
        let Some(window) = window else { return };
        workspace.add_panel(libp2p_ui.clone(), window, cx);

        workspace.register_action(|workspace, _: &ToggleLibp2pPanel, window, cx| {
            workspace.toggle_panel_focus::<Libp2pUi>(window, cx);
        });
    })
    .detach();
}

struct Libp2pUi {
    dock_position: DockPosition,
    focus_handle: FocusHandle,
    width: Option<Pixels>,
    peers: BTreeSet<PeerId>,
}

impl Libp2pUi {
    pub fn new(cx: &mut Context<Self>) -> Self {
        cx.subscribe_self(|this, event: &SwarmEvent<PeerieBehaviourEvent>, cx| {
            match event {
                SwarmEvent::Behaviour(PeerieBehaviourEvent::Mdns(MdnsEvent::Discovered(peers))) => {
                    for (peer_id, _addr) in peers {
                        tracing::info!(?peer_id, "Added peer");
                        this.peers.insert(*peer_id);
                    }
                    cx.notify();
                }
                SwarmEvent::Behaviour(PeerieBehaviourEvent::Mdns(MdnsEvent::Expired(peers))) => {
                    for (peer_id, _addr) in peers {
                        tracing::info!(?peer_id, "Removed peer");
                        this.peers.remove(peer_id);
                    }
                    cx.notify();
                }
                SwarmEvent::ConnectionEstablished {
                    peer_id,
                    connection_id,
                    endpoint,
                    num_established,
                    concurrent_dial_errors,
                    established_in,
                } => {
                    tracing::info!(?peer_id, "Added peer");
                    this.peers.insert(*peer_id);
                    cx.notify();
                }
                SwarmEvent::ConnectionClosed {
                    peer_id,
                    connection_id,
                    endpoint,
                    num_established,
                    cause,
                } => {
                    tracing::info!(?peer_id, "Removed peer");
                    this.peers.remove(peer_id);
                    cx.notify();
                }
                _ => {
                    //
                }
            }
        })
        .detach();

        Self {
            dock_position: DockPosition::Left,
            focus_handle: cx.focus_handle(),
            width: None,
            peers: Default::default(),
        }
    }
}

impl Render for Libp2pUi {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .border_1()
            .border_color(rgb(0xaa00bb))
            .child(
                div()
                    .w_full()
                    .border_1()
                    .border_color(rgb(0x440099))
                    .child("Peers:"),
            )
            .children(
                self.peers
                    .iter()
                    .enumerate()
                    .map(|(i, it)| ListItem::new(i).child(it.to_string())),
            )
    }
}

impl EventEmitter<PanelEvent> for Libp2pUi {}
impl EventEmitter<SwarmEvent<PeerieBehaviourEvent>> for Libp2pUi {}

impl Focusable for Libp2pUi {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Panel for Libp2pUi {
    fn persistent_name() -> &'static str {
        "Libp2p"
    }

    fn panel_key() -> &'static str {
        "libp2p"
    }

    fn position(&self, _window: &Window, _cx: &App) -> DockPosition {
        self.dock_position
    }

    fn position_is_valid(&self, _position: DockPosition) -> bool {
        true
    }

    fn set_position(
        &mut self,
        position: DockPosition,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) {
        self.dock_position = position;
    }

    fn size(&self, _window: &Window, _cx: &App) -> Pixels {
        self.width.unwrap_or_else(|| px(300.))
    }

    fn set_size(&mut self, size: Option<Pixels>, _window: &mut Window, _cx: &mut Context<Self>) {
        self.width = size;
    }

    fn icon(&self, _window: &Window, _cx: &App) -> Option<IconName> {
        Some(IconName::Link)
    }

    fn icon_tooltip(&self, _window: &Window, _cx: &App) -> Option<&'static str> {
        Some("Libp2p")
    }

    fn toggle_action(&self) -> Box<dyn Action> {
        Box::new(ToggleLibp2pPanel)
    }

    fn activation_priority(&self) -> u32 {
        0
    }
}
