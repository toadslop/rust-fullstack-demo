use crate::components::beer_card::BeerCard;
use crate::components::review_form::ReviewForm;
use crate::components::reviews_card::ReviewsCard;
use crate::{api::beer::get_beer, routes::Route};
use entity::beer::Relation;
use shared::ApiQueryParams;
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
    let beer_id = props
        .beer_id
        .parse::<i32>()
        .expect("the beer id to be parseable to an integer.");
    let has_error = use_state(|| false);
    let beer_handle = use_state(|| None);
    #[allow(clippy::redundant_closure)]
    let reviews_handle = use_state(|| Vec::new());
    let history = use_history().expect("history to be available");

    if *has_error {
        history.push(Route::NotFound)
    }

    {
        let beer_handle = beer_handle.clone();
        let reviews_handle = reviews_handle.clone();
        let handle_dep = reviews_handle.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let queries = ApiQueryParams {
                        expand: Some(Relation::Review.to_string()),
                    };

                    let result = get_beer(beer_id, Some(queries)).await;

                    match result {
                        Ok(result) => {
                            beer_handle.set(Some(result.0));
                            reviews_handle.set(result.1);
                        }
                        Err(_) => has_error.set(true),
                    }
                });
                || ()
            },
            handle_dep,
        );
    }

    html! {
        <div class="my-4 w-75 mx-auto">
          <div class="d-flex flex-column">
           <BeerCard class="mb-3" beer_handle={beer_handle} />
           <ReviewsCard class="mb-3" reviews_handle={reviews_handle.clone()} />
           <ReviewForm beer_id={beer_id} reviews_handle={reviews_handle} />
          </div>
        </div>
    }
}
