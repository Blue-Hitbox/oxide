use gloo_net::http::Request;
use serde::Deserialize;
use yew::prelude::*;

#[derive(Clone, PartialEq, Deserialize)]
struct FileInfo {
    name: String,
    is_dir: bool,
}

#[function_component(App)]
fn app() -> Html {
    let files = use_state(|| vec![]);
    {
        let files = files.clone();
        use_effect_with((), move |_| {
            let files = files.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_files: Vec<FileInfo> = Request::get("http://localhost:3000/api/files")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                files.set(fetched_files);
            });
            || ()
        });
    }

    let file_list = files
        .iter()
        .map(|file| {
            html! {
                <li key={file.name.clone()}>
                    { if file.is_dir { "📁 " } else { "📄 " } }
                    { &file.name }
                </li>
            }
        })
        .collect::<Html>();

    html! {
        <div style="display: flex; height: 100vh; font-family: sans-serif;">
            <div style="width: 250px; border-right: 1px solid #ccc; padding: 10px; background: #f5f5f5;">
                <h3>{"Oxide IDE"}</h3>
                <ul>
                    { file_list }
                </ul>
            </div>
            <div style="flex: 1; padding: 20px;">
                <h2>{"Editor"}</h2>
                <textarea style="width: 100%; height: 80%; font-family: monospace; padding: 10px;" placeholder="Select a file to edit..."></textarea>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
