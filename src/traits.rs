use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserId(pub Uuid);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StaffRole {
    Admin,
    Support,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CustomerType {
    Beta,
    Regular,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserType {
    Staff(StaffRole),
    Customer(CustomerType),
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub email: String,
    pub user_type: UserType,
}

impl User {
    pub fn new(name: String, email: String, user_type: UserType) -> Self {
        Self {
            id: UserId(Uuid::new_v4()),
            name,
            email,
            user_type,
        }
    }
}
