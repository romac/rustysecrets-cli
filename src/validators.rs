
pub mod num {

    pub fn strictly_positive(value: String) -> Result<(), String> {
        let n = value.parse::<u8>();

        if n.is_err() {
            return Err(format!("{} is not a positive number", value));
        }

        if n.unwrap() < 1 {
            return Err(format!("{} is not strictly positive", value));
        }

        Ok(())
    }

}

pub fn mime(value: String) -> Result<(), String> {
    Ok(())
}

pub mod fs {

    use std::path::Path;

    pub fn exists(value: String) -> Result<(), String> {
        let path = Path::new(&value);

        if !path.exists() {
            return Err(format!("'{}' does not exists", value));
        }

        Ok(())
    }

    pub fn not_exists(value: String) -> Result<(), String> {
        let path = Path::new(&value);

        if path.exists() {
            return Err(format!("'{}' already exists", value));
        }

        Ok(())
    }

    pub fn file(value: String) -> Result<(), String> {
        let path = Path::new(&value);

        if !path.exists() {
            return Err(format!("'{}' does not exists", value));
        }

        if !path.is_file() {
            return Err(format!("'{}' is not a file", value));
        }

        Ok(())
    }

    pub fn directory(value: String) -> Result<(), String> {
        let path = Path::new(&value);

        if !path.exists() {
            return Err(format!("'{}' does not exists", value));
        }

        if !path.is_dir() {
            return Err(format!("'{}' is not a directory", value));
        }

        Ok(())
    }

}
