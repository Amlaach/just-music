import os
import math
from PIL import Image, ImageDraw

def draw_music_icon(size):
    # Create RGBA image with smooth anti-aliasing (render at 4x and downscale)
    scale = 4
    w = size * scale
    img = Image.new("RGBA", (w, w), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)

    # Gradient background squircle
    margin = int(w * 0.06)
    radius = int(w * 0.22)
    rect_box = [margin, margin, w - margin, w - margin]

    # Draw rounded rect with gradient simulation
    gradient_img = Image.new("RGBA", (w, w), (0, 0, 0, 0))
    g_draw = ImageDraw.Draw(gradient_img)

    for y in range(margin, w - margin):
        ratio = (y - margin) / float(w - 2 * margin)
        # Interpolate between #FF416C (255, 65, 108) and #8A2387 (138, 35, 135)
        r = int(255 * (1 - ratio) + 138 * ratio)
        g = int(65 * (1 - ratio) + 35 * ratio)
        b = int(108 * (1 - ratio) + 135 * ratio)
        g_draw.line([(margin, y), (w - margin, y)], fill=(r, g, b, 255))

    # Mask for squircle
    mask = Image.new("L", (w, w), 0)
    m_draw = ImageDraw.Draw(mask)
    m_draw.rounded_rectangle(rect_box, radius=radius, fill=255)

    # Composite gradient with squircle mask
    img = Image.composite(gradient_img, img, mask)
    draw = ImageDraw.Draw(img)

    # Draw outer subtle soundwave circles
    cx, cy = w // 2, w // 2
    r1 = int(w * 0.36)
    r2 = int(w * 0.28)
    draw.ellipse([cx - r1, cy - r1, cx + r1, cy + r1], outline=(255, 255, 255, 30), width=int(w * 0.015))
    draw.ellipse([cx - r2, cy - r2, cx + r2, cy + r2], outline=(255, 255, 255, 50), width=int(w * 0.012))

    # Draw Music Note
    # Left note head
    lx, ly = int(w * 0.36), int(w * 0.65)
    rx1, ry1 = int(w * 0.08), int(w * 0.06)
    draw.ellipse([lx - rx1, ly - ry1, lx + rx1, ly + ry1], fill=(255, 255, 255, 240))

    # Right note head
    rx, ry = int(w * 0.64), int(w * 0.55)
    draw.ellipse([rx - rx1, ry - ry1, rx + rx1, ry + ry1], fill=(255, 255, 255, 240))

    # Stems
    stem_w = max(2, int(w * 0.035))
    draw.rectangle([lx + rx1 - stem_w, int(w * 0.28), lx + rx1, ly], fill=(255, 255, 255, 240))
    draw.rectangle([rx + rx1 - stem_w, int(w * 0.18), rx + rx1, ry], fill=(255, 255, 255, 240))

    # Beam connecting stems
    beam_h = int(w * 0.08)
    beam_pts = [
        (lx + rx1 - stem_w, int(w * 0.28)),
        (rx + rx1, int(w * 0.18)),
        (rx + rx1, int(w * 0.18) + beam_h),
        (lx + rx1 - stem_w, int(w * 0.28) + beam_h)
    ]
    draw.polygon(beam_pts, fill=(255, 255, 255, 240))

    # Equalizer pulse lines on sides
    bar_w = max(1, int(w * 0.016))
    left_bars = [(int(w * 0.18), int(w * 0.08)), (int(w * 0.22), int(w * 0.14)), (int(w * 0.26), int(w * 0.06))]
    right_bars = [(int(w * 0.74), int(w * 0.10)), (int(w * 0.78), int(w * 0.16)), (int(w * 0.82), int(w * 0.07))]

    for bx, bh in left_bars:
        by = cy - bh // 2
        draw.rounded_rectangle([bx, by, bx + bar_w, by + bh], radius=bar_w // 2, fill=(255, 255, 255, 200))
    for bx, bh in right_bars:
        by = cy - bh // 2
        draw.rounded_rectangle([bx, by, bx + bar_w, by + bh], radius=bar_w // 2, fill=(255, 255, 255, 200))

    # Downscale with high quality Lanczos filter
    final_img = img.resize((size, size), Image.Resampling.LANCZOS)
    return final_img

def main():
    sizes = [16, 24, 32, 48, 64, 128, 256, 512]
    assets_dir = r"C:\Users\USER\.gemini\antigravity\scratch\aether-player\assets"
    icons_dir = os.path.join(assets_dir, "icons")
    os.makedirs(icons_dir, exist_ok=True)

    images = {}
    for sz in sizes:
        img = draw_music_icon(sz)
        images[sz] = img
        png_path = os.path.join(icons_dir, f"icon_{sz}x{sz}.png")
        img.save(png_path, format="PNG")
        print(f"Saved {png_path}")

    # Save main 512x512 PNG
    images[512].save(os.path.join(assets_dir, "icon.png"), format="PNG")

    # Save ICO containing multi-resolution frames (16, 24, 32, 48, 64, 128, 256)
    ico_sizes = [16, 24, 32, 48, 64, 128, 256]
    ico_frames = [images[sz] for sz in ico_sizes]
    ico_path = os.path.join(assets_dir, "icon.ico")
    ico_frames[0].save(
        ico_path,
        format="ICO",
        sizes=[(sz, sz) for sz in ico_sizes],
        append_images=ico_frames[1:]
    )
    print(f"Saved multi-res ICO to {ico_path} (size: {os.path.getsize(ico_path)} bytes)")

if __name__ == "__main__":
    main()
