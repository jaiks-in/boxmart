use std::thread::spawn;
use leptos::*;
use leptos::prelude::*; 
use std::time::Duration;
use leptos::prelude::signal;


pub fn HomePage() ->impl IntoView{
        let (current_index,set_current_index)=signal(0);
        let (show_modal,set_show_modal)=signal(false);
        let (selected_type,set_selected_type)=signal("".to_string());
        let close_modal=move |_|set_show_modal.set(false);
    let go_to_products=|name:String|{

    };
    let go_to_contact=|_|{

    };
   
    view! {
        <div>
            // Hero Section
            <section class="bg-dark text-white text-center p-5">
                <div class="container">
                    <h1 class="display-4">"Premium Packaging Solutions"</h1>
                    <p class="lead">"Your Boxes, Your Brand"</p>
                    <button class="btn btn-primary me-2"
                        on:click=move |_| go_to_products("Shop Now".to_string())>
                        "Shop Now"
                    </button>
                    <button class="btn btn-outline-light" on:click=go_to_contact>
                        "Get a Quote"
                    </button>
                </div>
            </section>

            // Featured Products Section
            <section class="py-5">
                <div class="container">
                    <h2 class="mb-4 text-center">"Featured Products"</h2>
                    <div class="row row-cols-1 row-cols-md-3 g-4">

                        // Corrugated
                        <div class="col">
                            <div class="card h-100 text-center">
                                <img src="https://res.cloudinary.com/dtc2dfweh/image/upload/v1757511629/Gemini_Generated_Image_e7l9due7l9due7l9_erbott.png"
                                    class="card-img-top" alt="Corrugated Box"/>
                                <div class="card-body">
                                    <h5 class="card-title">"Corrugated Box"</h5>
                                    <p class="card-text">"Strong, durable boxes for shipping."</p>
                                </div>
                                <div class="card-footer">
                                    <button class="btn btn-primary w-100"
                                        on:click=move |_| go_to_products("Corrugated Box".to_string())>
                                        "View Details"
                                    </button>
                                </div>
                            </div>
                        </div>

                        // Duplex
                        <div class="col">
                            <div class="card h-100 text-center">
                                <img src="https://res.cloudinary.com/dtc2dfweh/image/upload/v1756829688/Gemini_Generated_Image_6511cs6511cs6511_dqedee.png"
                                    class="card-img-top" alt="Duplex Box"/>
                                <div class="card-body">
                                    <h5 class="card-title">"Duplex Box"</h5>
                                    <p class="card-text">"Premium printed boxes for retail."</p>
                                </div>
                                <div class="card-footer">
                                    <button class="btn btn-primary w-100"
                                        on:click=move |_| go_to_products("Duplex Box".to_string())>
                                        "View Details"
                                    </button>
                                </div>
                            </div>
                        </div>

                        // Visiting Card
                        <div class="col">
                            <div class="card h-100 text-center">
                                <img src="https://res.cloudinary.com/dtc2dfweh/image/upload/v1756829689/Gemini_Generated_Image_cbr682cbr682cbr6_yntnsz.png"
                                    class="card-img-top" alt="Visiting Card"/>
                                <div class="card-body">
                                    <h5 class="card-title">"Visiting Card"</h5>
                                    <p class="card-text">"Reusable and sturdy plastic packaging."</p>
                                </div>
                                <div class="card-footer">
                                    <button class="btn btn-primary w-100"
                                        on:click=move |_| go_to_products("Visiting Card".to_string())>
                                        "View Details"
                                    </button>
                                </div>
                            </div>
                        </div>
                          <div class="col">
                            <div class="card h-100 text-center">
                                <img src="https://res.cloudinary.com/dtc2dfweh/image/upload/v1756829689/Gemini_Generated_Image_cbr682cbr682cbr6_yntnsz.png"
                                    class="card-img-top" alt="Tags"/>
                                <div class="card-body">
                                    <h5 class="card-title">"Tags"</h5>
                                    <p class="card-text">"Reusable and sturdy packaging."</p>
                                </div>
                                <div class="card-footer">
                                    <button class="btn btn-primary w-100"
                                        on:click=move |_| go_to_products("Tags".to_string())>
                                        "View Details"
                                    </button>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            // About Section
            <section class="bg-light py-5">
                <div class="container text-center">
                    <h2>"About Boxmart"</h2>
                    <p class="lead mt-3">
                        "Ananya Industries specializes in high-quality packaging solutions for businesses of all sizes."
                    </p>
                </div>
            </section>

            // Features Section
            <section class="py-5">
                <div class="container">
                    <h2 class="mb-4 text-center">"Why Choose Us?"</h2>
                    <div class="row text-center g-4">
                        <div class="col-md-3">
                            <div class="card border-0">
                                <div class="card-body">
                                    <i class="bi bi-award fs-1 text-primary"></i>
                                    <h5 class="card-title mt-3">"Quality"</h5>
                                    <p class="card-text">"Premium materials for every box."</p>
                                </div>
                            </div>
                        </div>
                        <div class="col-md-3">
                            <div class="card border-0">
                                <div class="card-body">
                                    <i class="bi bi-pencil-square fs-1 text-primary"></i>
                                    <h5 class="card-title mt-3">"Customization"</h5>
                                    <p class="card-text">"Custom sizes & prints."</p>
                                </div>
                            </div>
                        </div>
                        <div class="col-md-3">
                            <div class="card border-0">
                                <div class="card-body">
                                    <i class="bi bi-truck fs-1 text-primary"></i>
                                    <h5 class="card-title mt-3">"Fast Delivery"</h5>
                                    <p class="card-text">"On-time shipments every time."</p>
                                </div>
                            </div>
                        </div>
                        <div class="col-md-3">
                            <div class="card border-0">
                                <div class="card-body">
                                    <i class="bi bi-currency-dollar fs-1 text-primary"></i>
                                    <h5 class="card-title mt-3">"Affordable"</h5>
                                    <p class="card-text">"Competitive pricing for all clients."</p>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            // Contact CTA Section
            <section class="bg-primary text-white text-center p-5">
                <div class="container">
                    <h2>"Ready to get started?"</h2>
                    <button class="btn btn-light mt-3" on:click=go_to_contact>
                        "Contact Us"
                    </button>
                </div>
            </section>

            // Modal (conditionally render)
            {move || show_modal.get().then(|| view! {
                <div class="modal fade show d-block" tabindex="-1" style="background: rgba(0,0,0,0.5);">
                    <div class="modal-dialog modal-lg">
                        <div class="modal-content p-4">
                            <div class="modal-header">
                                <h5 class="modal-title">{selected_type.get()}</h5>
                                <button class="btn-close" on:click=close_modal></button>
                            </div>
                            <div class="modal-body">
                                <p>"Cost estimator form for " {selected_type.get()}</p>
                                
                            </div>
                        </div>
                    </div>
                </div>
            })}
        </div>
    }
}