use leptos::*;
use leptos::prelude::*;



#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window)]
    fn Hls() -> JsValue;
}

#[component]
fn App() -> impl IntoView {
    leptos::on_mount(|| {
        let hls = js_sys::Reflect::get(&window().unwrap(), &"Hls".into()).unwrap();
        let is_supported = js_sys::Reflect::get(&hls, &"isSupported".into()).unwrap()
            .dyn_into::<js_sys::Function>().unwrap()
            .call0(&JsValue::NULL).unwrap()
            .as_bool().unwrap();

        if is_supported {
            let video = window().unwrap()
                .document().unwrap()
                .get_element_by_id("video").unwrap()
                .dyn_into::<HtmlVideoElement>().unwrap();

            let hls_instance = js_sys::Reflect::construct(&hls, &js_sys::Array::new()).unwrap();
            let load_source = js_sys::Reflect::get(&hls_instance, &"loadSource".into()).unwrap()
                .dyn_into::<js_sys::Function>().unwrap();
            load_source.call1(&hls_instance, &JsValue::from_str("output.m3u8")).unwrap();

            let attach_media = js_sys::Reflect::get(&hls_instance, &"attachMedia".into()).unwrap()
                .dyn_into::<js_sys::Function>().unwrap();
            attach_media.call1(&hls_instance, &video.into()).unwrap();
        }
    });

    view! {
        <video id="video" controls width="640" height="360" />
    }
}


fn main() {
    mount_to_body(App);

}
