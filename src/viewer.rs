use dioxus::prelude::*;
use crate::notebook::Notebook;


#[component]
pub fn IpynbViewer() -> Element {
    let notebook = use_signal(|| Option::<Notebook>::None);
    let error = use_signal(|| Option::<String>::None);

    rsx! {
        div {
            input {
                r#type: "file",
                accept: ".ipynb",

                onchange: move |evt| {
                    if let Some(file) = evt.files().into_iter().next() {

                        spawn({
                            let mut notebook = notebook.clone();
                            let mut error = error.clone();

                            async move {
                                match file.read_bytes().await {
                                    Ok(data) => {
                                        match serde_json::from_slice::<Notebook>(&data) {
                                            Ok(nb) => {
                                                notebook.set(Some(nb));
                                                error.set(None);
                                            }
                                            Err(e) => {
                                                error.set(Some(format!("JSON parse error: {e}")));
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        error.set(Some(format!("Failed to read file: {e}")));
                                    }
                                }
                            }
                        });
                    }
                }
            }

            {
                let err = error.read();
                if let Some(e) = err.as_ref() {
                    rsx!( p { style: "color: red;", "Error: {e}" } )
                } else {
                    rsx!( Fragment {} )
                }
            }

            {
                let nb = notebook.read();
                if let Some(nb) = nb.as_ref() {
                    rsx! {
                        div {
                            h2 { "Notebook:" }

                            for cell in &nb.cells {
				
                                div {
				    hr {}
                                    h3 { "Cell type: {cell.cell_type}" }
                                    pre {
                                        for line in &cell.source {
                                            "{line}"
                                        }
                                    }
				    hr {}
                                }
                            }
                        }
                    }
                } else {
                    rsx!( p { "No notebook loaded." } )
                }
            }
        }
    }
}
