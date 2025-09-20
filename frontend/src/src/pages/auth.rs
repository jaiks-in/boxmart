use leptos::*;
use leptos_router::*;
use leptos::prelude::*;
use leptos_router::hooks::*;
use crate::components::login::Login;
use crate::components::signup::Signup;

#[component]
pub fn AuthPage()->impl IntoView{
    let (is_logged_in,set_is_logged_in)=signal(false);
    view!{
        <div class="container">
            <div class="row justify-content-center margin-top-5">
                            <Show
                            when=move|| is_logged_in.get()
                            fallback=move||view!{
                                <Login/>
                            }
                            >
                                <Signup/>
                            </Show>
                </div>
            </div>
          
    }
}