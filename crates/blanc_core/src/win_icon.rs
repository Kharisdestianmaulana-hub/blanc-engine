use image::imageops::FilterType;
use image::{DynamicImage, ImageFormat, Rgba, RgbaImage};
use std::io::Cursor;
use std::path::Path;

/// Decode PNG/image bytes and resize to 64x64 RGBA for window titlebar/taskbar icons.
pub fn decode_window_icon_rgba(src_bytes: &[u8]) -> Option<(Vec<u8>, u32, u32)> {
    let img = image::load_from_memory(src_bytes).ok()?;
    let resized = img.resize_exact(64, 64, FilterType::Lanczos3).to_rgba8();
    Some((resized.into_raw(), 64, 64))
}

/// Generate a crisp 256x256 default Blanc Engine logo PNG (dark rounded tile + indigo diamond).
pub fn default_blanc_logo_png() -> Vec<u8> {
    let size = 256u32;
    let mut img = RgbaImage::new(size, size);
    let center = 127.5f32;
    let corner_r = 44.0f32;

    for y in 0..size {
        for x in 0..size {
            let fx = x as f32;
            let fy = y as f32;
            // Rounded square SDF
            let dx = (fx - center).abs() - (center - corner_r);
            let dy = (fy - center).abs() - (center - corner_r);
            let dist = dx.max(0.0).hypot(dy.max(0.0)) + dx.max(dy).min(0.0) - corner_r;

            if dist <= 0.0 {
                // Diamond in center
                let md = (fx - center).abs() + (fy - center).abs();
                if md <= 78.0 {
                    let inner = (fx - center).abs() + (fy - center).abs();
                    if inner <= 34.0 {
                        img.put_pixel(x, y, Rgba([165, 180, 252, 255]));
                    } else {
                        img.put_pixel(x, y, Rgba([99, 102, 241, 255]));
                    }
                } else {
                    img.put_pixel(x, y, Rgba([20, 24, 38, 255]));
                }
            } else {
                img.put_pixel(x, y, Rgba([0, 0, 0, 0]));
            }
        }
    }

    let mut out = Vec::new();
    let _ = DynamicImage::ImageRgba8(img).write_to(&mut Cursor::new(&mut out), ImageFormat::Png);
    out
}

/// Convert PNG/image bytes into a multi-resolution `.ico` byte buffer (256, 48, 32, 16).
pub fn png_to_ico_bytes(src_bytes: &[u8]) -> Result<Vec<u8>, String> {
    let img = image::load_from_memory(src_bytes)
        .map_err(|e| format!("Failed to decode icon image: {}", e))?;

    let sizes: [u32; 4] = [256, 48, 32, 16];
    let mut frames: Vec<(u32, Vec<u8>)> = Vec::with_capacity(sizes.len());
    for s in sizes {
        let resized = img.resize_exact(s, s, FilterType::Lanczos3);
        let mut png_buf = Vec::new();
        resized
            .write_to(&mut Cursor::new(&mut png_buf), ImageFormat::Png)
            .map_err(|e| format!("Failed to encode {}px icon frame: {}", s, e))?;
        frames.push((s, png_buf));
    }

    let count = frames.len() as u16;
    let mut ico = Vec::new();
    // ICONDIR header: reserved(0), type(1=ICO), count
    ico.extend_from_slice(&0u16.to_le_bytes());
    ico.extend_from_slice(&1u16.to_le_bytes());
    ico.extend_from_slice(&count.to_le_bytes());

    let mut offset = 6u32 + (frames.len() as u32) * 16u32;
    for (s, data) in &frames {
        let dim = if *s >= 256 { 0u8 } else { *s as u8 };
        ico.push(dim); // bWidth
        ico.push(dim); // bHeight
        ico.push(0);   // bColorCount
        ico.push(0);   // bReserved
        ico.extend_from_slice(&1u16.to_le_bytes());  // wPlanes
        ico.extend_from_slice(&32u16.to_le_bytes()); // wBitCount
        ico.extend_from_slice(&(data.len() as u32).to_le_bytes());
        ico.extend_from_slice(&offset.to_le_bytes());
        offset += data.len() as u32;
    }

    for (_, data) in frames {
        ico.extend_from_slice(&data);
    }

    Ok(ico)
}

