use std::boxed::Box;
use std::env;
use std::ffi::OsStr;
use std::io::{Error, ErrorKind};
use std::path::Path;

pub struct TSC {
    verbose: bool,
}

#[derive(Copy, Clone, Debug)]
enum TSCEncodingMode {
    Encoding,
    Decoding,
}

impl TSC {
    pub fn new(verbose: bool) -> Self {
        Self { verbose: verbose }
    }

    fn process(&self, mode: TSCEncodingMode, input: Box<Path>, output_dir: Box<Path>) -> std::io::Result<usize> {
        let mut num = 0;
        let file_ext = match mode {
            TSCEncodingMode::Encoding => OsStr::new("txt"),
            TSCEncodingMode::Decoding => OsStr::new("tsc"),
        };
        let out_file_ext = match mode {
            TSCEncodingMode::Encoding => OsStr::new("tsc"),
            TSCEncodingMode::Decoding => OsStr::new("txt"),
        };

        if !output_dir.is_dir() {
            if !output_dir.exists() {
                let output_dir = self.create_output_dir(output_dir.clone()).unwrap();

                println!("Created output dir: {}", output_dir.display());
            } else {
                return Err(Error::new(ErrorKind::InvalidInput, "Invalid output dir provided"));
            }
        }

        if !input.is_dir() {
            return Err(Error::new(ErrorKind::InvalidInput, "Invalid input dir provided"));
        }

        for entry in std::fs::read_dir(input)? {
            let path = entry?.path();
            if path.file_name().is_none() {
                continue;
            }

            if path.is_dir() {
                let output = output_dir.clone().join(path.file_name().unwrap());

                std::fs::create_dir_all(output.clone())?;

                num += match mode {
                    TSCEncodingMode::Encoding => {
                        self.process(mode, path.into_boxed_path(), output.into_boxed_path())?
                    }
                    TSCEncodingMode::Decoding => {
                        self.process(mode, path.into_boxed_path(), output.into_boxed_path())?
                    }
                };
            } else {
                if path.extension().is_none() {
                    continue;
                }

                if path.extension().unwrap() != file_ext {
                    continue;
                }

                let mut output_path = output_dir.to_path_buf();
                output_path.push(path.file_name().unwrap());
                output_path.set_extension(out_file_ext);

                self.process_file(mode, &path, &output_path)?;

                num += 1;
            }
        }

        Ok(num)
    }

    fn process_file(&self, mode: TSCEncodingMode, input_path: &Path, output_path: &Path) -> std::io::Result<()> {
        let mut buf = std::fs::read(&input_path)?;
        if buf.len() < 3 {
            return Ok(());
        }

        let half = buf.len() / 2;
        let key = if let Some(0) = buf.get(half) { 0x7 } else { *buf.get(half).unwrap() };

        for (idx, byte) in buf.iter_mut().enumerate() {
            if idx == half {
                continue;
            }

            *byte = match mode {
                TSCEncodingMode::Encoding => byte.wrapping_add(key),
                TSCEncodingMode::Decoding => byte.wrapping_sub(key),
            };
        }

        std::fs::write(&output_path, buf.as_slice())?;

        if self.verbose {
            match mode {
                TSCEncodingMode::Encoding => {
                    println!("File {} encoded to {}", input_path.display(), output_path.display())
                }
                TSCEncodingMode::Decoding => {
                    println!("File {} decoded to {}", input_path.display(), output_path.display())
                }
            }
        }

        Ok(())
    }

    pub fn encode(&self, input: Box<Path>, output_dir: Box<Path>) -> std::io::Result<usize> {
        self.process(TSCEncodingMode::Encoding, input.clone(), output_dir.clone())
    }

    pub fn decode(&self, input: Box<Path>, output_dir: Box<Path>) -> std::io::Result<usize> {
        self.process(TSCEncodingMode::Decoding, input.clone(), output_dir.clone())
    }

    fn create_output_dir(&self, out: Box<Path>) -> std::io::Result<Box<Path>> {
        //TODO: create `output1` folder, if `output` already exists;
        //create `output2`, if `output1` already exists etc.
        let mut output_dir = env::current_dir().unwrap();
        output_dir.push(out.file_name().unwrap_or(&OsStr::new("output")));

        std::fs::create_dir(output_dir.clone()).unwrap();

        Ok(output_dir.into_boxed_path())
    }
}
