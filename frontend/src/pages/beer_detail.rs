use yew::{function_component, html, virtual_dom::AttrValue, Properties};

#[derive(Properties, PartialEq, Eq)]
pub struct Props {
    pub beer_id: AttrValue,
}

#[function_component(BeerDetail)]
pub fn beer_detail(props: &Props) -> Html {
    html! {
        <div class="mt-4 w-75 mx-auto">
          <div class="d-flex flex-column">
            <h1>{ &props.beer_id }</h1>

          </div>
        </div>
    }
}
