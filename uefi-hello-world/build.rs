fn main() {
    let src = png::Decoder::new(std::io::Cursor::new(std::fs::read("logo.png").unwrap()));
    let (info, mut src) = src.read_info().unwrap();
    let mut buf = vec![0u8; info.buffer_size()];
    src.next_frame(&mut buf[..]).unwrap();

    let w = info.width as usize;
    let h = info.height as usize;
    let mut dst = vec![0u8; w * h * 4];
    for y in 0..h {
        for x in 0..w {
            let (r, g, b, a) = match info.color_type {
                png::ColorType::RGB => {
                    let so = 3 * (x + y*h);
                    (buf[so + 0], buf[so + 1], buf[so + 2], 0xFF)
                },
                png::ColorType::RGBA => {
                    let so = 4 * (x + y*h);
                    (buf[so + 0], buf[so + 1], buf[so + 2], buf[so + 3])
                },
                other => panic!("Unsupported png::ColorType::{:?}", other),
            };

            let o = 4 * (x + y*h);
            dst[o + 0] = r;
            dst[o + 1] = g;
            dst[o + 2] = b;
            dst[o + 3] = a;
        }
    }

    std::fs::write("logo.png.dims", format!("({},{})", w, h)).unwrap();
    std::fs::write("logo.png.bin", dst).unwrap();
}
