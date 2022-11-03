
use crate::diagram_layout::DiagramLayout;

//wrap library features in a struct
pub struct Session {
   pub  measure_text: Option<fn(&str, f64) -> (f64, f64)>,
}

impl Session {
    pub fn new() -> Session {
        Session {
            measure_text: Option::None,
        }
    }

    //set the measure_text function
    pub fn set_measure_text_fn(&mut self, measure_text: fn(&str, f64) -> (f64, f64)) {
        self.measure_text = Option::Some(measure_text);
    }

    //get singleton instance
    pub fn get_instance() -> &'static mut Session {
        static mut INSTANCE: Option<Session> = None;
        unsafe {
            INSTANCE.get_or_insert_with(Session::new);
            INSTANCE.as_mut().unwrap()
        }
    }

}

//test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session() {
        let mut session = Session::new();
        session.set_measure_text_fn(|text, font_size| {
            (text.len() as f64 * font_size, font_size)
        });
        let (w, h) = session.measure_text.unwrap()("hello", 12.0);
        assert_eq!(w, 60.0);
        assert_eq!(h, 12.0);
    }

    #[test]
    fn test_session_singleton() {
        let mut session = Session::get_instance();
        session.set_measure_text_fn(|text, font_size| {
            (text.len() as f64 * font_size, font_size)
        });
        let (w, h) = session.measure_text.unwrap()("hello", 12.0);
        assert_eq!(w, 60.0);
        assert_eq!(h, 12.0);
    }

    #[test]
    fn test_session_singleton_2() {

        let mut session = Session::get_instance();
        session.set_measure_text_fn(|text, font_size| {
            (text.len() as f64 * font_size, font_size)
        });
        let (w, h) = session.measure_text.unwrap()("hello", 12.0);
        assert_eq!(w, 60.0);
        assert_eq!(h, 12.0);
    }
}