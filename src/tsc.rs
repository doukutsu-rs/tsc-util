use std::boxed::Box;
use std::env;
use std::ffi::OsStr;
use std::io::{Error, ErrorKind};
use std::path::Path;

pub struct TSC {
    verbose: bool,
}

impl TSC {
    pub fn new(verbose: bool) -> Self {
        Self { verbose: verbose }
    }

    pub fn encode(&self, input: Box<Path>, output_dir: Box<Path>) -> std::io::Result<usize> {
        let mut num = 0;

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

                num += self.encode(path.into_boxed_path(), output.into_boxed_path())?;
            } else {
                if path.extension().is_none() {
                    continue;
                }

                if path.extension().unwrap() != "txt" {
                    continue;
                }

                let mut output_path = output_dir.to_path_buf();
                output_path.push(path.file_name().unwrap());
                output_path.set_extension("tsc");

                self.encode_file(&path, &output_path)?;

                num += 1;
            }
        }

        Ok(num)
    }

    fn encode_file(&self, input_path: &Path, output_path: &Path) -> std::io::Result<()> {
        let mut buf = std::fs::read(&input_path)?;
        if buf.len() < 3 {
            return Ok(());
        }

        //Encode
        let half = buf.len() / 2;
        let key = if let Some(0) = buf.get(half) { 0x7 } else { *buf.get(half).unwrap() };

        for (idx, byte) in buf.iter_mut().enumerate() {
            if idx == half {
                continue;
            }

            *byte = byte.wrapping_add(key);
        }

        std::fs::write(&output_path, buf.as_slice())?;

        if self.verbose {
            println!("File {} encoded to {}", input_path.display(), output_path.display());
        }

        Ok(())
    }

    pub fn decode(&self, input: Box<Path>, output_dir: Box<Path>) -> std::io::Result<usize> {
        let mut num = 0;

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

                num += self.decode(path.into_boxed_path(), output.into_boxed_path())?;
            } else {
                if path.extension().is_none() {
                    continue;
                }

                if path.extension().unwrap() != "tsc" {
                    continue;
                }

                let mut output_path = output_dir.to_path_buf();
                output_path.push(path.file_name().unwrap());
                output_path.set_extension("txt");

                self.decode_file(&path, &output_path)?;

                num += 1;
            }
        }

        Ok(num)
    }

    fn decode_file(&self, input_path: &Path, output_path: &Path) -> std::io::Result<()> {
        let mut buf = std::fs::read(&input_path)?;
        if buf.len() < 3 {
            return Ok(());
        }

        //Decode
        let half = buf.len() / 2;
        let key = if let Some(0) = buf.get(half) { 0x7 } else { *buf.get(half).unwrap() };

        for (idx, byte) in buf.iter_mut().enumerate() {
            if idx == half {
                continue;
            }

            *byte = byte.wrapping_sub(key);
        }

        std::fs::write(&output_path, buf.as_slice())?;

        if self.verbose {
            println!("File {} decoded to {}", input_path.display(), output_path.display())
        };

        Ok(())
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
