#[macro_use]
extern crate glium;

use glium::Surface;
use glium::index::PrimitiveType;

mod support;

#[test]
#[should_panic(expected = "The program attribute `field1` does not match the vertex format")]
fn attribute_types_mismatch() {
    let display = support::build_display();

    #[derive(Copy, Clone)]
    struct Vertex {
        field1: [f32; 4],
    }

    implement_vertex!(Vertex, field1);

    let vertex_buffer = glium::VertexBuffer::new(&display, &Vec::<Vertex>::new()).unwrap();
    let index_buffer = glium::IndexBuffer::new(&display, PrimitiveType::Points,
                                               &Vec::<u16>::new()).unwrap();

    let program = program!(&display,
        140 => {
            vertex: "
                #version 140

                attribute vec2 field1;

                void main() {
                    gl_Position = vec4(field1, 0.0, 1.0);
                }
            ",
            fragment: "
                #version 140
                out vec4 color;
                void main() {
                    color = vec4(0.0, 0.0, 0.0, 1.0);
                }
            "
        },
        110 => {
            vertex: "
                #version 110

                attribute vec2 field1;

                void main() {
                    gl_Position = vec4(field1, 0.0, 1.0);
                }
            ",
            fragment: "
                #version 110
                void main() {
                    gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
                }
            "
        },
        100 => {
            vertex: "
                #version 100

                attribute mediump vec2 field1;

                void main() {
                    gl_Position = vec4(field1, 0.0, 1.0);
                }
            ",
            fragment: "
                #version 100
                void main() {
                    gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
                }
            "
        }
    ).unwrap();

    // drawing a frame
    let mut target = display.draw();
    target.draw(&vertex_buffer, &index_buffer, &program, &glium::uniforms::EmptyUniforms,
                &Default::default()).unwrap();
    target.finish().unwrap();

    display.assert_no_error(None);
}

#[test]
#[should_panic(expected = "The program attribute `field2` is missing in the vertex bindings")]
fn missing_attribute() {
    let display = support::build_display();

    #[derive(Copy, Clone)]
    struct Vertex {
        field1: [f32; 4],
    }

    implement_vertex!(Vertex, field1);

    let vertex_buffer = glium::VertexBuffer::new(&display, &Vec::<Vertex>::new()).unwrap();
    let index_buffer = glium::IndexBuffer::new(&display, PrimitiveType::Points,
                                               &Vec::<u16>::new()).unwrap();

    let program = program!(&display,
        140 => {
            vertex: "
                #version 140

                attribute vec2 field2;

                void main() {
                    gl_Position = vec4(field2, 0.0, 1.0);
                }
            ",
            fragment: "
                #version 140
                out vec4 color;
                void main() {
                    color = vec4(0.0, 0.0, 0.0, 1.0);
                }
            "
        },
        110 => {
            vertex: "
                #version 110

                attribute vec2 field2;

                void main() {
                    gl_Position = vec4(field2, 0.0, 1.0);
                }
            ",
            fragment: "
                #version 110
                void main() {
                    gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
                }
            "
        },
        100 => {
            vertex: "
                #version 100

                attribute mediump vec2 field2;

                void main() {
                    gl_Position = vec4(field2, 0.0, 1.0);
                }
            ",
            fragment: "
                #version 100
                void main() {
                    gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
                }
            "
        }
    ).unwrap();

    // drawing a frame
    let mut target = display.draw();
    target.draw(&vertex_buffer, &index_buffer, &program, &glium::uniforms::EmptyUniforms,
                &Default::default()).unwrap();
    target.finish().unwrap();

    display.assert_no_error(None);
}

