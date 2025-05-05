use qx_rs_err::err::Error;

#[test]
fn test() {

    println!("{}", Error::message("User Not Found"));
    println!("{}", Error::code_message("NOT_FOUND", "User Not Found"));

    println!("{}", Error::message("Something Went Wrong").ext("xxx"));

    let some_error = std::io::Error::new(std::io::ErrorKind::AddrInUse, "xxx");
    println!("{}", Error::error(Box::new(some_error)));

    println!("{}", Error::error_code_info("001", "failed"));
}