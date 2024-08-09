fn main() {
    struct Book {
        name: String,
        release_date: u16,
        author: String,
        edition: u16,
        is_poetry: bool,
    };

    let uvercinka = Book {
        name: String::from("Üvercinka"),
        release_date: 1958,
        author: String::from("Cemal Süreya"),
        edition: 4,
        is_poetry: true,
    };

    println!("{:?}", uvercinka.name);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
