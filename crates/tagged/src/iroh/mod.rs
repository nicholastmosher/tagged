use futures_concurrency::stream::Merge as _;
use iroh::Endpoint;
use tracing::{error, warn};
use zed::unstable::{
    db::smol::stream::{Stream, StreamExt},
    gpui::{AppContext, Global},
    ui::App,
};

pub fn init(cx: &mut App) {
    let (iroh, task) = Iroh::init();
    let _handle = tokio::task::spawn(task);
    cx.set_global(GlobalIroh(iroh));

    //
    let iroh = cx.iroh();
}

struct GlobalIroh(Iroh);
impl Global for GlobalIroh {}

enum IrohInput {
    //
    Initialized(Endpoint),
    Shutdown,
}

/// External handle API for iroh operations
#[derive(Clone)]
pub struct Iroh {
    tx: flume::Sender<IrohInput>,
}

impl Iroh {
    fn init() -> (Self, impl Future<Output = ()>) {
        let (tx, rx) = flume::bounded(10);

        let engine = IrohEngine::new();
        let future = engine.run(tx.clone(), rx);

        let iroh = Iroh {
            //
            tx,
        };
        (iroh, future)
    }
}

/// Internal engine state
pub struct IrohEngine {
    //
    endpoint: Option<Endpoint>,
}

impl IrohEngine {
    pub fn new() -> Self {
        IrohEngine { endpoint: None }
    }

    async fn create_input_stream(
        &self,
        rx: flume::Receiver<IrohInput>,
    ) -> impl Stream<Item = IrohInput> + use<> {
        let rx_stream = rx.into_stream();
        let stream = (rx_stream,).merge();
        stream
    }

    async fn run(
        //
        mut self,
        tx: flume::Sender<IrohInput>,
        rx: flume::Receiver<IrohInput>,
    ) {
        //
        let it = tokio::task::spawn(async move {
            let endpoint = Endpoint::builder().bind().await?;
            tx.send(IrohInput::Initialized(endpoint))?;
            anyhow::Ok(())
        });
        let mut input_stream = self.create_input_stream(rx).await;
        loop {
            //
            let result = self.try_run(&mut input_stream).await;
            if let Err(error) = result {
                error!(?error, "Error in IrohEngine");
            } else {
                warn!("Iroh Engine shutdown");
                return;
            }
        }
    }

    async fn try_run(
        &mut self,
        input: &mut (impl Unpin + Stream<Item = IrohInput>),
    ) -> anyhow::Result<()> {
        while let Some(event) = input.next().await {
            match event {
                IrohInput::Initialized(endpoint) => {
                    self.endpoint = Some(endpoint);
                }
                IrohInput::Shutdown => {
                    return Ok(());
                }
            }
        }

        Ok(())
    }
}

pub trait IrohExt {
    //
    fn iroh(&self) -> Iroh;
}

impl<'a, C: AppContext> IrohExt for &'a mut C {
    fn iroh(&self) -> Iroh {
        self.read_global::<GlobalIroh, _>(|it, _cx| it.0.clone())
    }
}
