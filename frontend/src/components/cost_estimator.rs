use leptos::*;
use leptos::prelude::*;
use gloo_net::http::Request;
use leptos::logging::log;
use crate::utils::paper_types::Paper;
use std::collections::HashSet;
use leptos::ev::SubmitEvent;

#[component]
pub fn CostEstimator() -> impl IntoView {
    let (data, set_data) = signal(Vec::<Paper>::new());
    let (err_msg, set_err_msg) = create_signal(String::new());
    let (box_type, set_box_type) = signal("".to_string());
    let (paper_brand,set_paper_brand)=signal(String::new());
    let (paper_gsm,set_paper_gsm)=signal(String::new());
    let (roll_type,set_roll_type)=signal(String::new());
    let (paper_variety,set_paper_variety)=signal(String::new());
    

    // Fetch data once on mount
    let _resource = LocalResource::new(move || async move {
        log!("Fetching paper options...");
        match Request::get("http://127.0.0.1:3000/api/get_paper_option")
            .send()
            .await
        {
            Ok(resp) => match resp.json::<Vec<Paper>>().await {
                Ok(papers) => {
                    log!("Fetched papers: {:?}", papers);
                    set_data.set(papers.clone());
                    Ok(papers)
                }
                Err(err) => {
                    log!("JSON parsing error: {}", err);
                    set_err_msg.set(err.to_string());
                    Err(err.to_string())
                }
            },
            Err(err) => {
                log!("Request failed: {}", err);
                set_err_msg.set(err.to_string());
                Err(err.to_string())
            }
        }
    });
    let mut product_type=vec!["duplex".to_string(),"corrugated".to_string(),"visiting_card".to_string(),"tags".to_string()];
    let mut roll_type=vec!["Agro".to_string(),"Gold".to_string(),"Semi-Kraft".to_string(),"Kraft".to_string()];
   

// submit handler (closure hi banana padega)
let calc_handler = move |ev: leptos::ev::SubmitEvent| {
    ev.prevent_default(); // form ke default submit ko rokne ke liye
     let selected_brand = {
    let paper_brand = paper_brand.clone();
    let paper_variety_clone=paper_variety.get();
    let data = data.clone();
    move || {
        let brand = paper_brand.get();
        let paper_variety=paper_variety.clone();
        let value = data
            .get()
            .iter()
            .find(|paper| paper.name == brand  && paper.variety==paper_variety_clone)
            .map(|paper| paper.cost_per_kg);

        log!("Selected brand cost: {:?}", value);
        value
    }
};
    let _ = selected_brand(); // call closure
};

    view! {
        <div class="card shadow-sm">
            <div class="card-header bg-primary text-white">
                <h5 class="mb-0">"Cost Estimator"</h5>
            </div>
            <div class="card-body">
                <form on:submit=calc_handler>
                    <div class="mb-3">
                        <label class="form-label">"Product Type"</label>
                        <select class="form-select"
                            on:change=move |ev| {
                                let value = event_target_value(&ev);
                                set_box_type.set(value);
                            }
                        >
                            <option selected>"Select type"</option>
                            {move || {
                                product_type.iter().map(|ptype| {
                                    view! {
                                        // assuming your struct has a field `name`
                                        <option value={ptype.clone()}>
                                            {ptype.clone()}
                                        </option>
                                    }
                                }).collect_view()
                            }}
                        </select>
                    </div>
                        <div class="mb-3">
                        <label class="form-label">"Paper Brand"</label>
                        <select class="form-select"
                            on:change=move |ev| {
                                let value = event_target_value(&ev);
                                set_paper_brand.set(value);
                                log!("{:?},{:?}",box_type.get(),paper_brand.get());
                            }
                        >
                            <option selected>"Select brand"</option>
                            {move || {
                                let unique: Vec<Paper> = {
                                    let mut seen = HashSet::new();
                                    data.get().into_iter().filter(|p| seen.insert(p.name.clone())).collect()};
                                unique.iter().map(|ptype| {
                                    view! {
                                        // assuming your struct has a field `name`
                                        <option value={ptype.name.clone()}>
                                            {ptype.name.clone()}
                                        </option>
                                    }
                
                                }).collect_view()
                            }}
                        </select>
                    </div>
                    <div class="mb-3">
                        <label class="form-label">"Paper Type"</label>
                        <select class="form-select"
                            on:change=move |ev| {
                                let value = event_target_value(&ev);
                                set_paper_variety.set(value);
                            }
                        >
                            <option selected>"Paper Type"</option>
                            {move || {
                                let unique: Vec<Paper> = {
                                    let mut seen = HashSet::new();
                                    data.get().into_iter().filter(|p| seen.insert(p.variety.clone())).collect()};
                                unique.iter().map(|ptype| {
                                    view! {
                                        // assuming your struct has a field `name`
                                        <option value={ptype.variety.clone()}>
                                            {ptype.variety.clone()}
                                        </option>
                                    }
                
                                }).collect_view()
                            }}
                        </select>
                    </div>
                      <div class="mb-3">
                        <label class="form-label">"Roll Type"</label>
                        <select class="form-select"
                            on:change=move |ev| {
                                let value = event_target_value(&ev);
                                set_roll_type.set(value);
                            }
                        >
                            <option selected>"Select Type"</option>
                            {move || {
                                roll_type.iter().map(|ptype| {
                                    view! {
                                        // assuming your struct has a field `name`
                                        <option value={ptype.clone()}>
                                            {ptype.clone()}
                                        </option>
                                    }
                                }).collect_view()
                            }}
                        </select>
                    </div>
                    <div class="mb-3">
                        <label class="form-label">"Paper GSM"</label>
                        <select class="form-select"
                            on:change=move |ev| {
                                let value = event_target_value(&ev);
                                set_paper_gsm.set(value);
                            }
                        >
                            <option selected>"Paper Gsm"</option>
                             {move || {
                                let unique: Vec<Paper> = {
                                    let mut seen = HashSet::new();
                                    data.get().into_iter().filter(|p| seen.insert(p.gsm.clone())).collect()};
                                unique.iter().map(|ptype| {
                                    view! {
                                        // assuming your struct has a field `name`
                                        <option value={ptype.gsm.clone()}>
                                            {ptype.gsm.clone()}
                                        </option>
                                    }
                
                                }).collect_view()
                            }}
                        </select>
                    </div>
                   <div class="col">
                            <label class="form-label">"Breadth"</label>
                            <input type="number" class="form-control" placeholder="cm"/>
                        </div>
                        <div class="col">
                            <label class="form-label">"Height"</label>
                            <input type="number" class="form-control" placeholder="cm"/>
                        </div>
                    <div class="col">
                            <label class="form-label">"Length"</label>
                            <input type="number" class="form-control" placeholder="cm"/>
                        </div>

                    <button type="submit" class="btn btn-success w-100">"Estimate Cost"</button>
                </form>
            </div>
            <div class="card-footer text-center">
                <strong>"Total: 0.00"</strong>
            </div>
        </div>
    }
}
