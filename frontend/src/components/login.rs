use leptos::*;
use leptos::prelude::*;
use leptos::ev::SubmitEvent;
use crate::utils::user_types::LoginPayload;
use crate::helper_functions::login_form_checker::login_form_checker;
use gloo_net::http::Request;
use wasm_bindgen_futures::spawn_local;
use leptos_router::hooks::use_navigate;
use leptos_router::*;

#[component]
pub fn Login() -> impl IntoView {
    let (email,set_email) = signal(String::new());
    let (password,set_password) = signal(String::new());
    let (show_err,set_show_err) = signal(false);
    let (err_message,set_err_message) = signal("".to_string());
    let (success,set_success) = signal("".to_string());
    let mut  to_products=use_navigate();
    let mut submit_handler= move |ev: SubmitEvent| {
        ev.prevent_default();

        let email = email.get();
        let password = password.get();
        let navigate=to_products.clone();

        spawn_local(async move {
            // Prepare payload
            let unchecked_payload = LoginPayload {
                email: email.clone(),
                password: password.clone(),
            };

            // Validate form
            let payload = match login_form_checker(unchecked_payload) {
                Ok(val) => val,
                Err(err) => {
                    set_err_message.set(err);
                    set_show_err.set(true);
                    return;
                }
            };

            // Send request
            let response = Request::post("http://localhost:3000/api/auth/login")
                .header("Content-Type", "application/json")
                .json(&payload)
                .unwrap()
                .send()
                .await;

            match response {
                Ok(resp) => {
                    if resp.status() == 200 {
                        let text = resp.text().await.unwrap_or_default();
                        set_success.set(text);
                        set_show_err.set(false);
                         navigate("/api/products",NavigateOptions::default());
                    } else {
                        set_err_message.set(format!("Error: {}", resp.status()));
                        set_show_err.set(true);
                    }
                }
                Err(err) => {
                    set_err_message.set(format!("Request failed: {:?}", err));
                    set_show_err.set(true);
                }
            }
        });
    };

    view! {
        <div class="container">
            <div class="row justify-content-center">
                <div class="col-lg-5">
                    <div class="card shadow-lg border-0">
                        <div class="card-body p-4">
                            <h4 class="text-center mb-3">"Login"</h4>
                            <p class="text-center text-muted small">"Welcome back! Please sign in to your account."</p>

                            <form class="row g-3" on:submit=submit_handler>

                                <div class="col-12">
                                    <label class="form-label">"Email address"</label>
                                    <input type="email" class="form-control"
                                        on:change=move|ev|{ set_email.set(event_target_value(&ev)) }
                                        placeholder="you@example.com" required/>
                                </div>

                                <div class="col-12">
                                    <label class="form-label">"Password"</label>
                                    <input type="password" class="form-control"
                                        on:change=move|ev|{ set_password.set(event_target_value(&ev)) }
                                        placeholder="********" required/>
                                </div>

                                <div class="col-12 d-flex justify-content-between align-items-center">
                                    <div class="form-check">
                                        <input class="form-check-input" type="checkbox" id="remember"/>
                                        <label class="form-check-label" for="remember">"Remember me"</label>
                                    </div>
                                    <a href="#" class="small text-decoration-none">"Forgot password?"</a>
                                </div>

                                <div class="col-12 d-grid">
                                    <button type="submit" class="btn btn-primary">"Login"</button>
                                </div>

                                <div class="col-12 text-center">
                                    <small class="text-muted">"Don't have an account? " <a href="/signup">"Sign up"</a></small>
                                </div>

                                <div class="col-12 text-center">
                                    <div class="text-muted my-2">"or continue with"</div>
                                    <div class="d-flex justify-content-center gap-2">
                                        <button type="button" class="btn btn-outline-danger btn-sm">"Google"</button>
                                        <button type="button" class="btn btn-outline-dark btn-sm">"GitHub"</button>
                                        <button type="button" class="btn btn-outline-secondary btn-sm">"Phone"</button>
                                    </div>
                                </div>

                                {move || if show_err.get() {
                                    view! { <p class="text-danger">{err_message.get()}</p> }
                                } else {
                                    view! { <p class="text-success">{success.get()}</p> }
                                }}

                            </form>

                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
