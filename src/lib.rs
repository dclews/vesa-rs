#![no_std]

//#[repr(C,packed)]
//pub struct vbe_info {
//    sig: [u8; 4],
//    entry_point: u16,
//    pm_initialise
//}

#[repr(C)]
pub enum VbeMemoryModel {
    Text = 0x0,
    Cga,
    Hercules,
    Planar,
    PackedPixel,
    NonChain4K,
    YUV,
}

#[derive(Copy, Clone, Debug)]
#[repr(C,packed)]
pub struct vbe_mode_info {
    pub attributes: u16,
    pub win_a_attributes: u8,
    pub win_b_attributes: u8,
    pub win_granularity: u16,
    pub win_size: u16,
    pub win_a_segment: u16,
    pub win_b_segment: u16,
    pub win_func_ptr: u32,
    pub bytes_per_scanline: u16,

    //VBE 1.2+
    pub x_res: u16,
    pub y_res: u16,
    pub x_char_size: u8,
    pub y_char_size: u8,
    pub num_planes: u8,
    pub bits_per_pixel: u8,
    pub num_banks: u8,
    pub mem_model: u8,
    pub bank_size: u8,
    pub num_image_pages: u8,
    reserved: u8,

    //Direct colour. Required for direct/6 and YUV/7 memory models.
    pub red_mask_size: u8,
    pub red_field_pos: u8,
    pub green_mask_size: u8,
    pub green_field_pos: u8,
    pub blue_mask_size: u8,
    pub blue_mask_pos: u8,
    reserved_mask_size: u8,
    reserved_field_position: u8,
    pub direct_color_mode_info: u8,

    //VBE2.0+
    pub phys_base_ptr: u32,
    reserved2: u32,
    reserved3: u16,

    //VBE3.0+
    pub linear_bytes_per_scanline: u16,
    pub bank_num_image_pages: u8,
    pub linear_num_image_pages: u8,
    pub linear_red_mask_size: u8,
    pub linear_red_field_pos: u8,
    pub linear_green_mask_size: u8,
    pub linear_green_field_pos: u8,
    pub linear_blue_mask_size: u8,
    pub linear_blue_field_pos: u8,
    pub linear_reserved_mask_size: u8,
    pub linear_reserved_field_pos: u8,
    pub max_pixel_clock: u32,
}
