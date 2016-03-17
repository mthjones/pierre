
 #[macro_export]
 macro_rules! unwrap_result_or_log {
     ($value:expr) => (
        match $value {
            Ok(value) => value,
            Err(e) => {
                error!("{:?}",&e);
                panic!(e)
            }
        };         
     )
 }