#[macro_export]
macro_rules! error {
    ($message:expr) => {
        println!("[ERROR] {}", $message)
    };
    ($message:expr, $var:expr) => {
        println!("[ERROR] {} \"{}\"", $message, $var)
    };
    ($message:expr; $err:ident) => {
        error!($message);
        println!("{}", $err)
    };
    ($message:expr, $var:expr; $err:ident) => {
        error!($message, $var);
        println!("{}", $err)
    };
}

#[macro_export]
macro_rules! warn {
    ($message:expr) => {
        println!("[WARN] {}", $message)
    };
    ($message:expr, $var:expr) => {
        println!("[WARN] {} \"{}\"", $message, $var)
    };
}

#[macro_export]
macro_rules! success {
	($message:expr) => {
		println!("[SUCCESS] {}", $message)
	};
	($message:expr, $var:expr) => {
		println!("[SUCCESS] {} \"{}\"", $message, $var)
	};
}