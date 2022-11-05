use crate::api::beer::get_beers;
use yew::{function_component, html, use_effect_with_deps, use_state};

#[function_component(BeerList)]
pub fn beer_list() -> Html {
    let beers = use_state(|| String::new());

    {
        let beers = beers.clone();
        use_effect_with_deps(
            move |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    beers.set(get_beers().await);
                });
                || ()
            },
            (),
        );
    }

    html! {
        <>
          <h1>{ "Hello World" }</h1>
          <div>{&*beers.clone()}</div>
        </>
    }
}
