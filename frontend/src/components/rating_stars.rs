use yew::{function_component, html, virtual_dom::VNode, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub rating: i32,
}

#[function_component(RatingStars)]
pub fn rating_stars(props: &Props) -> Html {
    let rating = props.rating;
    if rating > 5 || rating < 0 {
        return html!(<div>{"Invalid"}</div>);
    }

    html! {
        <div>
          {calc_star(1, rating)}
          {calc_star(2, rating)}
          {calc_star(3, rating)}
          {calc_star(4, rating)}
          {calc_star(5, rating)}
        </div>
    }
}

fn calc_star(star_num: i32, rating: i32) -> VNode {
    if rating >= star_num {
        html!(<i class="fa-solid fa-star"></i>)
    } else if rating < star_num && rating > star_num - 1 {
        html!(<i class="fa-solid fa-star"></i>)
    } else {
        html!(<i class="fa-regular fa-star"></i>)
    }
}
