use clap::App;

pub struct CommandLineArguments {
    pub filename: String,
    pub output_width: usize,
    pub output_height: usize,
    pub antialias_factor: usize,
    pub render_width: usize,
    pub render_height: usize,
}

impl CommandLineArguments {
    pub fn read() -> Self {
        let yaml = load_yaml!("cli.yml");
        let matches = App::from_yaml(yaml).get_matches();
        let filename = matches.value_of("output").unwrap_or("out.png");
        let antialias_factor = matches.value_of("antialias")
            .and_then(|a| a.parse::<usize>().ok())
            .unwrap_or(1);
        let output_width = matches.value_of("width")
            .and_then(|w| w.parse::<usize>().ok())
            .unwrap_or(800);
        let output_height = matches.value_of("height")
            .and_then(|h| h.parse::<usize>().ok())
            .unwrap_or(output_width);

        Self {
            filename: String::from(filename),
            output_width,
            output_height,
            antialias_factor,
            render_width: output_width * antialias_factor,
            render_height: output_height * antialias_factor,
        }
    }
}