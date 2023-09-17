use crate::scheme::*;
use anyhow::Result;
use std::collections::BTreeMap;
use std::path::PathBuf;
use templar;
use templar::*;

fn generate_template(original: PathBuf, replaced: PathBuf, scheme: &Scheme) -> Result<()> {
    let mut content = String::new();
    if let Ok(cont) = std::fs::read_to_string(original) {
        content = cont;
    }

    // if let Err(e) = templar::Templar::global().parse(&content) {
    //     println!("{}", e);
    // };

    let template = templar::Templar::global().parse(&content)?;
    let mut data = BTreeMap::new();
    // let mut data: templar::Document = templar::Document::default();
    if let Some(colors) = scheme.colors() {
        for (i, color) in colors.iter().enumerate() {
            let name = "color".to_string() + &i.to_string();
            data.insert(name.into(), color.to_rgb_hex_string(false).into());
        }
        data.insert(
            "background".into(),
            colors[0].to_rgb_hex_string(false).into(),
        );
        data.insert(
            "foreground".into(),
            colors[15].to_rgb_hex_string(false).into(),
        );
        data.insert("cursor".into(), colors[1].to_rgb_hex_string(false).into());
        data.insert("accent".into(), colors[1].to_rgb_hex_string(false).into());
    }

    if let Some(wallpaper) = scheme.image() {
        data.insert("wallpaper".into(), wallpaper.into());
    }
    if let Some(theme) = scheme.theme() {
        data.insert("theme".into(), theme.into());
    }

    let context = templar::StandardContext::new();
    context.set(templar::InnerData::Map(data))?;

    let new_content = (template.render(&context)?).to_string();
    std::fs::write(replaced, new_content.as_bytes()).unwrap();
    Ok(())
}

pub fn pattern_gneration(scheme: &mut Scheme) -> Result<()> {
    if let Some(patterns) = scheme.patterns() {
        for p in patterns.iter() {
            if std::fs::metadata(&p.0).is_ok() && std::fs::metadata(&p.1).is_ok() {
                generate_template(PathBuf::from(&p.0), PathBuf::from(&p.1), scheme)?;
                println!("generating :{} into: {}", p.0, p.1)
            } else {
                //TODO: better error handle
                println!("{} or {} is not a valid file", p.0, p.1)
            }
        }
    }

    Ok(())
}