macro_rules! attribute_test(
    ($name:ident, $attr_ty:ty, $glsl_ty:expr, $value:expr, $gl_pos:expr) => (
        #[test]
        fn $name() {
            let display = support::build_display();

            #[derive(Copy, Clone)]
            struct Vertex {
                field1: $attr_ty,
            }

            implement_vertex!(Vertex, field1);

            let vertex_buffer = glium::VertexBuffer::new(&display, &[
                                                             Vertex { field1: $value }
                                                         ]).unwrap();
            let index_buffer = glium::IndexBuffer::new(&display, PrimitiveType::Points,
                                                       &[0u16]).unwrap();

            let program = program!(&display,
                140 => {
                    vertex: &format!("
                        #version 140

                        in {} field1;

                        void main() {{
                            gl_Position = {};
                        }}
                    ", $glsl_ty, $gl_pos),
                    fragment: "
                        #version 140
                        out vec4 color;
                        void main() {
                            color = vec4(0.0, 0.0, 0.0, 1.0);
                        }
                    "
                },
                110 => {
                    vertex: &format!("
                        #version 110

                        attribute {} field1;

                        void main() {{
                            gl_Position = {};
                        }}
                    ", $glsl_ty, $gl_pos),
                    fragment: "
                        #version 110
                        void main() {
                            gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
                        }
                    "
                },
                100 => {
                    vertex: &format!("
                        #version 100

                        attribute lowp {} field1;

                        void main() {{
                            gl_Position = {};
                        }}
                    ", $glsl_ty, $gl_pos),
                    fragment: "
                        #version 100
                        void main() {
                            gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
                        }
                    "
                }
            ).unwrap();

            // drawing a frame
            let mut target = display.draw();
            target.draw(&vertex_buffer, &index_buffer, &program, &glium::uniforms::EmptyUniforms,
                        &Default::default()).unwrap();
            target.finish().unwrap();

            display.assert_no_error(None);
        }

    )
);

attribute_test!(attribute_float_f32, f32, "float", 0.0, "vec4(field1, 0.0, 0.0, 1.0)");
attribute_test!(attribute_vec2_f32, [f32; 2], "vec2", [0.0, 0.0], "vec4(field1, 0.0, 1.0)");
attribute_test!(attribute_vec2_tuple_f32, (f32, f32), "vec2", (0.0, 0.0), "vec4(field1, 0.0, 1.0)");
attribute_test!(attribute_vec3_f32, [f32; 3], "vec3", [0.0, 0.0, 0.0], "vec4(field1, 1.0)");
attribute_test!(attribute_vec3_tuple_f32, (f32, f32, f32), "vec3", (0.0, 0.0, 0.0), "vec4(field1, 1.0)");
attribute_test!(attribute_vec4_f32, [f32; 4], "vec4", [0.0, 0.0, 0.0, 0.0], "field1");
attribute_test!(attribute_vec4_tuple_f32, (f32, f32, f32, f32), "vec4", (0.0, 0.0, 0.0, 0.0), "field1");

attribute_test!(attribute_float_u8, u8, "float", 0, "vec4(field1, 0.0, 0.0, 1.0)");
attribute_test!(attribute_vec2_u8, [u8; 2], "vec2", [0, 0], "vec4(field1, 0.0, 1.0)");
attribute_test!(attribute_vec2_tuple_u8, (u8, u8), "vec2", (0, 0), "vec4(field1, 0.0, 1.0)");
attribute_test!(attribute_vec3_u8, [u8; 3], "vec3", [0, 0, 0], "vec4(field1, 1.0)");
attribute_test!(attribute_vec3_tuple_u8, (u8, u8, u8), "vec3", (0, 0, 0), "vec4(field1, 1.0)");
attribute_test!(attribute_vec4_u8, [u8; 4], "vec4", [0, 0, 0, 0], "field1");
attribute_test!(attribute_vec4_tuple_u8, (u8, u8, u8, u8), "vec4", (0, 0, 0, 0), "field1");

attribute_test!(attribute_float_i8, i8, "float", 0, "vec4(field1, 0.0, 0.0, 1.0)");
attribute_test!(attribute_vec2_i8, [i8; 2], "vec2", [0, 0], "vec4(field1, 0.0, 1.0)");
attribute_test!(attribute_vec2_tuple_i8, (i8, i8), "vec2", (0, 0), "vec4(field1, 0.0, 1.0)");
attribute_test!(attribute_vec3_i8, [i8; 3], "vec3", [0, 0, 0], "vec4(field1, 1.0)");
attribute_test!(attribute_vec3_tuple_i8, (i8, i8, i8), "vec3", (0, 0, 0), "vec4(field1, 1.0)");
attribute_test!(attribute_vec4_i8, [i8; 4], "vec4", [0, 0, 0, 0], "field1");
attribute_test!(attribute_vec4_tuple_i8, (i8, i8, i8, i8), "vec4", (0, 0, 0, 0), "field1");

attribute_test!(attribute_float_u16, u16, "float", 0, "vec4(field1, 0.0, 0.0, 1.0)");
attribute_test!(attribute_vec2_u16, [u16; 2], "vec2", [0, 0], "vec4(field1, 0.0, 1.0)");
attribute_test!(attribute_vec2_tuple_u16, (u16, u16), "vec2", (0, 0), "vec4(field1, 0.0, 1.0)");
attribute_test!(attribute_vec3_u16, [u16; 3], "vec3", [0, 0, 0], "vec4(field1, 1.0)");
attribute_test!(attribute_vec3_tuple_u16, (u16, u16, u16), "vec3", (0, 0, 0), "vec4(field1, 1.0)");
attribute_test!(attribute_vec4_u16, [u16; 4], "vec4", [0, 0, 0, 0], "field1");
attribute_test!(attribute_vec4_tuple_u16, (u16, u16, u16, u16), "vec4", (0, 0, 0, 0), "field1");

attribute_test!(attribute_float_i16, i16, "float", 0, "vec4(field1, 0.0, 0.0, 1.0)");
attribute_test!(attribute_vec2_i16, [i16; 2], "vec2", [0, 0], "vec4(field1, 0.0, 1.0)");
attribute_test!(attribute_vec2_tuple_i16, (i16, i16), "vec2", (0, 0), "vec4(field1, 0.0, 1.0)");
attribute_test!(attribute_vec3_i16, [i16; 3], "vec3", [0, 0, 0], "vec4(field1, 1.0)");
attribute_test!(attribute_vec3_tuple_i16, (i16, i16, i16), "vec3", (0, 0, 0), "vec4(field1, 1.0)");
attribute_test!(attribute_vec4_i16, [i16; 4], "vec4", [0, 0, 0, 0], "field1");
attribute_test!(attribute_vec4_tuple_i16, (i16, i16, i16, i16), "vec4", (0, 0, 0, 0), "field1");

attribute_test!(attribute_float_u32, u32, "float", 0, "vec4(field1, 0.0, 0.0, 1.0)");
attribute_test!(attribute_vec2_u32, [u32; 2], "vec2", [0, 0], "vec4(field1, 0.0, 1.0)");
attribute_test!(attribute_vec2_tuple_u32, (u32, u32), "vec2", (0, 0), "vec4(field1, 0.0, 1.0)");
attribute_test!(attribute_vec3_u32, [u32; 3], "vec3", [0, 0, 0], "vec4(field1, 1.0)");
attribute_test!(attribute_vec3_tuple_u32, (u32, u32, u32), "vec3", (0, 0, 0), "vec4(field1, 1.0)");
attribute_test!(attribute_vec4_u32, [u32; 4], "vec4", [0, 0, 0, 0], "field1");
attribute_test!(attribute_vec4_tuple_u32, (u32, u32, u32, u32), "vec4", (0, 0, 0, 0), "field1");

attribute_test!(attribute_float_i32, i32, "float", 0, "vec4(field1, 0.0, 0.0, 1.0)");
attribute_test!(attribute_vec2_i32, [i32; 2], "vec2", [0, 0], "vec4(field1, 0.0, 1.0)");
attribute_test!(attribute_vec2_tuple_i32, (i32, i32), "vec2", (0, 0), "vec4(field1, 0.0, 1.0)");
attribute_test!(attribute_vec3_i32, [i32; 3], "vec3", [0, 0, 0], "vec4(field1, 1.0)");
attribute_test!(attribute_vec3_tuple_i32, (i32, i32, i32), "vec3", (0, 0, 0), "vec4(field1, 1.0)");
attribute_test!(attribute_vec4_i32, [i32; 4], "vec4", [0, 0, 0, 0], "field1");
attribute_test!(attribute_vec4_tuple_i32, (i32, i32, i32, i32), "vec4", (0, 0, 0, 0), "field1");
