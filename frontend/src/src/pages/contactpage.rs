use leptos::*;
use leptos::prelude::{ElementChild,ClassAttribute,OnAttribute};

#[component]
pub fn ContactPage() ->impl IntoView{

    view!{
         <div class="container mt-5">
            <h2 class="text-center mb-4">"Contact Us"</h2>

            <div class="row justify-content-center">
                <div class="col-md-6">
                    <form class="p-4 border rounded bg-light shadow-sm">
                        <div class="mb-3">
                            <label for="name" class="form-label">"Name"</label>
                            <input type="text" class="form-control" id="name" placeholder="Enter your name"/>
                        </div>

                        <div class="mb-3">
                            <label for="email" class="form-label">"Email"</label>
                            <input type="email" class="form-control" id="email" placeholder="Enter your email"/>
                        </div>

                        <div class="mb-3">
                            <label for="message" class="form-label">"Message"</label>
                            <textarea class="form-control" id="message" rows="4" placeholder="Write your message"></textarea>
                        </div>

                        <button type="submit" class="btn btn-primary w-100">"Send Message"</button>
                    </form>
                </div>
            </div>
        </div>
    }
}