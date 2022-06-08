pub mod constants {
    pub const APP_TITLE: &str = "Tide App";
    pub const APP_VERSION: &str = "0.1.0";
    pub const TEMPLATE_DIR: &str = "templates/**/*";
}

pub mod structs {
    use crate::{context, Tera};
    use std::collections::HashMap;

    #[derive(Debug, Clone)]
    pub struct Context {
        pub details: HashMap<&'static str, &'static str>,
        pub tera: Tera,
    }

    impl Context {
        pub fn new(template_dir: &str) -> Self {
            let mut tera = match Tera::new(template_dir) {
                Ok(tera) => tera,
                Err(e) => panic!("Failed to initialize Tera: {}", e),
            };

            Self {
                details: HashMap::new(),
                tera: Tera::new(template_dir).unwrap(),
            }
        }

        pub fn get(&self) -> tera::Context {
            context! {
                "details" => self.details,
            }
        }
    }
}

pub mod functions {}

#[cfg(test)]
mod tests {
    #[test]
    fn test_context_new() {
        use crate::Context;

        let context = Context::new("*");

        assert_eq!(context.details.len(), 0);
        assert_ne!(context.tera.get_template_names().count(), 0);
    }
}
