struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // Creates an instance of the User struct.
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // To get the value for email we can use just if the instance is mutable.:
    let email = user1.email;

    // If the instance is mutable we can use the code below to change
    // the contents of the email field:
    // FIXME this code will fail because user1 is not mutable.
    //user1.email = String::from("rust_lang@rust.com");

    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user2.email = String::from("rust@rustaceans.com");

    build_user(String::from("rust@rusty.com"), String::from("rustynie"));

    // This is called Struct Update Syntax
    //
    // Example on how to create another instance of Struct User
    // combining the contents of another one.
    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("outrouser232"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    // This can also be declared as:
    // Here, .. syntax specifies that the other fields of user1 should
    // be used to fill the missing variables/fiedlds. Very clever.
    let user4 = User {
        email: String::from("email@memail.com"),
        username: String::from("myusername"),
        ..user1
    };

    // Tuple Structs without Named Fields
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

// Field init shorthand, userd to create and return a User instance.
// Because [email] and [username] fields are the same on the structure
// we can ommit the *declaration and just give the parameters names.
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
