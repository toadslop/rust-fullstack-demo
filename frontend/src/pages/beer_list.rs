use crate::{api::beer::get_beers, routes::Route};
use yew::{function_component, html, use_effect_with_deps, use_state, Callback, Html, MouseEvent};
use yew_router::history::History;
use yew_router::prelude::use_history;

#[function_component(BeerList)]
pub fn beer_list() -> Html {
    #[allow(clippy::redundant_closure)]
    let beers = use_state(|| Vec::new());
    let history = use_history().expect("history to be available");
    let row_click = |id: i32| -> Callback<MouseEvent> {
        let history = history.clone();
        Callback::once(move |_: yew::MouseEvent| {
            history.push(Route::BeerDetail {
                beer_id: id.to_string(),
            })
        })
    };

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
                    <col width="51%" />
                    <col width="16%" />
                    <col width="16%" />
                    <col width="16%" />
                </colgroup>
                <thead>
                    <th scope="col">{"Name"}</th>
                    <th scope="col">{"Alcohol Content"}</th>
                    <th scope="col">{"Rating"}</th>
                    <th scope="col">{"Style"}</th>
                </thead>
                <tbody>
                {
                    beers.clone().iter().map(|beer| {

                    html!{
                        <tr class="pointer" onclick={row_click(beer.id)}>
                            <th scope="row">{&beer.name}</th>
                            <td >{&beer.alcohol_content}</td>
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
