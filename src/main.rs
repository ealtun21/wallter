use std::panic::set_hook;

use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
enum Themes {
    Gruvbox,
    Nord,
    Solarized,
    Catppuccin,
    Dracula,
    Custom(Vec<(u8, u8, u8)>),
}

impl Themes {
    fn to_theme(self) -> Theme {
        match self {
            Themes::Gruvbox => Theme::gruvbox(),
            Themes::Nord => Theme::nord(),
            Themes::Solarized => Theme::solarized(),
            Themes::Catppuccin => Theme::catppuccin(),
            Themes::Dracula => Theme::dracula(),
            Themes::Custom(rbgs) => Theme::new(rbgs),
        }
    }
}

use std::str::FromStr;

impl FromStr for Themes {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "gruvbox" => Ok(Themes::Gruvbox),
            "nord" => Ok(Themes::Nord),
            "solarized" => Ok(Themes::Solarized),
            "catppuccin" => Ok(Themes::Catppuccin),
            "dracula" => Ok(Themes::Dracula),
            _ => {
                let mut colors: Vec<(u8, u8, u8)> = Vec::new();
                for hex_string in s.split(|c: char| c == ',' || c.is_whitespace()) {
                    let hex_string = hex_string.trim_start_matches('#').trim_start();
                    if !hex_string.is_empty() {
                        let bytes = hex::decode(hex_string.as_bytes())
                            .map_err(|e| format!("Failed to parse hex color code: {}", e))?;
                        if bytes.len() == 3 {
                            colors.push((bytes[0], bytes[1], bytes[2]));
                        } else {
                            return Err("RGB color tuple must have three components".to_owned());
                        }
                    }
                }
                Ok(Themes::Custom(colors))
            }
        }
    }
}

use clap::Parser;
use wallter::{apply_theme, Theme};

/// WALLter: the Simple Wallpaper Converter
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Filename to convert
    #[arg(short, long)]
    input: String,

    /// Theme [Gruvbox, Nord, Solarized, Catppuccin, Dracula, Custom(e.g. "#HexColor, #HexColor". NOTE: '"' is needed.)]
    #[arg(short, long)]
    theme: Themes,

    /// Output filename
    #[arg(short, long)]
    output: Option<String>,
}

fn get_output_path(args: &Args) -> PathBuf {
    let input_path = Path::new(&args.input);

    let mut output_path = match &args.output {
        Some(filename) => PathBuf::from(filename),
        None => {
            let stem = input_path.file_stem().unwrap().to_string_lossy();
            let output_filename = String::from(format!("{stem}-themed"));
            PathBuf::from(output_filename)
                .with_extension(input_path.extension().unwrap_or_default())
        }
    };

    // If the output path is a relative path, make it relative to the input file's directory
    if output_path.is_relative() {
        let input_dir = input_path.parent().unwrap_or_else(|| Path::new("."));
        output_path = input_dir.join(&output_path);
    }

    output_path
}

fn main() {
    set_hook(Box::new(|info| {
        if let Some(s) = info.payload().downcast_ref::<String>() {
            println!("{}", s);
        }
    }));

    let args = Args::parse();

    // Load input image and apply Dracula theme
    let input_image = image::io::Reader::open(args.input.clone())
        .expect("Unable to read file")
        .with_guessed_format()
        .expect("Unable to guess image format")
        .decode()
        .expect("Failed to load image");

    let themed_image = apply_theme(&input_image, &args.theme.clone().to_theme());

    let output_path = get_output_path(&args);
    // Save themed image to output file path
    themed_image
        .save(output_path)
        .expect("Incorrect output path/filename/extension");
}
