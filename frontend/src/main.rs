use leptos::*;
use leptos::html::Video;
use web_sys::HtmlVideoElement;
use js_sys::{Reflect, Function};
use wasm_bindgen::JsValue;
use leptos::prelude::StyleAttribute;
use leptos::mount::mount_to_body;
use leptos::prelude::NodeRef;
use leptos::prelude::NodeRefAttribute;
 use leptos::attr::Scope;
 use leptos::wasm_bindgen::JsCast;
 use web_sys::window;

// ─── Your App component ─────────────────────────────────────────────────────
#[component]
fn App() -> impl IntoView {
    // 1) create a NodeRef for the Leptos Video tag
    let video_ref = NodeRef::<Video>::new();

    // 2) when that <video> actually loads into the DOM, run your HLS logic
    video_ref.on_load(move |video_el: HtmlVideoElement| {
        // pull window.Hls and call .isSupported()
        let hls = Reflect::get(&window().unwrap(), &"Hls".into()).unwrap();
        let is_supported = Reflect::get(&hls, &"isSupported".into()).unwrap()
            .dyn_into::<Function>().unwrap()
            .call0(&JsValue::NULL).unwrap()
            .as_bool().unwrap();

        if is_supported {
            // construct the Hls instance
            let hls_ctor = hls.dyn_into::<Function>().unwrap();
            let hls_instance = Reflect::construct(&hls_ctor, &js_sys::Array::new()).unwrap();

            // hls.loadSource("…")
            Reflect::get(&hls_instance, &"loadSource".into()).unwrap()
                .dyn_into::<Function>().unwrap()
                .call1(&hls_instance, &JsValue::from_str("http://localhost:8000/media/output.m3u8")).unwrap();

            // hls.attachMedia(videoEl)
            Reflect::get(&hls_instance, &"attachMedia".into()).unwrap()
                .dyn_into::<Function>().unwrap()
                .call1(&hls_instance, &video_el.into()).unwrap();
        }
    });
    // 3) render your <video> with that node_ref
    view! { 
        <video
            node_ref=video_ref
            controls
            width="640"
            height="360"
            style="background:black;"
        />
    }
}

fn main() {
    mount_to_body(App);
}
