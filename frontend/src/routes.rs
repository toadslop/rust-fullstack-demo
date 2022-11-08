use crate::pages::{beer_detail::BeerDetail, beer_list::BeerList};
use yew::{html, virtual_dom::AttrValue, Html};
use yew_router::Routable;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Beers,
    #[at("/beers/:beer_id")]
    BeerDetail { beer_id: String },
}

fn switch(route: &Route) -> Html {
    match route {
        Route::Beers => html! { <BeerList /> },
        Route::BeerDetail { beer_id } => {
            html! {<BeerDetail beer_id={AttrValue::Owned(beer_id.to_owned())} />}
        }
    }
}
