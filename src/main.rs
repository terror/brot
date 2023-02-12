use {
  brotli::{enc::writer::CompressorWriter, DecompressorWriter},
  clap::Parser,
  std::{
    ffi::OsStr,
    fs::File,
    io::{self, prelude::*, Write},
    path::PathBuf,
    process,
    time::Instant,
  },
  walkdir::WalkDir,
};

#[derive(Parser)]
struct Arguments {
  #[clap(long, default_value = "false")]
  decompress: bool,
  #[clap(short, long)]
  directory: PathBuf,
  #[clap(short, long, default_value = "11")]
  quality: u32,
  #[clap(short, long, default_value = "22")]
  window_size: u32,
}

static EXCLUDE: &'static [&'static str] = &["", "SHA1SUM"];

impl Arguments {
  fn run(self) -> Result {
    WalkDir::new(self.directory)
      .into_iter()
      .filter_map(|e| e.ok())
      .filter(|e| {
        e.path().is_file()
          && !EXCLUDE.contains(
            &e.path()
              .file_name()
              .unwrap_or(OsStr::new(""))
              .to_str()
              .unwrap(),
          )
      })
      .try_for_each(|entry| {
        let mut file = File::open(entry.path())?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let mut compressed = Vec::new();

        let now = Instant::now();

        {
          let mut encoder = CompressorWriter::new(
            &mut compressed,
            4096,
            self.quality,
            self.window_size,
          );

          encoder.write_all(&buffer)?;
        }

        let diff = now.elapsed().as_millis();

        if self.decompress {
          let mut stdout = io::stdout();
          let mut decoder = DecompressorWriter::new(&mut stdout, 4096);
          decoder.write_all(&compressed)?;
        }

        println!(
          "Compressed {} by {:.2}% in {}ms",
          entry.path().display(),
          (1.0 - (compressed.len() as f64 / buffer.len() as f64)) * 100.0,
          diff
        );

        Ok(())
      })
  }
}

type Result<T = (), E = anyhow::Error> = std::result::Result<T, E>;

fn main() {
  if let Err(error) = Arguments::parse().run() {
    eprintln!("error: {error}");
    process::exit(1);
  }
}
