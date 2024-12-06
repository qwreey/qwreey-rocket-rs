use qwreey_utility_rs::ArcRwUserdata;
use rocket_dyn_templates::tera::Tera;

use crate::{RocketBuild, RocketIgnite, RocketOrbit, RouteList};

pub trait RouteExport: 'static + Sync + Send {
    fn routes(&self) -> RouteList {
        vec![]
    }
    #[allow(unused)]
    fn build(&self, rocket: RocketBuild, userdata: ArcRwUserdata) -> Result<RocketBuild, String> {
        Ok(rocket)
    }
    #[allow(unused)]
    fn ignite(&self, rocket: RocketIgnite, userdata: ArcRwUserdata) -> Result<RocketIgnite,String> {
        Ok(rocket)
    }
    fn base(&self) -> &'static str {
        "/"
    }
    #[allow(unused)]
    fn orbit(&self, rocket: &RocketOrbit, userdata: ArcRwUserdata) -> Result<(),String> {
        Ok(())
    }
    #[allow(unused)]
    fn tera(&self, tera: &mut Tera, userdata: ArcRwUserdata) {}
}
pub type RouteExportList = Vec<Box<dyn RouteExport>>;