/// Embed a PNG (or the default Blanc logo if `None`) as the Windows PE executable icon.
pub fn embed_png_icon_into_exe(exe_path: &Path, png_bytes: Option<&[u8]>) -> Result<(), String> {
    let fallback;
    let src = match png_bytes {
        Some(b) if !b.is_empty() => b,
        _ => {
            fallback = default_blanc_logo_png();
            &fallback
        }
    };
    let ico_bytes = png_to_ico_bytes(src)?;
    embed_ico_into_exe(exe_path, &ico_bytes)
}

#[cfg(windows)]
fn embed_ico_into_exe(exe_path: &Path, ico_bytes: &[u8]) -> Result<(), String> {
    use std::ffi::c_void;
    use std::os::windows::ffi::OsStrExt;

    #[link(name = "kernel32")]
    extern "system" {
        fn BeginUpdateResourceW(pFileName: *const u16, bDeleteExistingResources: i32) -> *mut c_void;
        fn UpdateResourceW(
            hUpdate: *mut c_void,
            lpType: *const u16,
            lpName: *const u16,
            wLanguage: u16,
            lpData: *const u8,
            cb: u32,
        ) -> i32;
        fn EndUpdateResourceW(hUpdate: *mut c_void, fDiscard: i32) -> i32;
    }

    if ico_bytes.len() < 6 {
        return Err("Invalid ICO data".to_string());
    }
    let count = u16::from_le_bytes([ico_bytes[4], ico_bytes[5]]) as usize;
    if ico_bytes.len() < 6 + count * 16 {
        return Err("Truncated ICO directory".to_string());
    }

    let mut wide_path: Vec<u16> = exe_path.as_os_str().encode_wide().collect();
    wide_path.push(0);

    unsafe {
        let handle = BeginUpdateResourceW(wide_path.as_ptr(), 0);
        if handle.is_null() {
            return Err(format!(
                "BeginUpdateResourceW failed for {}",
                exe_path.display()
            ));
        }

        // RT_ICON = 3, RT_GROUP_ICON = 14
        let rt_icon = 3usize as *const u16;
        let rt_group_icon = 14usize as *const u16;

        let mut grp = Vec::with_capacity(6 + count * 14);
        grp.extend_from_slice(&0u16.to_le_bytes()); // idReserved
        grp.extend_from_slice(&1u16.to_le_bytes()); // idType = 1
        grp.extend_from_slice(&(count as u16).to_le_bytes());

        for i in 0..count {
            let entry_off = 6 + i * 16;
            let bytes_in_res = u32::from_le_bytes([
                ico_bytes[entry_off + 8],
                ico_bytes[entry_off + 9],
                ico_bytes[entry_off + 10],
                ico_bytes[entry_off + 11],
            ]);
            let image_off = u32::from_le_bytes([
                ico_bytes[entry_off + 12],
                ico_bytes[entry_off + 13],
                ico_bytes[entry_off + 14],
                ico_bytes[entry_off + 15],
            ]) as usize;

            if image_off + (bytes_in_res as usize) > ico_bytes.len() {
                EndUpdateResourceW(handle, 1);
                return Err("ICO frame out of bounds".to_string());
            }

            let icon_id = (i + 1) as u16;
            let ok = UpdateResourceW(
                handle,
                rt_icon,
                (icon_id as usize) as *const u16,
                0,
                ico_bytes[image_off..].as_ptr(),
                bytes_in_res,
            );
            if ok == 0 {
                EndUpdateResourceW(handle, 1);
                return Err("UpdateResourceW failed for RT_ICON".to_string());
            }

            // GRPICONDIRENTRY (14 bytes = first 12 bytes of ICONDIRENTRY + u16 nId)
            grp.extend_from_slice(&ico_bytes[entry_off..entry_off + 12]);
            grp.extend_from_slice(&icon_id.to_le_bytes());
        }

        let ok_grp = UpdateResourceW(
            handle,
            rt_group_icon,
            1usize as *const u16,
            0,
            grp.as_ptr(),
            grp.len() as u32,
        );
        if ok_grp == 0 {
            EndUpdateResourceW(handle, 1);
            return Err("UpdateResourceW failed for RT_GROUP_ICON".to_string());
        }

        if EndUpdateResourceW(handle, 0) == 0 {
            return Err("EndUpdateResourceW failed".to_string());
        }
    }

    Ok(())
}

#[cfg(not(windows))]
fn embed_ico_into_exe(_exe_path: &Path, _ico_bytes: &[u8]) -> Result<(), String> {
    Ok(())
}
