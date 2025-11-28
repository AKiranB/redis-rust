pub fn format_resp(buf: &mut Vec<u8>, type_byte: u8, payload: &[u8]) {
    buf.push(type_byte);
    buf.extend_from_slice(payload);
    let delimiter = b"\r\n";
    buf.extend_from_slice(delimiter);
}
