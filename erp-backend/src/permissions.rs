use crate::constants::{ROLE_ADMIN, ROLE_SUPERVISOR};

pub fn can_approve(role_id: i32) -> bool {
    role_id == ROLE_ADMIN || role_id == ROLE_SUPERVISOR
}

#[allow(dead_code)]
pub fn is_admin(role_id: i32) -> bool {
    role_id == ROLE_ADMIN
}

#[allow(dead_code)]
pub fn is_supervisor_or_admin(role_id: i32) -> bool {
    can_approve(role_id)
}