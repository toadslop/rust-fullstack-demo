use crate::components::rating_stars::RatingStars;
use entity::review;
use yew::{classes, function_component, html, Html, Properties, UseStateHandle};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub reviews_handle: UseStateHandle<Vec<review::Model>>,
    pub class: &'static str,
}

#[function_component(ReviewsCard)]
pub fn reviews_card(props: &Props) -> Html {
    let reviews = &*props.reviews_handle;
    let class = props.class;

    html! {
        <div class={classes!("card", "shadow-sm", class)}>
            <div class="card-body">
                <h5>{"Reviews"}</h5>
            </div>
            {
                reviews.iter().map(|review| {
                    html!{
                        <div class="border-top card-body" >
                            <div class="d-flex justify-content-between">
                                <h6 class="fw-bold">{&review.reviewer_name}</h6>
                                <RatingStars rating={review.rating} />
                            </div>
                            <div>{&review.review_text}</div>
                            <div class="text-muted fst-italic">
                                {&review.date.format("%Y-%m-%d %H:%M:%S").to_string()}
                            </div>
                        </div>
                    }
                }).collect::<Html>()
            }

        </div>
    }
}
