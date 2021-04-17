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

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Clone)]
pub enum MenuType {
    Categories,
    Levels,
}

#[derive(Clone)]
pub enum ViewState {
    Loading,
    Menu,
    Levels,
    Game(String, u8), // cat: Pirates - lvl: 2
    Settings
}

pub struct App {
    current_category: String,
    current_level: u8,
    game_data: Option<GameData>,
    view_state: ViewState,
}

#[derive(Clone)]
pub enum AppModel {
    // Load,
    SetGameData(GameData),
    ChangeView(ViewState),
}

#[derive(Clone)]
pub enum AppView {
    Loading(Patch<View<HtmlElement>>),
    Menu(Patch<View<HtmlElement>>),
    Levels(Patch<View<HtmlElement>>),
    Game(Patch<View<HtmlElement>>),
    ChangeState(Patch<View<HtmlElement>>),
    Settings(Patch<View<HtmlElement>>),
}

impl AppView {
    fn patch_view(&self) -> Option<Patch<View<HtmlElement>>> {
        match self {
            AppView::ChangeState(patch) => Some(patch.clone()),
            _ => Some(Patch::Replace {index:0, value: View::from(ViewBuilder::from(builder!{<h1>"Default"</h1>}))})
        }
    }
}

impl Component for App {
    type DomNode = HtmlElement;
    type ModelMsg = AppModel;
    type ViewMsg = AppView;

    fn bind(&self, sub: &Subscriber<AppModel>)  {}

    fn update(&mut self, msg: &AppModel, tx: &Transmitter<AppView>, sub: &Subscriber<AppModel>) {
        match msg {
            AppModel::ChangeView(view) => {
                info!("here we should change the state");
                self.view_state = view.clone();
                let to_show: ViewBuilder<HtmlElement> = match view {
                    ViewState::Loading => {
                        info!("loading...");
                        let b = builder! {<h1>"Loading..."</h1>};
                        b
                    },
                    ViewState::Menu => builder! {<h1>"Categories"</h1>},
                    ViewState::Levels => builder! {<h1>"Levels"</h1>},
                    ViewState::Game(category, level) => builder!{<h1>"let's play"</h1>},
                    ViewState::Settings => builder!{<h1>"Settings"</h1>},
                    _ => builder! {<h1>"404"</h1>}
                };
                tx.send(&AppView::ChangeState(Patch::Replace {index:0, value: View::from(ViewBuilder::from(to_show))}));
            },
            AppModel::SetGameData(data) => {
                self.game_data = Some(data.clone());
            }
        }
    }

    fn view(&self, tx: &Transmitter<AppModel>, rx: &Receiver<AppView>) -> ViewBuilder<HtmlElement> {
        match self.view_state  {
          ViewState::Loading => {
              info!("I'm loading!");
              let txx = trns();
              let t = tx.clone();
              txx.send_async(async move{
                  let game_data = api::fetch_game_data().await.unwrap();
                  t.send(&AppModel::SetGameData(game_data));
                  t.send(&AppModel::ChangeView(ViewState::Menu));
              });
          },
            _ => info!("meee")
        };

        return builder!{
            <div
                patch:children=rx.branch_filter_map(AppView::patch_view)
            >
                "Bebuzzle"
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(Level::Trace).unwrap();
    let gizmo = Gizmo::from(App{
        current_category: String::from(""),
        current_level: 0,
        game_data: None,
        view_state: ViewState::Loading,
    });
    gizmo.trns.send(&AppModel::ChangeView(ViewState::Loading));
    let view = View::from(gizmo.view_builder());
    view.run();
    Ok(())
}