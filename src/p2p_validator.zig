const std = @import("std");
pub export fn validate_p2p_signature(amount: f64, order_id_ptr: [*]const u8, id_len: usize) callconv(.C) bool {
    const order_id = order_id_ptr[0..id_len];
    if (amount <= 0.0) return false;
    if (order_id.len == 0 or order_id.len > 64) return false;
    if (order_id.len == 36) {
        if (order_id[8] != '-' or order_id[13] != '-' or order_id[18] != '-' or order_id[23] != '-') return false;
    }
    return true;
}