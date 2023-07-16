rust
    pub fn get_live_order_mut(&mut self, client_id: &str, server_id: &str) -> Option<&mut Order> {
        let this = self as *mut Self;
        unsafe {
            if let Some(x) = (*this).live_orders.get_by_cid_mut(client_id) {
                Some(x)
            } else {
                (*this).live_orders.get_by_sid_mut(server_id)
            }
        }
    }
