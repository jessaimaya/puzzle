use log::{Level};
use log::{info};
use mogwai::prelude::*;
use std::panic;
use wasm_bindgen::prelude::*;
use web_sys::{
    Headers,
    Request,
    Response,
    RequestInit,
    RequestMode,
};

mod api;
mod theme;
mod containers;
mod components;
mod utils;
use crate::theme::Theme;
use crate::containers::*;
use crate::utils::gamedata;
use crate::utils::gamedata::GameData;
use js_sys::Atomics::is_lock_free;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct App {
    clicks: u32,
    loading: bool,
    game_data: Option<GameData>,
}

#[derive(Clone)]
enum AppModel {
    Click,
    Load(bool),
    SetGameData(GameData),
}

#[derive(Clone)]
enum AppView {
    Clicked(u32),
    Loaded(bool),
}

impl Component for App {
    type DomNode = HtmlElement;
    type ModelMsg = AppModel;
    type ViewMsg = AppView;

    fn bind(&self, sub: &Subscriber<AppModel>)  {}

    fn update(&mut self, msg: &AppModel, tx: &Transmitter<AppView>, _sub: &Subscriber<AppModel>) {
        match msg {
            AppModel::Click => {
                self.clicks += 1;
                tx.send(&AppView::Clicked(self.clicks.clone()));
            },
            AppModel::Load(isLoading) => {
                self.loading = isLoading.to_owned();
                tx.send(&AppView::Loaded(self.loading.clone()));
            },
            AppModel::SetGameData(game_data) => {
                info!("game data added!: {:?}", game_data);
                self.game_data = Some(game_data.clone());
            }
        }
    }

    fn view(&self, tx: &Transmitter<AppModel>, rx: &Receiver<AppView>) -> ViewBuilder<HtmlElement> {
        if self.loading {
            let txx = trns();
            let t = tx.clone();
            txx.send_async(async move{
                let game_data = api::fetch_game_data().await.unwrap();
                t.send(&AppModel::Load(false).clone());
                t.send(&AppModel::SetGameData(game_data));
            });
        }
        return builder! {
            <div>
                <h1>{(format!("Loading..."), rx.branch_map(|msg| match msg {
                    AppView::Loaded(v) => match v {
                        false => format!("Done!"),
                        true => format!("Loading...")
                    },
                _ => format!("")
            }))}</h1>
            </div>
        }

    }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(Level::Trace).unwrap();
    let gizmo = Gizmo::from(App{
        clicks: 0,
        loading: true,
        game_data: None,
    });
    let view = View::from(gizmo.view_builder());
    view.run();
    Ok(())
}