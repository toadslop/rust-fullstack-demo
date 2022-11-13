use crate::{api::review::add_review, components::rating_stars::RatingStars};
use entity::review;
use wasm_bindgen::JsCast;
use web_sys::{Element, EventTarget, HtmlInputElement, HtmlTextAreaElement};
use yew::{function_component, html, use_state, Callback, Properties, UseStateHandle};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub beer_id: i32,
    pub reviews_handle: UseStateHandle<Vec<review::Model>>,
}

#[function_component(ReviewForm)]
pub fn review_form(props: &Props) -> Html {
    let beer_id = props.beer_id;
    let new_rating_handle = use_state(|| review::Model {
        beer_id,
        rating: 0,
        ..Default::default()
    });

    let on_click_star = {
        let new_rating_handle = new_rating_handle.clone();
        let new_rating = &*new_rating_handle;
        let mut new_rating = new_rating.to_owned();
        Callback::once(move |e: yew::MouseEvent| {
            let target = e.target().expect("event target to be present");
            let element = target
                .dyn_ref::<Element>()
                .expect("the clicked start to be an Element");
            let value = element
                .get_attribute("data-rating")
                .expect("there to be a data rating on the Element")
                .parse::<i32>();

            new_rating.rating = value.expect("the rating to be parsable to i32");
            new_rating_handle.set(new_rating);
        })
    };

    let handle_name_change = {
        let new_rating_handle = new_rating_handle.clone();
        let new_rating = &*new_rating_handle;
        let mut new_rating = new_rating.to_owned();
        Callback::once(move |e: yew::Event| {
            let target: Option<EventTarget> = e.target();
            let input = target
                .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
                .expect("the target to be an input element");
            let reviewer_name = input.value();
            new_rating.reviewer_name = reviewer_name;
            new_rating_handle.set(new_rating);
        })
    };

    let handle_comment_change = {
        let new_rating_handle = new_rating_handle.clone();
        let new_rating = &*new_rating_handle;
        let mut new_rating = new_rating.to_owned();
        Callback::once(move |e: yew::Event| {
            let target: Option<EventTarget> = e.target();
            let input = target
                .and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok())
                .expect("the target to be a text area element");
            let comment = input.value();
            new_rating.review_text = comment;
            new_rating_handle.set(new_rating);
        })
    };

    let handle_submit = {
        let new_rating_handle = new_rating_handle.clone();
        let new_rating = &*new_rating_handle;
        let new_rating = new_rating.to_owned();
        let reviews_handle = props.reviews_handle.clone();
        Callback::once(move |e: yew::MouseEvent| {
            e.prevent_default();
            wasm_bindgen_futures::spawn_local(async move {
                let result = add_review(beer_id, &new_rating).await;

                match result {
                    Ok(resp) => {
                        let reviews = &*reviews_handle;
                        let mut reviews = reviews.clone();
                        reviews.push(resp);
                        reviews_handle.set(reviews);
                    }

                    Err(err) => {
                        gloo_console::error!(err.to_string());
                    }
                };
            });
        })
    };

    html! {
        <div class="card">
          <div class="card-body">
            <h5>{"Add new review"}</h5>
            <form>
              <div class="mb-3">
                <label
                  for="reviewer_name"
                  class="form-label">{"Your name"}</label>
                <input
                  onchange={handle_name_change}
                  type="text"
                  class="form-control"
                  id="reviewer_name"
                  aria-describedby="review_name_help"
                  value={new_rating_handle.reviewer_name.to_owned()}
                   />
                <div id="review_name_help" class="form-text">{"Enter your name."}</div>
              </div>
              <div class="mb-3">
                <label for="rating" class="form-label">{"Rating"}</label>
                <input
                  type="hidden"
                  class="form-control"
                  id="rating" value={new_rating_handle.rating.to_string()} />
                <RatingStars
                  rating={new_rating_handle.rating}
                  editable={true}
                  on_click_star={Some(on_click_star)} />
              </div>
              <div class="mb-3">
                <label class="form-check-label" for="review_text">{"Comment"}</label>
                <textarea onchange={handle_comment_change} type="text" class="form-control" id="review_text" rows={"3"} />
              </div>
              <button
                onclick={handle_submit}
                type="submit"
                class="btn btn-primary">{"Submit"}</button>
            </form>
          </div>
        </div>
    }
}
