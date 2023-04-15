#![feature(impl_trait_in_assoc_type)]
#![feature(type_alias_impl_trait)]
#![feature(let_chains)]

use axum::{routing::get, Router};

mod env;
mod err;
mod img;
mod log;
mod srv;
mod url;

#[tokio::main]
async fn main() {
  log::init();
  let mut router = Router::new();
  macro_rules! get {
    (=> $func:ident) => {
      get!("/", $func)
    };
    ($url:stmt => $func:ident) => {
      use const_str::{concat, replace};
      get!(
        concat!('/', replace!(replace!(stringify!($url), " ", ""), "&", ":")),
        $func
      )
    };
    ($url:expr, $func:ident) => {
      router = router.route($url, get($crate::url::$func::get))
    };
  }

  get!( => stat);
  // get!("/*li" => img);

  router = router.route(r"/*li", get(crate::url::img::get));

  srv::srv(router).await;
}
