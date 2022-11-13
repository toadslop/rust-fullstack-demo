use entity::review::Model as Review;
use yew::{function_component, html, Html, Properties, UseStateHandle};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub reviews_handle: UseStateHandle<Vec<Review>>,
}

#[function_component(ReviewsCard)]
pub fn reviews_card(props: &Props) -> Html {
    let reviews = &*props.reviews_handle;

    html! {
    <div class="card">
      {
        reviews.iter().map(|review| {

        html!{
            <div >
                <div>{&review.review_text}</div>
            </div>
        }
    }).collect::<Html>()}
    </div>}
}
