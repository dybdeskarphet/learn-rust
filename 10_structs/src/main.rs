fn main() {
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u32,
    };

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

    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    };

    let user1 = User {
        email: String::from("deneme123@gmail.com"),
        username: String::from("onionland"),
        sign_in_count: 1,
        active: true,
    };

    let user2 = User {
        username: String::from("garlicland"),
        email: String::from("deneme1234@gmail.com"),
        ..user1
    };

    println!("{:?}", user1.sign_in_count);
    println!("{:?}", user2.sign_in_count);
}
