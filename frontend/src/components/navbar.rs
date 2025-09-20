use leptos::*;
use leptos_router::*;
use leptos::prelude::*;
use leptos_router::hooks::*;
use crate::components::dropdown::Dropdown;
use crate::components::login::Login;
#[component]
pub fn NavBar() -> impl IntoView {
    let navigate_home = use_navigate();
    let navigate_contacts=use_navigate();
    let navigate_login=use_navigate();
    let (is_logged_in, set_is_logged_in) = create_signal(false);
    
    let handle_login = move |_| {
       navigate_login("/login", NavigateOptions::default());
    };

    let handle_logout = move |_| {
        set_is_logged_in.set(false);
    };

    let render_home_page = move |_| {
        navigate_home("/", NavigateOptions::default());
    };

    let render_contact_details = move |_| {
        println!("get_all_contact_details");
        navigate_contacts("/contacts", NavigateOptions::default());
    };

    view! {
        <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
            <div class="container-fluid">
                <span class="navbar-brand">"Boxmart"</span>
                <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarNav">
                    <span class="navbar-toggler-icon"></span>
                </button>
                <div class="collapse navbar-collapse" id="navbarNav">
                    <ul class="navbar-nav">
                        <li class="nav-item">
                            <a class="nav-link" on:click=render_home_page style="cursor: pointer;">"Home"</a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link" style="cursor: pointer;">"Products"</a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link" on:click=render_contact_details style="cursor: pointer;">"Contact"</a>
                        </li>
                    </ul>
                </div>

                <div class="d-flex">
                    <Show
                        when=move || is_logged_in.get()
                        fallback=move || view! {
                            
                            <button class="btn btn-outline-success" on:click=handle_login.clone()>"Login"</button>
                        }
                    >
                        <Dropdown handle_logout=handle_logout/>
                    </Show>
                </div>
            </div>
        </nav>
    }
}