use yew::{classes, function_component, html, Callback, MouseEvent, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub rating: i32,
    #[prop_or_default]
    pub editable: bool,
    #[prop_or_default]
    pub on_click_star: Option<Callback<MouseEvent>>,
}

#[function_component(RatingStars)]
pub fn rating_stars(props: &Props) -> Html {
    let rating = props.rating;
    let editable = props.editable;
    let on_click_star = &props.on_click_star;

    let class = if editable { Some("pointer") } else { None };

    if !(0..=5).contains(&rating) {
        return html!(<div>{"Invalid"}</div>);
    }

    html! {
        <div class={classes!(class)}>
            <i data-rating={1} onclick={on_click_star.clone()} class={calc_star(1, rating)}></i>
            <i data-rating={2} onclick={on_click_star.clone()} class={calc_star(2, rating)}></i>
            <i data-rating={3} onclick={on_click_star.clone()} class={calc_star(3, rating)}></i>
            <i data-rating={4} onclick={on_click_star.clone()} class={calc_star(4, rating)}></i>
            <i data-rating={5} onclick={on_click_star.clone()} class={calc_star(5, rating)}></i>
        </div>
    }
}

fn calc_star(star_num: i32, rating: i32) -> String {
    if rating >= star_num {
        String::from("fa-solid fa-star")
    } else if rating < star_num && rating > star_num - 1 {
        String::from("fa-regular fa-star-half-stroke")
    } else {
        String::from("fa-regular fa-star")
    }
}
