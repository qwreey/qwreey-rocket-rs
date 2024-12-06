use std::{net::IpAddr, sync::Arc};

use qwreey_utility_rs::{ArcRwUserdata, ErrToString};
use rocket::{config::Ident, fairing::AdHoc, Config};
use rocket_dyn_templates::Template;

use crate::{tera_utility::add_builtin, RocketOrbit, RouteExport, RouteExportList};

// Web Backend Builder
pub struct WebBackendBuilder {
    exports: RouteExportList,
    port: u16,
    address: IpAddr,
    userdata: Option<ArcRwUserdata>,
}
impl Default for WebBackendBuilder {
    fn default() -> Self {
        Self {
            exports: Vec::new(),
            port: 80,
            address: "127.0.0.1".parse::<IpAddr>().unwrap(),
            userdata: None,
        }
    }
}
impl WebBackendBuilder {
    pub async fn build(self) -> Result<(), String> {
        let builder = Arc::new(self);
        let mut rocket = rocket::build().configure(Config {
            cli_colors: false,
            ident: Ident::try_new("MC-Captcha").unwrap(),
            port: builder.port,
            address: builder.address,
            ..Default::default()
        });

        // Append userdata
        let userdata = builder.userdata.clone().unwrap();
        rocket = rocket.manage(userdata.clone());

        // Mount routes
        for export in &builder.exports {
            rocket = rocket.mount(export.base(), export.routes());
        }

        // Add template handle
        let builder_template_clone = builder.clone();
        let userdata_template_clone = userdata.clone();
        rocket = rocket.attach(Template::custom(move |engines| {
            add_builtin(&mut engines.tera);
            for export in &builder_template_clone.exports {
                export.tera(&mut engines.tera, userdata_template_clone.clone());
            }
            engines.tera.autoescape_on(vec![]);
        }));

        // Add orbit handle
        let builder_liftoff_clone = builder.clone();
        let userdata_liftoff_clone = userdata.clone();
        rocket = rocket.attach(AdHoc::on_liftoff("orbit", move |orbit: &RocketOrbit| {
            Box::pin(async move {
                for export in &builder_liftoff_clone.exports {
                    export.orbit(orbit, userdata_liftoff_clone.clone()).unwrap();
                }
            })
        }));

        // Emit build
        for export in &builder.exports {
            rocket = export.build(rocket, userdata.clone())?;
        }

        // Ignite
        let mut rocket = rocket.ignite().await.err_tostring()?;
        for export in &builder.exports {
            rocket = export.ignite(rocket, userdata.clone())?;
        }

        // Launch & Await
        rocket.launch().await.err_tostring()?;
        Ok(())
    }
    pub fn new() -> Self {
        Self::default()
    }
    pub fn port(mut self, port: Option<u16>) -> Self {
        self.port = port.unwrap_or(Self::default().port);
        self
    }
    pub fn bind(mut self, address: Option<IpAddr>) -> Self {
        self.address = address.unwrap_or(Self::default().address);
        self
    }
    pub fn userdata(mut self, userdata: ArcRwUserdata) -> Self {
        self.userdata = Some(userdata);
        self
    }
    pub fn add_export(mut self, export: impl RouteExport + 'static) -> Self {
        self.exports.push(Box::new(export));
        self
    }
    pub fn add_export_many(mut self, exports: RouteExportList) -> Self {
        for export in exports {
            self.exports.push(export);
        }
        self
    }
}

#[macro_export]
macro_rules! export_list {
    [$($item:expr,)*] => {
        vec![$(Box::new($item),)*]
    };
    [$($item:expr),*] => {
        vec![$(Box::new($item),)*]
    };
}
