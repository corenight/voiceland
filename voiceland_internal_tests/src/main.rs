use std::ffi::CString;

use rsmpeg::avcodec;

fn main() {
    let name = "libopus";

    let a = avcodec::AVCodec::find_encoder_by_name(&CString::new(name).unwrap()).unwrap();

    println!("{:#?}", a.pix_fmts());
}
