use std::{
    fs::{read, read_dir, File},
    io::Write,
};

use anyhow::{anyhow, Result};
use naga::{
    back::{spv, wgsl as b_wgsl},
    front::wgsl,
    valid::{Capabilities, ValidationFlags, Validator},
};

use naga_oil::compose::{ComposableModuleDescriptor, Composer, NagaModuleDescriptor};

const SHADERS_DIR: &str = "./shaders";
const SPV_DIR: &str = "./spv";

fn main() -> Result<()> {
    for entry in read_dir(SHADERS_DIR)? {
        let entry = entry?;

        let entry_ftype = entry.file_type()?;
        if entry_ftype.is_dir() {
            let file_name = entry
                .path()
                .file_name()
                .ok_or(anyhow!("Filename is not set"))?
                .to_str()
                .unwrap()
                .to_string();

            if file_name.contains("composed") {
                let shader_name = &file_name[9..];
                let common_shader_name = format!("{SHADERS_DIR}/{file_name}/{shader_name}.wgsl");
                let mut composer = Composer::default();

                for sub_entry in read_dir(entry.path())? {
                    let sub_entry = sub_entry?;
                    let mut load_composable = |source: &str, file_path: &str| match composer
                        .add_composable_module(ComposableModuleDescriptor {
                            source,
                            file_path,
                            ..Default::default()
                        }) {
                        Ok(_module) => {}
                        Err(e) => {
                            println!("? -> {e:#?}")
                        }
                    };

                    let sub_entry_path = sub_entry.path();
                    let sub_source = String::from_utf8(read(sub_entry_path)?)?;
                    let sub_file_name = sub_entry
                        .path()
                        .file_name()
                        .ok_or(anyhow!("Filename is not set"))?
                        .to_str()
                        .unwrap()
                        .to_string();

                    if !sub_file_name.contains(shader_name) {
                        load_composable(&sub_source, &sub_file_name);
                    }
                }

                let source = String::from_utf8(read(common_shader_name.clone())?)?;
                let module = composer.make_naga_module(NagaModuleDescriptor {
                    source: &source,
                    file_path: &format!("{SHADERS_DIR}/{common_shader_name}.wgsl"),
                    shader_defs: [(Default::default())].into(),
                    ..Default::default()
                })?;
                let info = Validator::new(ValidationFlags::all(), Capabilities::default())
                    .validate(&module)?;

                let wgsl_bytes =
                    b_wgsl::write_string(&module, &info, b_wgsl::WriterFlags::EXPLICIT_TYPES)
                        .unwrap();
                let mut wgsl_file = File::create(common_shader_name)?;

                wgsl_file.write_all(wgsl_bytes.as_bytes())?;
            }
        } else {
            let entry_path = entry.path();
            let mut spv_entry = entry_path.clone();
            spv_entry.set_extension("");

            let spv_file_name = spv_entry
                .file_name()
                .ok_or(anyhow!("Filename is not set"))?
                .to_str()
                .unwrap();
            let spv_file_name = format!("{SPV_DIR}/{spv_file_name}.spv");
            let mut spv_file = File::create(spv_file_name)?;

            let sh_data = read(entry_path)?;
            let sh_module = wgsl::parse_str(&String::from_utf8(sh_data)?)?;
            let sh_info = Validator::new(
                ValidationFlags::default(),
                Capabilities::CLIP_DISTANCE | Capabilities::CULL_DISTANCE,
            )
            .validate(&sh_module)?;

            let spv_data = spv::write_vec(&sh_module, &sh_info, &Default::default(), None)?;
            let spv_bytes =
                spv_data
                    .iter()
                    .fold(Vec::with_capacity(spv_data.len() * 4), |mut v, w| {
                        v.extend_from_slice(&w.to_le_bytes());
                        v
                    });

            spv_file.write_all(&spv_bytes)?;
        }
    }

    Ok(())
}
