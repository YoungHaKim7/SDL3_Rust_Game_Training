// // Author: Luca Zampetti
// // Title: vscode-glsl-canvas Buffers examples
// #ifdef GL_ES
// precision mediump float;
// #endif

// uniform sampler2D u_buffer0;
// uniform sampler2D u_buffer1;
// uniform float u_time; // Animation time
// varying vec2 uv;      // Texture coordinates
// varying vec2 st;      // Screen coordinates

// // Circle function
// float circle(vec2 st, float radius) {
//     return smoothstep(radius + 0.01, radius, length(st - vec2(0.5)));
// }

// #if defined(BUFFER_0)

// void main() {
//     vec3 color = vec3(
//         0.5 + cos(u_time) * 0.5,
//         0.5 + sin(u_time) * 0.5,
//         1.0
//     );
//     vec3 buffer = texture2D(u_buffer1, uv, 0.0).rgb;
//     buffer *= 0.99;
//     vec2 p = vec2(
//         st.x + cos(u_time * 5.0) * 0.3, 
//         st.y + sin(u_time * 2.0) * 0.3
//     );
//     float c = circle(p, 0.2 + 0.1 * sin(u_time));
//     buffer = mix(buffer, color, c * 1.0);
//     gl_FragColor = vec4(buffer, 1.0);
// }

// #elif defined(BUFFER_1)

// void main() {
//     vec3 color = vec3(
//         0.5 + cos(u_time) * 0.5,
//         0.5 + sin(u_time) * 0.5,
//         1.0
//     );
//     vec3 buffer = texture2D(u_buffer0, uv, 0.0).rgb;
//     buffer *= 0.99;
//     vec2 p = vec2(
//         st.x + sin(u_time * 2.0) * 0.3, 
//         st.y + cos(u_time * 6.0) * 0.3
//     );
//     float c = circle(p, 0.2 + 0.1 * cos(u_time));
//     buffer = mix(buffer, color, c * 1.0);
//     gl_FragColor = vec4(buffer, 1.0);
// }

// #else

// void main() {
//     vec3 color = BLACK;
//     // vec3 b0 = texture2D(u_buffer0, uv).rgb;
//     vec3 b1 = texture2D(u_buffer1, uv).rgb;
//     // color += b0;
//     color += b1;
//     gl_FragColor = vec4(color, 1.0);
// }

// #endif

#ifdef GL_ES
precision mediump float;
#endif

uniform sampler2D u_buffer0;
uniform sampler2D u_buffer1;
uniform float u_time; // Animation time
varying vec2 uv;           // Pass texture coordinates to the fragment shader
varying vec2 st;           // Pass normalized screen coordinates to the fragment shader

attribute vec3 a_position;  // Vertex position (input)
attribute vec2 a_texcoord; // Texture coordinates (input)


// Circle function
float circle(vec2 st, float radius) {
    return smoothstep(radius + 0.01, radius, length(st - vec2(0.5)));
}

#if defined(BUFFER_0)

void main() {
    vec3 color = vec3(
        0.5 + cos(u_time) * 0.5,
        0.5 + sin(u_time) * 0.5,
        1.0
    );
    vec3 buffer = texture2D(u_buffer1, uv).rgb;
    buffer *= 0.99;
    vec2 p = vec2(
        st.x + cos(u_time * 5.0) * 0.3, 
        st.y + sin(u_time * 2.0) * 0.3
    );
    float c = circle(p, 0.2 + 0.1 * sin(u_time));
    buffer = mix(buffer, color, c);
    gl_FragColor = vec4(buffer, 1.0);
}

#elif defined(BUFFER_1)

void main() {
    vec3 color = vec3(
        0.5 + cos(u_time) * 0.5,
        0.5 + sin(u_time) * 0.5,
        1.0
    );
    vec3 buffer = texture2D(u_buffer0, uv).rgb;
    buffer *= 0.99;
    vec2 p = vec2(
        st.x + sin(u_time * 2.0) * 0.3, 
        st.y + cos(u_time * 6.0) * 0.3
    );
    float c = circle(p, 0.2 + 0.1 * cos(u_time));
    buffer = mix(buffer, color, c);
    gl_FragColor = vec4(buffer, 1.0);
}

#else

void main() {
    uv = a_texcoord;                  // Pass texture coordinates
    st = a_position.xy * 0.5 + 0.5;   // Convert to normalized device coordinates (0 to 1)
    gl_Position = vec4(a_position, 1.0); // Set the vertex position in clip space

    vec3 color = vec3(0.0); // Black color
    vec3 b1 = texture2D(u_buffer1, uv).rgb;
    color += b1;
    gl_FragColor = vec4(color, 1.0);
}

#endif
    
