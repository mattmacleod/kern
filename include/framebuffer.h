#include "common.h"

#define FRAMEBUFFER_COLS 80
#define FRAMEBUFFER_ROWS 25

#define VGACOLOR_BLACK   0
#define VGACOLOR_BLUE    1
#define VGACOLOR_GREEN   2
#define VGACOLOR_CYAN    3
#define VGACOLOR_RED     4
#define VGACOLOR_MAGENTA 5
#define VGACOLOR_YELLOW  6
#define VGACOLOR_WHITE   7

#define VGACOLOR_LIGHT_BLACK   8
#define VGACOLOR_LIGHT_BLUE    9
#define VGACOLOR_LIGHT_GREEN   10
#define VGACOLOR_LIGHT_CYAN    11
#define VGACOLOR_LIGHT_RED     12
#define VGACOLOR_LIGHT_MAGENTA 13
#define VGACOLOR_LIGHT_YELLOW  14
#define VGACOLOR_LIGHT_WHITE   15

void fb_clear();
void fb_write_str(char *c);
void fb_write_char(char c);
void fb_write_char_with_color(char c, u8int fg, u8int bg);
void fb_set_colors(u8int fg, u8int bg);
