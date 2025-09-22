use leptos::*;
use leptos::prelude::*;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use leptos::logging::log;
use crate::utils::paper_types::Paper;
#[component]
pub fn CostEstimator() -> impl IntoView {
    let (data, set_data) = create_signal(Vec::<String>::new());
let (err_msg, set_err_msg) = create_signal(String::new());
  let async_data = LocalResource::new(|| async {
        log!("Starting to fetch paper options...");
        
        match Request::get("http://127.0.0.1:3000/api/get_paper_option")
            .send()
            .await
        {
            Ok(resp) => {
                log!("Response received, status: {}", resp.status());
                
                // Parse as Vec<Paper> - array of paper objects
                let result = resp.json::<Vec<Paper>>()
                    .await
                    .map_err(|e| e.to_string());
                
                match &result {
                    Ok(papers) => {
                        log!("Successfully fetched {} paper options", papers.len());
                        log!("Sample paper: {:?}", papers);
                    }
                    Err(err) => {
                        log!("JSON parsing error: {}", err);
                    }
                }
                
                result
            }
            Err(err) => {
                log!("Request failed: {}", err);
                Err(err.to_string())
            }
        }
    });


    view! {
        <div class="card shadow-sm">
            <div class="card-header bg-primary text-white">
                <h5 class="mb-0">"Cost Estimator"</h5>
            </div>
            <div class="card-body">
                <form>
                    <div class="mb-3">
                        <label class="form-label">"Product Type"</label>
                        <select class="form-select">
                            <option selected>"Roll"</option>
                            <option>"Roll"</option>
                            <option>"Sheet"</option>
                            <option>"Box"</option>
                        </select>
                    </div>
                    <div class="mb-3">
                        <label class="form-label">"Quantity"</label>
                        <input type="number" class="form-control" placeholder="Enter quantity"/>
                    </div>
                    <div class="mb-3">
                        <label class="form-label">"Size"</label>
                        <input type="text" class="form-control" placeholder="e.g. 40x23 cm"/>
                    </div>
                    <button type="button" class="btn btn-success w-100">"Estimate Cost"</button>
                </form>
            </div>
            <div class="card-footer text-center">
                <strong>"Total: 0.00"</strong>
            </div>
        </div>
    }
}
