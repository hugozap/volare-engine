
use crate::diagram_layout::DiagramLayout;
//use TextOptions
use crate::shape_text::TextOptions;
//wrap library features in a struct
pub struct Session {
   pub  measure_text: Option<fn(&str, &TextOptions) -> (f64, f64)>,
}

impl Session {
    pub fn new() -> Session {
        Session {
            measure_text: Some(|text, text_options| {
                panic!("measure_text not implemented");
            })
        }
    }

    //set the measure_text function
    pub fn set_measure_text_fn(&mut self, measure_text: fn(&str, &TextOptions) -> (f64, f64)) {
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
        session.set_measure_text_fn(|text, textOptions| {
            (text.len() as f64 * textOptions.font_size, textOptions.font_size)
        });
        let (w, h) = session.measure_text.unwrap()("hello", &TextOptions{font_size: 12.0, ..Default::default()});
        assert_eq!(w, 60.0);
        assert_eq!(h, 12.0);
    }

    #[test]
    fn test_session_singleton() {
        let mut session = Session::get_instance();
        session.set_measure_text_fn(|text, text_options| {
            (text.len() as f64 * text_options.font_size, text_options.font_size)
        });
        let (w, h) = session.measure_text.unwrap()("hello", &TextOptions{font_size: 12.0, ..Default::default()});
        assert_eq!(w, 60.0);
        assert_eq!(h, 12.0);
    }

    #[test]
    fn test_session_singleton_2() {

        let mut session = Session::get_instance();
        session.set_measure_text_fn(|text, text_options| {
            (text.len() as f64 * text_options.font_size, text_options.font_size)
        });
        let (w, h) = session.measure_text.unwrap()("hello", &TextOptions{font_size: 12.0, ..Default::default()});
        assert_eq!(w, 60.0);
        assert_eq!(h, 12.0);
    }
}
