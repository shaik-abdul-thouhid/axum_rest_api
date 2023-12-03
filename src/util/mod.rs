#[macro_export]
macro_rules! debug_info {
    ($message:literal) => {
		if cfg!(debug_assertions) {
            let file_name = file!();
            let module_name = module_path!();
            let line_number = line!();
            let column_number = column!();

			let message = $message;
            println!("module: {module_name:?} {file_name:>15} {line_number}:{column_number} {:>10} message: \"{message}\"", "");
		}
	};
    () => {
		if cfg!(debug_assertions) {
            let file_name = file!();
            let module_name = module_path!();
            let line_number = line!();
            let column_number = column!();

            println!("module: {module_name:?} {file_name:>15} {line_number}:{column_number}");
		}
	};
}
