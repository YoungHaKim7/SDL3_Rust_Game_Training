//------------------------------------------------------------------------------
//--------------------------Code By: 3DSage-------------------------------------
//----------------Video tutorial on YouTube-3DSage------------------------------
//------------------------------------------------------------------------------
/*#ifdef __APPLE__*/
/*#include <GLUT/glut.h>*/
/*#else*/
/*#include <GL/glut.h>*/
/*#endif*/
/**/
#include <OpenGL/gl3.h>
#define __gl_h_
#include <GLUT/glut.h>

int main(int argc, char** argv)
{

    // init GLUT and create window
    glutInit(&argc, argv);
    glutInitDisplayMode(GLUT_DEPTH | GLUT_DOUBLE | GLUT_RGBA);
    glutInitWindowPosition(100, 100);
    glutInitWindowSize(320, 320);
    glutCreateWindow("Lighthouse3D- GLUT Tutorial");

    // register callbacks
    glutDisplayFunc(renderScene);

    // Here is our new entry in the main function
    glutReshapeFunc(changeSize);

    // enter GLUT event processing cycle
    glutMainLoop();

    return 1;
}

