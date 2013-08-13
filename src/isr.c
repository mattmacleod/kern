//
// isr.c -- High level interrupt service routines and interrupt request handlers.
//          Part of this code is modified from Bran's kernel development tutorials.
//          Rewritten for JamesM's kernel development tutorials.
//

#include "../include/common.h"
#include "../include/isr.h"
#include "../include/framebuffer.h"

// This gets called from our ASM interrupt handler stub.
void isr_handler(registers_t regs)
{
    fb_write_str("recieved interrupt: ");
    fb_write_int(regs.int_no);
    fb_write_char('\n');
}
