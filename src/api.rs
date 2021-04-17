use log::{Level};
use log::{info};
use wasm_bindgen::prelude::*;
use mogwai::prelude::{utils, JsFuture};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use snafu::{OptionExt, Snafu, ResultExt};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{
    Headers,
    Request,
    Response,
    RequestInit,
    RequestMode,
};
use crate::utils::gamedata::GameData;
use std::collections::HashMap;

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub")]
pub enum ApiError {
    #[snafu(display("could not call Window object"))]
    CantUseWindow,
    #[snafu(display("could not construct request"))]
    ConstructRequest,
    #[snafu(display("could not create request headers"))]
    CantCreateHeaders,
    #[snafu(display("could not append property to request headers"))]
    CantAppendToHeaders,
    #[snafu(display("could not create Response with url and opts"))]
    CantCreateRequest,
    #[snafu(display("could not fetch with request"))]
    RequestFailure,
    #[snafu(display("response was malformed"))]
    MalformedResponse,
    #[snafu(display("time out awaiting json"))]
    FailedAwaitingJson,
    #[snafu(display("could not convert into GameData"))]
    FailedToParseGameData,
    #[snafu(display("could not fetch GameData file"))]
    FailedToFetchGameData,
    #[snafu(display("could not set item to local storage"))]
    FailedToSetToStorage,
    #[snafu(display("could not get item from local storage"))]
    FailedToGetFromStorage,
}

pub async fn fetch(url: &str) -> Result<JsValue, ApiError> {
    let mut opts = RequestInit::new();
    let headers:Headers = Headers::new().ok().with_context(|| CantCreateHeaders)?;
    headers
        .append("Content-Type", "application/json; charset=utf-8")
        .ok()
        .with_context(|| CantAppendToHeaders)?;
    opts.headers(&headers);
    opts.mode(RequestMode::Cors);
    opts.method("GET");

    let req = Request::new_with_str_and_init(url, &opts)
        .ok()
        .with_context(|| CantCreateRequest)?;

    let resp = JsFuture::from(web_sys::window().with_context(|| CantUseWindow)?.fetch_with_request(&req))
        .await
        .ok()
        .with_context(|| RequestFailure)?
        .dyn_into::<Response>()
        .ok()
        .with_context(|| MalformedResponse)?;

    let js_value: JsValue = JsFuture::from(resp.json().unwrap())
        .await
        .ok()
        .with_context(|| FailedAwaitingJson )?;

    Ok(js_value)
}

pub async fn fetch_game_data() -> Result<GameData, ApiError> {
    info!("Fetch game_data invoked");
    let item = "game_data";
    let game_data: Result<GameData, ApiError> = match  exists_in_storage(item){
        true => {
            let game_str = get_from_storage(item).unwrap();
            let g_data:GameData = serde_json::from_str(&game_str).unwrap();
            Ok(g_data)
        },
        false => {
            let data_url = "https://storage.googleapis.com/bebuzzle/data.json";
            let res = fetch(data_url).await.ok().with_context(|| FailedToFetchGameData)?;
            let g_data: GameData = res.into_serde::<GameData>().unwrap();
            let g_str = serde_json::to_string(&g_data).unwrap();
            set_to_storage(item, &g_str);
            Ok(g_data)
        }
    };

    let game_data = game_data.ok().unwrap();

    load_assets(&game_data).await;

    Ok(game_data)
}

pub async fn load_assets(game_data: &GameData) {
    info!("loading assets....");
    let mut assets:HashMap<String, String> = HashMap::new();
    for cat in &game_data.categories{
      info!("Category: {}", cat.name);
        assets.insert(
            cat.slug.to_string(),
            format!("{}{}", cat.cover.route, cat.cover.filename)
        );
        for lvl in &cat.levels {
            assets.insert(
                lvl.slug.to_string(),
                format!("{}{}", lvl.cover.route, lvl.cover.filename)
            );
            assets.insert(
                format!("{}-{}-bg", cat.slug, lvl.slug),
                format!("{}{}", lvl.background.route, lvl.background.filename)
            );
            for (i, piece) in lvl.pieces.iter().enumerate() {
                assets.insert(
                    format!("{}-{}-p{}", cat.slug, lvl.slug, i),
                    format!("{}{}", piece.img.route ,piece.img.filename)
                );
            }
        }
    }
    info!("Hashmap: {:?}", assets);

}

pub fn exists_in_storage(item: &str) -> bool {
    let storage = mogwai::utils::window()
        .local_storage()
        .unwrap()
        .expect("Could not get local storage");

    match storage.get_item(item) {
        Ok(resp) => {
            match resp {
                Some(_) => true,
                None => false
            }
        },
        _ => false
    }

}

pub fn get_from_storage(key: &str) -> Result<String, ApiError> {
    let storage = mogwai::utils::window()
        .local_storage()
        .unwrap()
        .expect("Could not get local storage");

    match storage.get_item(key) {
        Ok(resp) => {
            match resp {
                Some(v) => Ok(v),
                None => Err(ApiError::FailedToGetFromStorage)
            }
        },
        _ => Err(ApiError::FailedToGetFromStorage)
    }
}

pub fn set_to_storage(key: &str, value: &str) -> Result<(), ApiError> {
    let storage = mogwai::utils::window()
        .local_storage()
        .unwrap()
        .expect("Could not get local storage");

    storage.set_item(key, value).ok().with_context(|| FailedToSetToStorage)?;
    Ok(())
}