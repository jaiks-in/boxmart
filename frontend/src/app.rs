// In your main App component
use leptos::prelude::*;
use leptos_router::*;
use leptos::prelude::ElementChild;
use leptos_router::components::{Routes,Router,Route};
use crate::components::navbar::NavBar;
use crate::pages::homepage::HomePage;
use crate::pages::contactpage::ContactPage;
use crate::pages::auth::AuthPage;
use crate::components::signup::Signup;
use crate::pages::products::products;
#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
        <main>
            <NavBar/>
            <Routes fallback=|| "Not found.">
            <Route path=path!("/") view=HomePage/>
            <Route path=path!("/contacts") view=ContactPage />
            <Route path=path!("/signup") view=Signup  />
            <Route path=path!("/login") view =AuthPage/>
            <Route path=path!("products") view=products  />
            <Route path=path!("/*any") view=|| view! { <h1>"Not Found"</h1> }/>
            </Routes>
        </main>
        </Router>

    }}
