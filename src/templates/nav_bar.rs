use crate::Route;
use tailyew::molecules::{AppBar, AppBarPosition};
use yew::prelude::*;
use yew_router::prelude::Link;

#[function_component(NavBar)]
pub fn nav_bar() -> Html {
    html! {
      <AppBar
          title={Some("TailYew")}
          position={AppBarPosition::Top}
          logo_url={Some("/images/logo.png")}
          links={vec![
              html! { <Link<Route> to={Route::LandingPage}>{ "About" }</Link<Route>> },
          ]}
          actions={vec![]}
      />
    }
}
