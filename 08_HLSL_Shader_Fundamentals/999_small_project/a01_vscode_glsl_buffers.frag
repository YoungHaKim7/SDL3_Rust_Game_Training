// Author: Luca Zampetti
// Title: vscode-glsl-canvas Buffers examples

#ifdef GL_ES
precision mediump float;
#endif

uniform sampler2D u_buffer0;
uniform sampler2D u_buffer1;
uniform float u_time;// Animation time
varying vec2 uv;// Texture coordinates
varying vec2 st;// Screen coordinates

// Circle function
float circle(vec2 st,float radius){
    return smoothstep(radius+.01,radius,length(st-vec2(.5)));
}

#if defined(BUFFER_0)

void main(){
    vec3 color=vec3(
        .5+cos(u_time)*.5,
        .5+sin(u_time)*.5,
        1.
    );
    vec3 buffer=texture2D(u_buffer1,uv).rgb;
    buffer*=.99;
    vec2 p=vec2(
        st.x+cos(u_time*5.)*.3,
        st.y+sin(u_time*2.)*.3
    );
    float c=circle(p,.2+.1*sin(u_time));
    buffer=mix(buffer,color,c);
    gl_FragColor=vec4(buffer,1.);
}

#elif defined(BUFFER_1)

void main(){
    vec3 color=vec3(
        .5+cos(u_time)*.5,
        .5+sin(u_time)*.5,
        1.
    );
    vec3 buffer=texture2D(u_buffer0,uv).rgb;
    buffer*=.99;
    vec2 p=vec2(
        st.x+sin(u_time*2.)*.3,
        st.y+cos(u_time*6.)*.3
    );
    float c=circle(p,.2+.1*cos(u_time));
    buffer=mix(buffer,color,c);
    gl_FragColor=vec4(buffer,1.);
}

#else

void main(){
    vec3 color=vec3(0.);// Black color
    vec3 b0=texture2D(u_buffer0,uv).rgb;// Uncomment if needed
    vec3 b1=texture2D(u_buffer1,uv).rgb;
    color+=b0;
    color+=b1;
    gl_FragColor=vec4(color,1.);
}

#endif

