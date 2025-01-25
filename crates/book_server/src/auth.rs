use axum::{extract::Request, middleware::Next, response::Response, Extension};
use http::StatusCode;
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Permission {
    ReadBook,
    WriteBook,
    ManageUsers,
    ManageSystem,
}

#[derive(Clone, Debug)]
pub enum Role {
    Admin(HashSet<Permission>),
    User(HashSet<Permission>),
    Guest(HashSet<Permission>),
}

impl Role {
    pub fn admin() -> Self {
        let mut permissions = HashSet::new();
        permissions.insert(Permission::ReadBook);
        permissions.insert(Permission::WriteBook);
        permissions.insert(Permission::ManageUsers);
        permissions.insert(Permission::ManageSystem);
        Role::Admin(permissions)
    }

    pub fn user() -> Self {
        let mut permissions = HashSet::new();
        permissions.insert(Permission::ReadBook);
        permissions.insert(Permission::WriteBook);
        Role::User(permissions)
    }

    pub fn guest() -> Self {
        let mut permissions = HashSet::new();
        permissions.insert(Permission::ReadBook);
        Role::Guest(permissions)
    }

    pub fn has_permission(&self, permission: &Permission) -> bool {
        match self {
            Role::Admin(perms) | Role::User(perms) | Role::Guest(perms) => {
                perms.contains(permission)
            }
        }
    }
}

pub async fn require_permission(
    Extension(role): Extension<Role>,
    Extension(permission): Extension<Permission>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if !role.has_permission(&permission) {
        return Err(StatusCode::FORBIDDEN);
    }
    Ok(next.run(req).await)
}
