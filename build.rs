use actix_web_static_files::NpmBuild;
use std::env;

const SERVE_DIR: &str = "web/dist/media-server-one";

fn main() {
    let run_npm = match env::var("DONT_RUN_NPM") {
        Ok(var) => {
            if var == "1" {
                false
            } else {
                true
            }
        }
        Err(_) => true,
    };

    if run_npm {
        NpmBuild::new("web")
            .install()
            .unwrap()
            .run("build")
            .unwrap()
            .target(SERVE_DIR)
            .to_resource_dir()
            .build()
            .unwrap();
    }
}
