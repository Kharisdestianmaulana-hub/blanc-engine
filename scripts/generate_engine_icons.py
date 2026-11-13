"""
Blanc Engine - Icon Asset Generator
Processes LogoBlancEngine.png to create high-quality raster icons and multi-resolution .ico files.
"""

import os
import math
from PIL import Image, ImageDraw

def create_icons():
    source_path = "LogoBlancEngine.png"
    if not os.path.exists(source_path):
        print(f"Error: {source_path} not found!")
        return

    out_dir = os.path.join("assets", "icons")
    os.makedirs(out_dir, exist_ok=True)
    orig_dir = os.path.join(out_dir, "original")
    os.makedirs(orig_dir, exist_ok=True)

    print(f"Loading {source_path}...")
    src = Image.open(source_path).convert("RGB")
    w, h = src.size
    bg_color = (18, 20, 25)

    # 1. Generate Full Frame icons (original composition resized directly)
    standard_sizes = [512, 256, 128, 64, 48, 32, 24, 16]
    for size in standard_sizes:
        resized = src.resize((size, size), Image.Resampling.LANCZOS)
        resized.save(os.path.join(orig_dir, f"blanc_full_{size}.png"), format="PNG", optimize=True)

    # 2. Extract Transparent Logo (Anti-aliased background keying)
    print("Generating anti-aliased transparent cutout...")
    rgba = Image.new("RGBA", (w, h))
    src_pix = src.load()
    dst_pix = rgba.load()

    for y in range(h):
        for x in range(w):
            r, g, b = src_pix[x, y]
            dist = math.sqrt((r - bg_color[0])**2 + (g - bg_color[1])**2 + (b - bg_color[2])**2)
            if dist < 12:
                dst_pix[x, y] = (0, 0, 0, 0)
            elif dist > 42:
                dst_pix[x, y] = (r, g, b, 255)
            else:
                t = (dist - 12) / (42 - 12)
                alpha = int(t * 255)
                # de-bleed background color for clean alpha blending on light/dark surfaces
                dst_pix[x, y] = (r, g, b, alpha)

    # 3. Compute tight bounding box of transparent logo symbol
    bbox = rgba.getbbox() # (min_x, min_y, max_x, max_y)
    print(f"Detected Symbol Bounding Box: {bbox}")
    sym_w = bbox[2] - bbox[0]
    sym_h = bbox[3] - bbox[1]
    sym_center_x = (bbox[0] + bbox[2]) / 2.0
    sym_center_y = (bbox[1] + bbox[3]) / 2.0

    # Define standard app-icon framing:
    # Symbol occupies ~80% of canvas dimension with ~10% margin on each side
    max_sym_dim = max(sym_w, sym_h)
    frame_size = int(max_sym_dim / 0.80)
    # Ensure square frame centered on symbol
    crop_x1 = int(sym_center_x - frame_size / 2)
    crop_y1 = int(sym_center_y - frame_size / 2)
    crop_x2 = crop_x1 + frame_size
    crop_y2 = crop_y1 + frame_size

    # Crop symbol with framing
    # For solid dark background icon:
    icon_solid_master = Image.new("RGBA", (frame_size, frame_size), bg_color + (255,))
    # Paste the transparent symbol centered
    paste_x = int((frame_size - sym_w) / 2)
    paste_y = int((frame_size - sym_h) / 2)
    symbol_cropped = rgba.crop(bbox)
    icon_solid_master.paste(symbol_cropped, (paste_x, paste_y), symbol_cropped)

    # For transparent icon:
    icon_trans_master = Image.new("RGBA", (frame_size, frame_size), (0, 0, 0, 0))
    icon_trans_master.paste(symbol_cropped, (paste_x, paste_y), symbol_cropped)

    # 4. Generate all standard sizes for Solid and Transparent icons
    solid_images_for_ico = []
    trans_images_for_ico = []

    print("Generating standard icon resolutions...")
    for size in standard_sizes:
        # Solid icon
        s_img = icon_solid_master.resize((size, size), Image.Resampling.LANCZOS)
        s_path = os.path.join(out_dir, f"icon_{size}.png")
        s_img.save(s_path, format="PNG", optimize=True)
        if size <= 256:
            solid_images_for_ico.append(s_img)

        # Transparent icon
        t_img = icon_trans_master.resize((size, size), Image.Resampling.LANCZOS)
        t_path = os.path.join(out_dir, f"icon_transparent_{size}.png")
        t_img.save(t_path, format="PNG", optimize=True)
        if size <= 256:
            trans_images_for_ico.append(t_img)

    # 5. Generate Modern Squircle (Rounded Rect) variant for 512, 256, 128, 64
    for size in [512, 256, 128, 64]:
        mask = Image.new("L", (size, size), 0)
        draw = ImageDraw.Draw(mask)
        radius = int(size * 0.22) # ~22% squircle radius
        draw.rounded_rectangle((0, 0, size - 1, size - 1), radius=radius, fill=255)
        
        sq_img = icon_solid_master.resize((size, size), Image.Resampling.LANCZOS)
        sq_result = Image.new("RGBA", (size, size), (0, 0, 0, 0))
        sq_result.paste(sq_img, (0, 0), mask)
        sq_result.save(os.path.join(out_dir, f"icon_squircle_{size}.png"), format="PNG", optimize=True)

    # 6. Save Multi-resolution Windows .ico files
    ico_sizes = [(s, s) for s in [256, 128, 64, 48, 32, 24, 16]]
    
    # Solid .ico
    solid_ico_path = os.path.join(out_dir, "blanc_engine.ico")
    icon_solid_master.resize((256, 256), Image.Resampling.LANCZOS).save(
        solid_ico_path,
        format="ICO",
        sizes=ico_sizes
    )

    # Transparent .ico
    trans_ico_path = os.path.join(out_dir, "blanc_engine_transparent.ico")
    icon_trans_master.resize((256, 256), Image.Resampling.LANCZOS).save(
        trans_ico_path,
        format="ICO",
        sizes=ico_sizes
    )

    print("All engine icons generated successfully!")
    print(f"Output files in: {out_dir}")

if __name__ == "__main__":
    create_icons()
