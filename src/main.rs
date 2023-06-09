#![feature(impl_trait_in_assoc_type)]
#![feature(type_alias_impl_trait)]
#![feature(let_chains)]

use axum::{routing::get, Router};

mod env;
mod img;
mod url;

use crate::env::TO;

// Blackmagic URSA Mini Pro是一款革命性的数字电影摄影机，搭载 12288 x 6480 12K Super 35传感器
pub const MAX_WIDTH: u32 = 16380;
pub const MAX_HEIGHT: u32 = 16380;

#[tokio::main]
async fn main() {
  awp::init();

  unsafe {
    env::TO = std::env::var("TO").unwrap();
    tracing::info!("→ {TO}");
  }

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

  awp::srv(router, 9911).await;
}
