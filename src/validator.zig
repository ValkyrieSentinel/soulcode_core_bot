const std = @import("std");

/// Ультра-швидка валідація P2P замовлення на рівні пам'яті (Ring 3)
/// Перевіряє коректність суми та структури ідентифікатора замовлення.
pub export fn validate_p2p_signature(amount: f64, order_id_ptr: [*]const u8, id_len: usize) bool {
    // Безпечне приведення сирого вказівника до зрізу (slice) в Zig
    const order_id = order_id_ptr[0..id_len];
    
    // Базова валідація бізнес-логіки
    if (amount <= 0.0) {
        return false;
    }
    
    if (order_id.len == 0 or order_id.len > 64) {
        return false;
    }
    
    // Перевірка структури: UUID має містити дефіси у правильних індексах
    // Формат UUID: xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx (36 символів)
    if (order_id.len == 36) {
        if (order_id[8] != '-' or order_id[13] != '-' or order_id[18] != '-' or order_id[23] != '-') {
            return false;
        }
    }
    
    return true;
}
