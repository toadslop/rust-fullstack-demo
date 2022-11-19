use entity::beer::Model as Beer;
use yew::{classes, function_component, html, Properties, UseStateHandle};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub beer_handle: UseStateHandle<Option<Beer>>,
    pub class: &'static str,
}

#[function_component(BeerCard)]
pub fn beer_card(props: &Props) -> Html {
    let beer = &*props.beer_handle;
    let class = props.class;

    if let Some(beer) = &beer {
        html! {
          <div class={classes!("card", "shadow-sm", class)}  >
             <div class="row g-0">
                 <div class="col">
                     <div class="card-body">
                         <h6>{&beer.brewery}</h6>
                         <div class="d-flex align-items-baseline">
                            <h5 class="card-title">{&beer.name}</h5>
                            {" "}
                            <h6>{{format!("({:.2})", &beer.average_rating)}}</h6>
                         </div>
                         <p class="card-text">{&beer.description}</p>
                     </div>
                 </div>
                 <div class="col-md-auto">
                     <img
                     src={beer.image_url.to_owned()}
                     class="card-img-right beer-img m-3"
                     alt={format!("Photo of {}", &beer.name)} />
                 </div>
             </div>
          </div>
        }
    } else {
        html! {"Loading..."}
    }
}
