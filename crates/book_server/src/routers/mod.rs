pub mod route;
pub mod cs;
pub mod bs;
pub mod login;
pub mod public;

pub use cs::cs_routes;
pub use bs::bs_routes;
pub use public::PublicRouter;
