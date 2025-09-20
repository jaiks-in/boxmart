use leptos::*;
use leptos::prelude::*;

#[component]
pub fn Dropdown(
    handle_logout: impl Fn(ev::MouseEvent) + 'static
) -> impl IntoView {
    view! {
        <div class="dropdown">
            <a class="d-flex align-items-center text-white text-decoration-none dropdown-toggle"
                href="#"
                id="userMenu"
                data-bs-toggle="dropdown"
                aria-expanded="false">
                <img src="https://www.w3schools.com/howto/img_avatar.png"
                    alt="avatar"
                    width="32"
                    height="32"
                    class="rounded-circle me-2"/>
                <strong>"Jai"</strong>
            </a>
            <ul class="dropdown-menu dropdown-menu-end dropdown-menu-dark text-small"
                aria-label="userMenu">
                <li><a class="dropdown-item" href="#">"Profile"</a></li>
                <li><a class="dropdown-item" href="#">"Settings"</a></li>
                <li><hr class="dropdown-divider"/></li>
                <li><button class="dropdown-item" on:click=handle_logout> "Sign out" </button></li>
            </ul>
        </div>
    }
}