use qwreey_utility_rs::ArcRwUserdata;
use rocket::{Build, Ignite, Orbit, Rocket, Route, State};

mod builder;
mod export;
mod responder;
mod tera_utility;

pub use rocket;
pub use rocket_dyn_templates;
pub use tera_utility::{add_builtin, TemplateToContent, ErrToTeraError};
pub use builder::WebBackendBuilder;
pub use export::{RouteExport, RouteExportList};
pub type UserdataState = State<ArcRwUserdata>;
pub type RocketIgnite = Rocket<Ignite>;
pub type RocketBuild = Rocket<Build>;
pub type RocketOrbit = Rocket<Orbit>;
pub type RouteList = Vec<Route>;
pub type TeraValue = rocket_dyn_templates::tera::Value;
pub type TeraError = rocket_dyn_templates::tera::Error;
pub use responder::{ElementResponder, ToElementResponder};
