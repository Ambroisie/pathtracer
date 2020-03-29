use pathtracer::scene::Scene;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Options {
    /// Input description for the scene to be rendered.
    #[structopt(short, long, parse(from_os_str), default_value = "scene.yaml")]
    input: PathBuf,
    /// Output image for the rendered scene.
    #[structopt(short, long, parse(from_os_str), default_value = "scene.png")]
    output: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = Options::from_args();
    let f = std::fs::File::open(options.input)?;

    let scene: Scene = serde_yaml::from_reader(f)?;
    let image = scene.render();

    image.save(options.output)?;
    Ok(())
}
