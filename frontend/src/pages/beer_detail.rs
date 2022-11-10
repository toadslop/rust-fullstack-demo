use crate::{api::beer::get_beer, routes::Route};
use yew::{
    function_component, html, use_effect_with_deps, use_state, virtual_dom::AttrValue, Properties,
};
use yew_router::history::History;
use yew_router::prelude::use_history;

#[derive(Properties, PartialEq, Eq)]
pub struct Props {
    pub beer_id: AttrValue,
}

#[function_component(BeerDetail)]
pub fn beer_detail(props: &Props) -> Html {
    let beer_id = props.beer_id.parse::<i32>().unwrap();
    let has_error = use_state(|| false);
    let beer = use_state(|| None);
    let history = use_history().expect("history to be available");

    if *has_error {
        history.push(Route::NotFound)
    }

    {
        let beer = beer.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let result = get_beer(beer_id).await;
                    match result {
                        Ok(result) => beer.set(Some(result)),
                        Err(_) => has_error.set(true),
                    }
                });
                || ()
            },
            (),
        );
    }

    let content = if let Some(beer) = &*beer {
        html! {
           <div class="card" >
                <div class="row g-0">
                    <div class="col">
                        <div class="card-body">
                            <h6>{&beer.brewery}</h6>
                            <h5 class="card-title">{&beer.name}</h5>
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
        html! { <div>{"Loading..."}</div> }
    };

    html! {
        <div class="mt-4 w-75 mx-auto">
          <div class="d-flex flex-column">
           {content}
          </div>
        </div>
    }
}
