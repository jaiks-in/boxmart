use leptos::*;
use leptos::prelude::*;
use leptos_router::*;

#[component]
pub fn ProductCard()->impl IntoView{
        view!{
            <div class="col">
  <div class="card h-100 shadow-sm">
    <img src="https://via.placeholder.com/300x200" class="card-img-top" alt="Product Image"/>
    <div class="card-body">
      <h5 class="card-title">"Product Name"</h5>
      <p class="card-text">"Short description of the product goes here. Example: Corrugated Box, 40x23 cm."</p>
      <p class="card-text"><strong>"Price:0"</strong></p>
      <button class="btn btn-primary w-100">"View Details"</button>
    </div>
  </div>
</div>

        }
}