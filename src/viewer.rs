use crate::notebook::Notebook;
use dioxus::prelude::*;

#[component]
pub fn IpynbViewer() -> Element {
    let notebook = use_signal(|| Option::<Notebook>::None);
    let error = use_signal(|| Option::<String>::None);

    rsx! {
        div {
            input {
        id: "fileselector",
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
			    id: "notebook",
			    h2 { "Notebook:" }
			    
			    for cell in &nb.cells {
				div {
				    id: match cell.cell_type.as_str() {
					"markdown" => "markdowncell",
					"code" => "codecell",
					"output" => "outputcell",
					_ => "unknowncell",
				    },
				    style: {
					match cell.cell_type.as_str() {
					    "code" => "
                                    background-color: #1e1e1e;
                                    color: #d4d4d4;
                                    padding: 10px;
                                    border-radius: 6px;
                                ",
					    _ => ""
					}
				    },
				    
				    hr {}
				    h3 { "Cell type: {cell.cell_type}" }
				    
				    pre {
					for line in &cell.source {
					    "{line}"
					}
				    }
				    
				    hr {}
				    p { "Output:" }
				    
				    pre {
					for output in &cell.outputs {
					    "{output}"
					}
				    }
				    
				    hr {}
				}
			    }
			}
		    }
		} else {
		    rsx! {
			div {
			    id: "usererror",
			    p { "No notebook loaded." }
			}
		    }
		}
	    }
	    

 //              {
 //                  let nb = notebook.read();
 //                  if let Some(nb) = nb.as_ref() {
 //                      rsx! {
 //                          div {
 //  			    id: "notebook",
 //                              h2 { "Notebook:" }
 //  			    
 //                              for cell in &nb.cells {
 //  				
 //  				div {
 //  				    id:  match cell.cell_type.as_str() {
 //  					"markdown" => "markdowncell",
 //  					"code" =>  "codecell",
 //  					"output" => "outputcell",
 //  					_ =>  "unknowncell",
 //  				    },
 //  				    
 //  				    hr {}
 //                                      h3 { "Cell type: {cell.cell_type}" }
 //                                      pre {
 //  					for line in &cell.source {
 //                                              "{line}"
 //  					}
 //                                      }
 //  				
 //  				    hr{}
 //  				    p { "Output:" }
 //  				    pre {
 //  					for output in &cell.outputs {
 //  					    div {
 //  						id: "outputcell"
 //  					    },
 //  					    "{output}"
 //  					}
 //  				    }
 //  				hr {}
 //  				}
 //                              }
 //  			}
 //                      }
 //  		} else {
 //  		    rsx!(
 //  			div {
 //  			    id: "usererror",
 //  			    p {
 //  				"No notebook loaded."
 //  			    }
 //  			}
 //  		    )
 //  		}
 //              } // end let
	}
    }
}
