use leptos::*;
use leptos::prelude::*;

#[component]
pub fn Login() -> impl IntoView {
    view! {
        <div class="container">
            <div class="row justify-content-center">
                <div class="col-lg-5">
                    <div class="card shadow-lg border-0">
                        <div class="card-body p-4">
                            
                            <h4 class="text-center mb-3">"Login"</h4>
                            <p class="text-center text-muted small">"Welcome back! Please sign in to your account."</p>

                            <form class="row g-3">

                                <div class="col-12">
                                    <label class="form-label">"Email address"</label>
                                    <input type="email" class="form-control" placeholder="you@example.com" required/>
                                </div>

                                <div class="col-12">
                                    <label class="form-label">"Password"</label>
                                    <input type="password" class="form-control" placeholder="********" required/>
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

                            </form>

                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}