// ユーザ情報
pub struct DomainUser {
    id: i32,
    // ユーザ名
    name: String,
    // ユーザ画像
    image: String,
}

impl DomainUser {
    pub fn new(id: i32, name: String, image: String) -> DomainUser {
        DomainUser {
            id,
            name,
            image,
        }
    }
    pub fn id(&self) -> i32 {
        self.id
    }
    pub fn zname(&self) -> String {
        self.name.to_string()
    }
    pub fn image(&self) -> String {
        self.image.to_string()
    }
}