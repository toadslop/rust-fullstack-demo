use yew::{
    function_component, html, use_effect_with_deps, use_state, virtual_dom::AttrValue, Properties,
};

use crate::api::beer::get_beer;

#[derive(Properties, PartialEq, Eq)]
pub struct Props {
    pub beer_id: AttrValue,
}

#[function_component(BeerDetail)]
pub fn beer_detail(props: &Props) -> Html {
    let beer_id = props.beer_id.parse::<i32>().unwrap();
    let beer = use_state(|| None);
    {
        let beer = beer.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    beer.set(get_beer(beer_id).await);
                });
                || ()
            },
            (),
        );
    }

    html! {
        <div class="mt-4 w-75 mx-auto">
          <div class="d-flex flex-column">
            <h1>{ &props.beer_id }</h1>

          </div>
        </div>
    }
}
