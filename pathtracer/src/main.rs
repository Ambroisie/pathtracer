use pathtracer::render::{Pathtracer, Raytracer};
use pathtracer::scene::Scene;
use std::path::PathBuf;
use std::str;
use structopt::clap::arg_enum;
use structopt::StructOpt;

arg_enum! {
    #[derive(Debug)]
    enum RenderOption {
        Raytracer,
        Pathtracer,
    }
}

#[derive(StructOpt, Debug)]
struct Options {
    /// Input description for the scene to be rendered.
    #[structopt(short, long, parse(from_os_str), default_value = "scene.yaml")]
    input: PathBuf,
    /// Output image for the rendered scene.
    #[structopt(short, long, parse(from_os_str), default_value = "scene.png")]
    output: PathBuf,
    /// Which renderer should be used on the input scene.
    #[structopt(
        short,
        long,
        possible_values = &RenderOption::variants(),
        case_insensitive = true,
        default_value = "Raytracer"
    )]
    renderer: RenderOption,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = Options::from_args();
    let f = std::fs::File::open(options.input)?;

    let scene: Scene = serde_yaml::from_reader(f)?;
    let image = match options.renderer {
        RenderOption::Raytracer => Raytracer::new(scene).render(),
        RenderOption::Pathtracer => Pathtracer::new(scene).render(),
    };

    image.save(options.output)?;
    Ok(())
}
