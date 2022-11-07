use crate::api::beer::get_beers;
use yew::{function_component, html, use_effect_with_deps, use_state, Html};

#[function_component(BeerList)]
pub fn beer_list() -> Html {
    #[allow(clippy::redundant_closure)]
    let beers = use_state(|| Vec::new());

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
        <div class="mt-4 w-75 mx-auto">
          <div class="d-flex flex-column">
            <h1>{ "Beer List" }</h1>
            <table class="table">
                <colgroup>
                    <col width="50%" />
                    <col width="25%" />
                    <col width="25%" />
                </colgroup>
                <thead>
                    <th scope="col">{"Name"}</th>
                    <th scope="col">{"Rating"}</th>
                    <th scope="col">{"Style"}</th>
                </thead>
                <tbody>
                {beers.clone().iter().map(|beer| {
                    html!{
                        <tr>
                            <th scope="row">{&beer.name}</th>
                            <td >{&beer.average_rating}</td>
                            <td >{&beer.style}</td>
                        </tr>
                    }
                }).collect::<Html>()}
                </tbody>
            </table>
          </div>
        </div>
    }
}
