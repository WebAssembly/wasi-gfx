#[cfg(test)]
mod tests {
    #[test]
    fn graphics_context() {
        assert_dir_parses("graphics-context");
    }

    #[test]
    fn webgpu() {
        assert_dir_parses("webgpu");
    }

    #[test]
    fn surface() {
        assert_dir_parses("surface");
    }

    #[test]
    fn frame_buffer() {
        assert_dir_parses("frame-buffer");
    }

    fn assert_dir_parses(path: &str) {
        let mut resolve = wit_parser::Resolve::new();
        resolve
            .push_dir(&format!("../../{path}"))
            .expect(&format!("{path} failed to parse"));
    }
}
