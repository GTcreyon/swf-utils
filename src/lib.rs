/// Signature found at the end of a bundle that indicates that a SWF file is present.
const SIGNATURE: [u8; 4] = [86, 52, 18, 250];

/// If the buffer contains a valid SWF header, return Some containing the size of the SWF file in bytes.
/// Else, return None.
pub fn swf_size_from_buf(buf: &Vec<u8>) -> Option<u32> {
    if buf_is_bundle(buf) {
        // Get the size from the final 4 bytes
        let offset = buf.len() - 4;
        let buf_size = buf[offset..].try_into().unwrap();
        Some(u32::from_le_bytes(buf_size))
    }
    else {
        None
    }
}

/// Check if the buffer contains a valid SWF header - i.e. it is a bundled SWF/EXE.
pub fn buf_is_bundle(buf: &Vec<u8>) -> bool {
    // Get the bytes from where the signature should be
    let offset = buf.len() - 8;
    let swf_signature: [u8; 4] = buf[offset .. offset + 4].try_into().unwrap();

    // Return true if the signature is 0xFA123456
    swf_signature == SIGNATURE
}

/// Return the bytes of the SWF file contained in a given bundle buffer.
/// Must also pass in the size of the SWF file.
/// This can be obtained with swf_size_from_buf().
pub fn swf_bytes_from_buf(buf: &Vec<u8>, size: &u32) -> Vec<u8> {
    let offset: usize = (8 + size).try_into().unwrap();
    let length = buf.len();
    buf[length - offset .. length - 8].try_into().unwrap()
}

/// Return the bytes of the standalone projector contained in a given bundle buffer.
/// Must also pass in the size of the SWF file.
/// This can be obtained with swf_size_from_buf().
pub fn exe_bytes_from_buf(buf: &Vec<u8>, size: &u32) -> Vec<u8> {
    let offset: usize = (8 + size).try_into().unwrap();
    let length = buf.len();
    buf[..length - offset].try_into().unwrap()
}

/// Bundles the bytes of a SWF file together with the bytes from a standalone executable.
pub fn bundle_swf_exe_bytes(swf: &mut Vec<u8>, exe: &mut Vec<u8>) -> Vec<u8> {
    // Get the size of the SWF, convert it to bytes.
    let size = (swf.len() as u32).to_le_bytes();
    exe.append(swf);
    exe.extend_from_slice(&SIGNATURE);
    exe.extend_from_slice(&size);
    exe.to_vec()
}
