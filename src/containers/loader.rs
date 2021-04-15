use log::{Level};
use log::info;
use std::sync::{Arc};
use mogwai::prelude::*;
use mogwai::component::subscriber::Subscriber;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    HtmlElement,
};
use wasm_bindgen_futures::JsFuture;
use serde::{Deserialize, Serialize};
use js_sys::{Function, Boolean};
use mogwai::prelude::EventTargetType::Window;
use std::collections::HashMap;

pub struct Loader {
    pub percentage: u32,
}

impl Component for Loader {
    type ModelMsg = ();
    type ViewMsg = ();
    type DomNode = HtmlElement;

     fn bind(&self, input_sub: &Subscriber<Self::ViewMsg>) {
        info!("hello");
        // let r = get_repo(String::from("main")).await;
        // info!("R: {:?}", r);

        /*let req = XmlHttpRequest::new().unwrap();
        let  blob = Blob::new().unwrap();

        let cb = Closure::wrap(Box::new(move |event: &web_sys::ProgressEvent|{
            let total = event.total();
            let loaded = event.loaded();
            let per = (loaded * 100f64) / total;
            info!("callback: {:?}", per);
        })as Box<dyn FnMut(&web_sys::ProgressEvent)>);
        let cb_end = Closure::wrap(Box::new(move | r: &web_sys::XmlHttpRequest| {
            info!("response: {:?}", r.response());
            //let src = Url::create_object_url_with_blob(&blob);
            //info!("src: {:?}", src.ok());
        }) as Box<dyn FnMut(&web_sys::XmlHttpRequest)>);

        req.open_with_async("GET", "https://s2k7tnzlhrpw.statuspage.io/api/v2/status.json", true);
        req.set_response_type(XmlHttpRequestResponseType::Arraybuffer);
        req.set_onprogress(Some(cb.as_ref().unchecked_ref()));
        req.set_onload(Some(cb_end.as_ref().unchecked_ref()));
        req.send();


        cb.forget();
        cb_end.forget();
        */
    }

    fn update(&mut self, msg: &Self::ModelMsg, tx_view: &Transmitter<Self::ViewMsg>, sub: &Subscriber<Self::ModelMsg>) {
    }

    fn view(&self, tx: &Transmitter<Self::ModelMsg>, rx: &Receiver<Self::ViewMsg>) -> ViewBuilder<Self::DomNode> {
        builder!(
            <div id="App">
                <div id="progress_bar">
                    <div id="progress_percentage"></div>
                </div>
            </div>
        )
    }
}