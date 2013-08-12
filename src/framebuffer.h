#include "common.h"

#define VGACOLOR_WHITE 15
#define VGACOLOR_BLACK 0

#define FRAMEBUFFER_COLS 80
#define FRAMEBUFFER_ROWS 25

void fb_clear();
void fb_write_str(char *c);
void fb_write_char(char c);
void fb_write_char_with_color(char c, u8int fg, u8int bg);
