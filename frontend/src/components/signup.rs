use leptos::*;
use leptos::prelude::*;
use crate::utils::user_types::CheckPayload;
use crate::helper_functions::form_checker::form_checker;
use leptos::ev::SubmitEvent;
use wasm_bindgen_futures::spawn_local;
use leptos::logging::log;
use gloo_net::http::Request;

#[component]
pub fn Signup() -> impl IntoView {
        let (name ,set_name)=signal("".to_string());
        let (username,set_username)=signal("".to_string());
        let (company_name,set_company_name)=signal("".to_string());
        let (email,set_email)=signal("".to_string());
        let (password,set_password)=signal("".to_string());
        let (confirm_password,set_confirm_password)=signal("".to_string());
        let (is_checked,set_is_checked)=signal("".to_string());
        let (err_message,set_err_message)=signal("".to_string());
        let (user_type,set_user_type)=signal("general".to_string());
        let (show_err,set_show_err)=signal(false);
        let (message,set_message)=signal("".to_string());
            log!("this is from signup{:?}",message.get());
          let submit_handler=move|ev:SubmitEvent|{
            ev.prevent_default();
             let check_payload=CheckPayload{
                id:1,
                name:name.get(),
                username:username.get(),
                org_name:Some(company_name.get()),
                email:email.get(),
                password:password.get(),
                confirm_password:confirm_password.get(),
                user_type:Some(user_type.get())

        };
            let value=form_checker(&check_payload);
            match form_checker(&check_payload){
                Ok(payload)=>{
                    spawn_local(async move {
                            let resp=Request::post("http://localhost:3000/api/auth/signup").header("Content-Type","Application-Json")
                            .json(&payload).unwrap().send().await;

                            match resp{
                                Ok(res)=>{
                                    if res.status()==200{
                                            set_message.set("auth successfull".to_string());
                                            set_show_err.set(false);
                                    }else{
                                        set_err_message.set(format!("same email and username exists{:?}",res.status()));
                                        set_show_err.set(true);
                                    }
                                },
                                Err(err)=>{
                                    set_err_message.set(format!("{:}",err));
                                }
                            }
                    });
                    

                }
                Err(err)=>{
                
                    set_err_message.set(err);
                    set_show_err.set(true);
                }
            }

          };
        
     
    view! {
        <div class="container">
            <div class="row justify-content-center margin-top-5">
                <div class="col-lg-8">
                    <div class="card shadow-lg">
                        <div class="row g-0">
                            
                            // Left branding
                            <div class="col-md-5 bg-primary text-white p-4 d-flex flex-column justify-content-center">
                                <h4 class="fw-bold">"Boxmart"</h4>
                                <p class="mb-1">"Create your account"</p>
                                <small class="mb-3">"Start managing your packaging business with ease."</small>
                                <ul class="list-unstyled small">
                                    <li>"Quick signup"</li>
                                    <li>"Secure authentication"</li>
                                    <li>"Free trial"</li>
                                </ul>
                            </div>

                            // Right signup form
                            <div class="col-md-7 p-4">
                                <h5 class="mb-3">"Sign Up"</h5>
                               <div>
        {move || {
            if show_err.get() {
                view! { <p style="color:red;">{err_message.get()}</p> }
            } else {
                view! { <p style="color:green;">{message.get()}</p> }
            }
        }}
    </div>
                                <form class="row g-3" on:submit=submit_handler >
                                    
                                    <div class="col-md-6">
                                        <label class="form-label">"Full Name"</label>
                                        <input type="text" class="form-control" on:change=move|ev|{set_name.set(event_target_value(&ev))} placeholder="Your name" required/>
                                    </div>
                                      <div class="col-md-6">
                                        <label class="form-label">"Username"</label>
                                        <input type="text" class="form-control" on:change=move|ev|{set_username.set(event_target_value(&ev))} placeholder="Your Username" required/>
                                    </div>

                                    <div class="col-md-6">
                                        <label class="form-label">"Company (optional)"</label>
                                        <input type="text" class="form-control" on:change=move|ev|{set_company_name.set(event_target_value(&ev))} placeholder="Company name"/>
                                    </div>

                                    <div class="col-12">
                                        <label class="form-label">"Email"</label>
                                        <input type="email" class="form-control" on:change=move|ev|{set_email.set(event_target_value(&ev))} placeholder="you@example.com" required/>
                                    </div>

                                    <div class="col-md-6">
                                        <label class="form-label">"Password"</label>
                                        <input type="password" class="form-control" on:change=move|ev|{set_password.set(event_target_value(&ev))} placeholder="********" required/>
                                    </div>

                                    <div class="col-md-6">
                                        <label class="form-label">"Confirm Password"</label>
                                        <input type="password" class="form-control" on:change=move|ev|{set_confirm_password.set(event_target_value(&ev))} placeholder="********" required/>
                                    </div>

                                    <div class="col-12">
                                        <div class="form-check">
                                            <input class="form-check-input" type="checkbox" on:input=move|ev|{set_is_checked.set(event_target_value(&ev))} id="terms" required/>
                                            <label class="form-check-label" for="terms">
                                                "I agree to the " <a href="#">"Terms & Privacy"</a>
                                            </label>
                                        </div>
                                    </div>

                                    <div class="col-12 d-grid">
                                        <button type="submit" class="btn btn-primary">"Create Account"</button>
                                    </div>

                                    <div class="col-12 text-center">
                                        <small class="text-muted">"Already have an account? " <a href="/login">"Login"</a></small>
                                    </div>

                                    <div class="col-12 text-center">
                                        <div class="text-muted my-2">"or continue with"</div>
                                        <div class="d-flex justify-content-center gap-2">
                                            <button type="button" class="btn btn-outline-danger btn-sm">"Google"</button>
                                            <button type="button" class="btn btn-outline-dark btn-sm">"GitHub"</button>
                                            <button type="button" class="btn btn-outline-secondary btn-sm">"Phone"</button>
                                        </div>
                                    </div>

                                </form>
                            </div>

                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}