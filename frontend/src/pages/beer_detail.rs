use crate::api::beer::get_beers;
use entity::beer::Model as Beer;
use yew::{function_component, html, use_effect_with_deps, use_state, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub beer: Beer,
}

#[function_component(BeerList)]
pub fn beer_list(props: &Props) -> Html {
    #[allow(clippy::redundant_closure)]

    html! {
        <div class="mt-4 w-75 mx-auto">
          <div class="d-flex flex-column">
            <h1>{ &props.beer.name }</h1>

          </div>
        </div>
    }
}
