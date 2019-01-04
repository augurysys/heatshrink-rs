#[repr(C)]
#[derive(Debug)]
pub struct HeatshrinkDecoder {
    input_size: u16,
    input_index: u16,
    output_count: u16,
    output_index: u16,
    head_index: u16,
    state: u8,
    current_byte: u8,
    bit_index: u8,
}

#[repr(C)]
#[derive(Debug)]
pub enum HSD_sink_res {
    Ok,
    Full,
    ErrorNull = -1,
}

#[repr(C)]
#[derive(Debug)]
pub enum HSD_poll_res {
    Empty,
    More,
    ErrorNull = -1,
    ErrorUnknown = -2,
}

#[repr(C)]
#[derive(Debug)]
pub enum HSD_finish_res {
    Done,
    More,
    ErrorNull = -1,
}

#[link(name = "heatshrink_static", kind = "dylib")]
extern "C" {
    fn heatshrink_decoder_reset(hearshrink_decoder: *mut HeatshrinkDecoder);
    fn heatshrink_decoder_sink(
        heatshrink_decoder: *mut HeatshrinkDecoder,
        in_buf: *mut u8,
        size: usize,
        input_size: *mut usize,
    ) -> HSD_sink_res;
    fn heatshrink_decoder_poll(
        heatshrink_decoder: *mut HeatshrinkDecoder,
        out_buf: *mut u8,
        out_buf_size: usize,
        output_size: *mut usize,
    ) -> HSD_poll_res;
    fn heatshrink_decoder_finish(heatshrink_decoder: *mut HeatshrinkDecoder);
}

impl HeatshrinkDecoder {
    pub fn new() -> HeatshrinkDecoder {
        HeatshrinkDecoder {
            input_size: 0,
            input_index: 0,
            output_count: 0,
            output_index: 0,
            head_index: 0,
            state: 0,
            current_byte: 0,
            bit_index: 0,
        }
    }

    pub fn reset(&mut self) {
        unsafe {
            heatshrink_decoder_reset(self);
        }
    }

    pub fn sink(&mut self, in_buf: &mut Vec<u8>, input_size: &mut usize) -> HSD_sink_res {
        unsafe { heatshrink_decoder_sink(self, in_buf.as_mut_ptr(), in_buf.len(), input_size) }
    }

    pub fn poll(&mut self, out_buf: &mut Vec<u8>, output_size: &mut usize) -> HSD_poll_res {
        unsafe { heatshrink_decoder_poll(self, out_buf.as_mut_ptr(), out_buf.len(), output_size) }
    }

    pub fn finish(&mut self) {
        unsafe {
            heatshrink_decoder_finish(self);
        }
    }
}
