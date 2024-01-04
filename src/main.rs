use std::{
    fs::{read, read_dir, File},
    io::Write,
};

use anyhow::{anyhow, Result};
use naga::{
    back::spv,
    front::wgsl,
    valid::{Capabilities, ValidationFlags, Validator},
};

const SHADERS_DIR: &'static str = "./shaders";
const SPV_DIR: &'static str = "./spv";

fn main() -> Result<()> {
    for entry in read_dir(SHADERS_DIR)? {
        let entry = entry?.path();
        let mut spv_entry = entry.clone();
        spv_entry.set_extension("");

        let spv_file_name = spv_entry
            .file_name()
            .ok_or(anyhow!("Filename is not set"))?
            .to_str()
            .unwrap();
        let spv_file_name = format!("{SPV_DIR}/{spv_file_name}.spv");
        let mut spv_file = File::create(spv_file_name)?;

        let sh_data = read(entry)?;
        let sh_module = wgsl::parse_str(&String::from_utf8(sh_data)?)?;
        let sh_info = Validator::new(
            ValidationFlags::default(),
            Capabilities::CLIP_DISTANCE | Capabilities::CULL_DISTANCE,
        )
        .validate(&sh_module)?;

        let spv_data = spv::write_vec(&sh_module, &sh_info, &Default::default(), None)?;
        let spv_bytes = spv_data
            .iter()
            .fold(Vec::with_capacity(spv_data.len() * 4), |mut v, w| {
                v.extend_from_slice(&w.to_le_bytes());
                v
            });

        spv_file.write_all(&spv_bytes)?;
    }

    Ok(())
}
