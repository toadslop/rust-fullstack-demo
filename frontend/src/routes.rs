use crate::pages::{beer_detail::BeerDetail, beer_list::BeerList};
use yew::{html, virtual_dom::AttrValue, Html};
use yew_router::Routable;

#[derive(Clone, Routable, PartialEq, Eq)]
pub enum Route {
    #[at("/")]
    Beers,
    #[at("/beers/:beer_id")]
    BeerDetail { beer_id: String },
    #[not_found]
    #[at("/settings/404")]
    NotFound,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        #[allow(clippy::let_unit_value)]
        Route::Beers => html! { <BeerList /> },
        Route::BeerDetail { beer_id } => {
            html! {<BeerDetail beer_id={AttrValue::Owned(beer_id.to_owned())} />}
        }
        Route::NotFound => html! { <h2>{"404! Not found!"}</h2>},
    }
}
