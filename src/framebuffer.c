#include "framebuffer.h"

// The VGA framebuffer starts at 0xB8000.
u16int *video_memory = (u16int *)0xB8000;

// Stores the cursor position.
u8int cursor_x = 0;
u8int cursor_y = 0;

// Store current color state
u8int defaultForeground = VGACOLOR_WHITE;
u8int defaultBackground = VGACOLOR_BLACK;

// Updates the hardware cursor.
static void move_cursor(){
    u16int cursorLocation = (cursor_y * FRAMEBUFFER_COLS) + cursor_x;
    outb(0x3D4, 14);                  // Tell the VGA board we are setting the high cursor byte.
    outb(0x3D5, cursorLocation >> 8); // Send the high cursor byte.
    outb(0x3D4, 15);                  // Tell the VGA board we are setting the low cursor byte.
    outb(0x3D5, cursorLocation);      // Send the low cursor byte.
}

// Returns an integer representing an ASCII character with supplied
// foreground and background colours
static u16int character_with_color( char c, u8int fg, u8int bg ){
    u8int attributeByte = (bg << 4) | (fg & 0x0F);
    return c | (attributeByte << 8);
}

// Scrolls the text on the screen up by one line if we have reached the bottom.
static void scroll(){

    if( cursor_y >= FRAMEBUFFER_ROWS ){

        // Move the current text chunk that makes up the screen
        // back in the buffer by a line
        int i;
        for (i = 0; i < ((FRAMEBUFFER_ROWS-1) * FRAMEBUFFER_COLS); i++){
            video_memory[i] = video_memory[i+FRAMEBUFFER_COLS];
        }

        // The last line should now be blank. Do this by writing
        // 80 spaces to it.
        u16int blank = character_with_color( 0x20, defaultForeground, defaultBackground);
        for (i = ((FRAMEBUFFER_ROWS-1) * FRAMEBUFFER_COLS); i < (FRAMEBUFFER_ROWS * FRAMEBUFFER_COLS); i++){
            video_memory[i] = blank;
        }

        // The cursor should now be on the last line.
        cursor_y = FRAMEBUFFER_ROWS-1;

    }

}

void fb_set_colors( u8int fg, u8int bg ){
    defaultForeground = fg;
    defaultBackground = bg;
}

// Write a single character to the framebuffer with the default colours
void fb_write_char(char c){
    fb_write_char_with_color(c, defaultForeground, defaultBackground);
}

// Writes a single character out to the framebuffer with the specified colours
void fb_write_char_with_color(char c, u8int fg, u8int bg){

    u16int *location;

    if (c == 0x08 && cursor_x){
        // Handle a backspace, by moving the cursor back one space
        cursor_x--;

    } else if (c == 0x09){
        // Handle a tab by increasing the cursor's X, but only to a point
        // where it is divisible by 8.
        cursor_x = (cursor_x+8) & ~(8-1);

    } else if (c == '\r'){
        // Handle carriage return
        cursor_x = 0;

    } else if (c == '\n'){
        // Handle newline by moving cursor back to left and increasing the row
        cursor_x = 0;
        cursor_y++;

    } else if (c >= ' '){
        // Handle any other printable character.
        location = video_memory + (cursor_y*FRAMEBUFFER_COLS + cursor_x);
        *location = character_with_color( c, fg, bg );
        cursor_x++;
    }

    // Check if we need to insert a new line because we have reached the end
    // of the screen.
    if (cursor_x >= FRAMEBUFFER_COLS){
        cursor_x = 0;
        cursor_y ++;
    }

    // Scroll the screen if needed.
    scroll();

    // Move the hardware cursor.
    move_cursor();

}

// Clears the screen, by copying lots of spaces to the framebuffer.
void fb_clear(){

    u16int blank = character_with_color( 0x20, defaultForeground, defaultBackground);

    int i;
    for (i = 0; i < FRAMEBUFFER_COLS*FRAMEBUFFER_ROWS; i++){
        video_memory[i] = blank;
    }

    // Move the hardware cursor back to the start.
    cursor_x = 0;
    cursor_y = 0;
    move_cursor();

}

// Outputs a null-terminated ASCII string to the framebuffer.
void fb_write_str(char *c){
    int i = 0;
    while ( c[i] ){
        fb_write_char(c[i++]);
    }
}

static int int_pow( int a, int b ){
    if( b == 0 ){
        return 1;
    } else if( b == 1 ){
        return a;
    } else {
        return a * int_pow(a, b-1);
    }
}

// Outputs an integer to the framebuffer.
void fb_write_int(int i){
    
    int field_width = (i / 10) + 1;
    int pos = 0;
    char out[20]; // Max 64-bit uInt length

    for( pos = 0; pos < field_width; pos++ ){
        int chr = (i / int_pow( 10, (field_width - pos) - 1 )) % 10;
        out[pos] = chr + 48;
    }

    out[field_width] = '\0';
    fb_write_str(out);

}
