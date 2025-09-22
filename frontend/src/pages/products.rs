use leptos::*;
use leptos::prelude::*;
use crate::components::cost_estimator::CostEstimator;
use crate::components::product_card::ProductCard;
pub fn products()->impl IntoView{
    view!{
       <div class="container-fluid">
            <div class="row">
                // Left: 75% Products Listing
                <div class="col-md-9 col-sm-12 mb-3">
                    <div class="row row-cols-1 row-cols-md-3 g-4">
                        // Example Product Cards
                        <ProductCard />
                        <ProductCard />
                        <ProductCard />
                    </div>
                </div>

                // Right: 25% Cost Estimator
                <div class="col-md-3 col-sm-12 mb-3">
                    <CostEstimator />
                </div>
            </div>
        </div>
         
    }
}