use dioxus::prelude::*;
use nbcat::{DisplayIpynb, Opts};
use serde::Deserialize;

mod notebook;
use notebook::Notebook;

mod viewer;
use viewer::IpynbViewer;

    
const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
    IpynbViewer {}
        // FileSelector {  }
    }
}



//  #[component]
//  pub fn FileSelector() -> Element {
//      let selected_file = use_signal(|| Option::<String>::None);
//  
//      rsx! {
//          div {
//              input {
//                  r#type: "file",
//                  onchange: move |evt| {
//                      let files = evt.files();        // Vec<FileData>
//  
//                      if let Some(file) = files.first() {
//                          selected_file.clone().set(Some(file.name().to_string()));
//                      } else {
//                          selected_file.clone().set(None);
//                      }
//                  }
//              }
//  
//              match selected_file() {
//                  Some(name) => rsx! {
//              p { "Selected file: {name}" }
//  
//              let ipynb = serde_json::from_str(&name)?;
//  
//  
//          },
//                  None => rsx! { p { "No file selected" } },
//              }
//          }
//      }
//  }
//  
