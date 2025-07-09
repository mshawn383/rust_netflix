use leptos::*;
use leptos::prelude::*;


fn main() {
    mount_to_body(move || {
        view! {
            <video 
            controls
           id="video"
           width="640"
           height="360"
            />
        }
    });

}
