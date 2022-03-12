#[macro_use]
extern crate glium;

use glium::Surface;

mod support;

#[test]
fn magnify_nearest_filtering() {
    // ignoring test on travis
    // TODO: find out why they are failing
    if ::std::env::var("TRAVIS").is_ok() {
        return;
    }

    let display = support::build_display();
    let (vb, ib) = support::build_rectangle_vb_ib(&display);

    let program = program!(&display,
        110 => {
            vertex: "
                #version 110

                attribute vec2 position;

                void main() {
                    gl_Position = vec4(position, 0.0, 1.0);
                }
            ",
            fragment: "
                #version 110

                uniform sampler2D texture;

                void main() {
                    gl_FragColor = texture2D(texture, vec2(0.51, 0.0));
                }
            ",
        },
        100 => {
            vertex: "
                #version 100

                attribute lowp vec2 position;

                void main() {
                    gl_Position = vec4(position, 0.0, 1.0);
                }
            ",
            fragment: "
                #version 100

                uniform lowp sampler2D texture;

                void main() {
                    gl_FragColor = texture2D(texture, vec2(0.51, 0.0));
                }
            ",
        }).unwrap();

    let texture_data = vec![vec![(0u8, 0, 0), (255, 255, 255)]];
    let texture = glium::texture::Texture2d::new(&display, texture_data).unwrap();

    let uniforms = uniform! {
        texture: texture.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
    };

    let output = support::build_renderable_texture(&display);
    output.as_surface().clear_color(0.0, 0.0, 0.0, 0.0);

    match output.as_surface().draw(&vb, &ib, &program, &uniforms, &Default::default()) {
        Ok(_) => (),
        Err(glium::DrawError::SamplersNotSupported) => return,
        Err(e) => panic!("{:?}", e)
    };

    let data: Vec<Vec<(u8, u8, u8, u8)>> = output.read();
    assert_eq!(data[0][0], (255, 255, 255, 255));

    display.assert_no_error(None);
}
