// This has many warnings,
// Chapter 10 will cover how to fix

// --------
struct AlwaysEqual;

// --------
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// --------
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {

    // --------
    let subject = AlwaysEqual;

    // --------
    let black = Color(30, 31, 33);
    let origin = Point(0, 0, 0,);
    
    // --------
    // Create admin user
    let admin = User {
        email: String::from("root@here.io"),
        username: String::from("Digitizer100"),
        active: true,
        sign_in_count: 1,
    };

    // Clone admin as user1 with different email
    let user1 = User {
        email: String::from("admin_user_account@here.io"),
        ..admin
    };

}

fn build_user(email: String, username: String) -> User {
    User {
        email, // shorthand for "email: email"
        username, // shorthand for "username: username"
        active: true,
        sign_in_count: 1,
    }
}
