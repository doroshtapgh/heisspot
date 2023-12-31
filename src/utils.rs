use windows::core::{wcslen, PCWSTR};

pub unsafe fn vary_array_to_vec<Origin, To>(length: usize, vary_arr: &[Origin]) -> Vec<To>
    where for<'a> To: From<&'a Origin> {
    let reference = std::ptr::addr_of!(vary_arr[0]);

    let slc = std::slice::from_raw_parts(reference, length);
    
    slc.iter().map(std::convert::Into::into).collect::<Vec<To>>()
}

pub fn vary_utf16_to_string(utf16_str: &[u16]) -> String {
    let ptr = utf16_str.as_ptr();
    
    let slice = unsafe {
        let wptr: PCWSTR = std::mem::transmute(ptr);
        let len = wcslen(wptr);
        std::slice::from_raw_parts(ptr, len)
    };

    String::from_utf16_lossy(slice)
}